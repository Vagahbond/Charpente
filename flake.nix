{
  description = "Carpentry: A non-intrusive nix multi-host framework.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nix-darwin.url = "github:nix-darwin/nix-darwin";
  };

  outputs =
    {
      self,
      nixpkgs,
      nix-darwin,
      ...
    }:
    let

      isDarwin =
        host:
        assert nixpkgs.lib.assertMsg (builtins.hasAttr "platform" host)
          "You need to specify 'platform' in your host's config!";
        (nixpkgs.lib.hasInfix "darwin" host.platform);

      isNixos = host: !(isDarwin host);

      getDarwinSystems = hosts: (builtins.filter isDarwin hosts);

      mkHosts = { config }: { };

      # nixpkgs.lib.map mkHost (getDarwinSystems hosts);

      prepareSystem =
        { host, modules }:
        {
          name =
            assert nixpkgs.lib.assertMsg (builtins.hasAttr "name" host)
              "You need to specify 'platform' in your host's config!";
            host.name;

          value = nix-darwin.lib.darwinSystem {
            inherit modules;
          };
        };

      joinHostsAndModules =
        hosts: modules: filter:
        let
          iHosts = builtins.map import hosts;
        in
        builtins.map (h: {
          inherit modules;
          host = h;
        }) (filter iHosts);

      mkDarwinSystems =
        hosts: modules:
        builtins.listToAttrs (
          builtins.map prepareSystem (joinHostsAndModules hosts modules getDarwinSystems)
        );
    in
    {

      lib = {
        mkSystems =
          {
            hosts ? [ ],
            modules ? [ ],

          }:
          {
            nixosSystems = { };
            darwinSystems = mkDarwinSystems hosts modules;

          };
      };

    };
}
