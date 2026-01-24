{
  lib,
  inputs,
  callerLib,
  hosts,
  modules,
  utils,
  extraArgs,
  globalModules,
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
        modules =
          preparedModules
          ++ [
            (_: { nixpkgs.hostPlatform = lib.mkDefault host.platform; })
            host.configuration
          ]
          ++ globalModules;
        specialArgs = extraArgs;
      };
    };

  /**
    Import a single module
  */
  importModule =
    name: value:
    if (value == { }) then
      callerLib.importFilesFromCallerFlake "modules" [ name ]
    else
      callerLib.importFilesFromCallerFlake "modules/${name}" value;

  /**
    Import modules arborescencce
  */
  importModules = lib.lists.flatten (
    lib.attrsets.attrValues (lib.attrsets.mapAttrs importModule modules)
  );

  /**
      Create the final attrset for all configurations
  */
  joinHostsAndModules =
    filter:
    let
      iHosts = callerLib.importFilesFromCallerFlake "hosts" hosts;
      iModules = importModules;
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

  /**
    Build nixos systems
  */
  mkNixosSystems = builtins.listToAttrs (
    builtins.map prepareSystem (joinHostsAndModules utils.getNixosSystems)
  );

in
{
  inherit
    mkDarwinSystems
    mkNixosSystems
    joinHostsAndModules
    prepareSystem
    ;
}
