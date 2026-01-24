{ lib }:
let

  /**
    Predicate to find whether a host is of the Darwin platform
  */
  isDarwin =
    host:
    assert lib.assertMsg (builtins.hasAttr "platform" host)
      "You need to specify 'platform' in your host's config!";
    (lib.hasInfix "darwin" host.platform);

  /**
    Predicate to find whether a host is of the Nixos platform
  */
  isNixos = host: !(isDarwin host);

  /**
    In a list of imported hosts, filter out all non-darwin systems
  */
  getDarwinSystems = hosts: (builtins.filter isDarwin hosts);

  /**
    In a list of imported hosts, filter out all non-nixos systems
  */
  getNixosSystems = hosts: (builtins.filter isNixos hosts);
in
{
  inherit
    isDarwin
    isNixos
    getDarwinSystems
    getNixosSystems
    ;
}
