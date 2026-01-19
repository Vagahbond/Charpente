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
    Gather hosts and modules into a single system's configuration
  */
  prepareModules = {

  };

  /**
    Gather hosts and modules into a single system's configuration
  */
  prepareSystem =
    { host }:
    {
      name =
        assert lib.assertMsg (builtins.hasAttr "name" host)
          "You need to specify a name (corresponds to hostname) in your host's config!";
        host.name;

      value = inputs.nix-darwin.lib.darwinSystem {
        modules = prepareModules;
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
