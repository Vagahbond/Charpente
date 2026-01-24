{ lib, inputs }:
{
  mkSystems =
    {
      root,
      hosts ? [ ],
      modules ? [ ],
      extraArgs ? [ ],
      globalModules ? [ ],

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
          extraArgs
          globalModules
          ;
      };
    in
    {
      nixosSystems = confBuilder.mkNixosSystems;
      darwinSystems = confBuilder.mkDarwinSystems;
    };
}
