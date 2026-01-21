{
  lib,
  inputs,
  callerLib,
  hosts,
  modules,
  utils,
  extraArgs,
}:
let

  /**
    Pick the right elements from a module
  */
  prepareModuleArray =
    host: module:
    let
      hasDarwinModule = (builtins.hasAttr "darwinConfiguration" module) && utils.isDarwin host;
      hasNixosModule = (builtins.hasAttr "nixosConfiguration" module) && utils.isNixos host;

    in
    if (lib.lists.any (t: t == host.name) module.targets) then
      [
        (if (builtins.hasAttr "sharedConfiguration" module) then module.sharedConfiguration else _: { })

        (if hasDarwinModule then module.darwinConfiguration else _: { })

        (if hasNixosModule then module.nixosConfiguration else _: { })
      ]
    else
      [ ];

  /**
    Gather hosts and modules into a single system's configuration
  */
  prepareModules = host: modules: lib.lists.flatten (builtins.map (prepareModuleArray host) modules);

  /**
    Gather hosts and loaded modules into a single system's configuration
  */
  prepareSystem =
    { host, modules }:
    let
      preparedModules = prepareModules host modules;

    in
    {
      name =
        assert lib.assertMsg (builtins.hasAttr "name" host)
          "You need to specify a name (corresponds to hostname) in your host's config!";
        host.name;

      value = inputs.nix-darwin.lib.darwinSystem {
        modules = preparedModules ++ [
          (_: { nixpkgs.hostPlatform = lib.mkDefault host.platform; })
          host.configuration
        ];
        specialArgs = extraArgs;
      };
    };

  /**
      Create the final attrset for all configurations
  */
  joinHostsAndModules =
    filter:
    let
      iHosts = callerLib.importFilesFromCallerFlake "hosts" hosts;
      iModules = callerLib.importFilesFromCallerFlake "modules" modules;
    in
    builtins.map (h: {
      modules = iModules;
      host = h;
    }) (filter iHosts);

  /**
    Build darwin systems
  */
  mkDarwinSystems = builtins.listToAttrs (
    builtins.map prepareSystem (joinHostsAndModules utils.getDarwinSystems)
  );
in
{
  inherit mkDarwinSystems joinHostsAndModules prepareSystem;
}
