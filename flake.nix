{
  description = "Carpentry: A non-intrusive nix multi-host framework.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nix-darwin.url = "github:nix-darwin/nix-darwin";
  };

  outputs =
    {
      nixpkgs,
      ...
    }@inputs:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-darwin"
        ] (system: f (import nixpkgs { inherit system; }));
    in
    {
      # TODO: Reduce args usage and try import from root
      lib = {
        inherit
          (import ./lib/default.nix {
            inherit inputs;
            inherit (nixpkgs) lib;
          })
          mkSystems
          ;
      };

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
          ];
        };
      });

      packages = forAllSystems (pkgs: {
        cli = pkgs.rustPlatform.buildRustPackage {
          name = "charpente";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildInputs = with pkgs; [
            cargo
            rustc
          ];
        };
      });
    };
}
