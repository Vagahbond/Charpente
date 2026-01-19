{
  lib,
  inputs,
  callerLib,
  hosts,
  modules,
  utils,
}:
let

  /**
    Pick the right elements from a module
  */
  prepareModule =
    host: module:
    if (builtins.has module.targets host.name) then
      (
        module.sharedConfiguration
        // (if utils.isDarwin host then module.darwinConfiguration else module.nixosConfiguration)
      )
    else
      { };

  /**
    Gather hosts and modules into a single system's configuration
  */
  prepareModules = host: modules: builtins.map (prepareModule host) modules;

  /**
    Gather hosts and loaded modules into a single system's configuration
  */
  prepareSystem =
    { host, modules }:
    {
      name =
        assert lib.assertMsg (builtins.hasAttr "name" host)
          "You need to specify a name (corresponds to hostname) in your host's config!";
        host.name;

      value = inputs.nix-darwin.lib.darwinSystem {
        modules = prepareModules host modules;
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
