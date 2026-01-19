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

    };
}
