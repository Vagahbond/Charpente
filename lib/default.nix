{ lib, inputs }:
{
  mkSystems =
    {
      root,
      hosts ? [ ],
      modules ? [ ],

    }:
    let
      utils = import ./utils.nix {
        inherit lib;
      };

      callerLib = import ./callerFlake.nix {
        inherit root lib;
      };

      confBuilder = import ./confBuilder.nix {
        inherit
          utils
          callerLib
          hosts
          modules
          lib
          inputs
          ;
      };
    in
    {
      nixosSystems = { };
      darwinSystems = confBuilder.mkDarwinSystems;
    };
}
