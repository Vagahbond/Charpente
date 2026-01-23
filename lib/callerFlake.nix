{
  lib,
  root,
}:
let

  /**
    Gather hosts and modules into a system's configuration
  */
  getFileOrDir =
    fullPath:
    if (builtins.pathExists "${fullPath}/default.nix") then
      fullPath
    else
      (if (builtins.pathExists "${fullPath}.nix") then "${fullPath}.nix" else null);

  /**
    Import all modules from a configuration
  */
  importFilesFromCallerFlake =
    path: names:
    builtins.map (
      h:
      let
        fullPath = "${root}/${path}/${h}";
        file = getFileOrDir fullPath;
      in
      assert lib.assertMsg (
        file != null
      ) "You need to create either `${path}/${h}.nix` or `${path}/${h}/default.nix` !";
      import file
    ) names;

in
{
  inherit
    importFilesFromCallerFlake
    getFileOrDir
    ;

}
