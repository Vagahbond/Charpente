{

  targets = [ "potatoe" ];

  sharedConfiguration =
    {
      pkgs,
      ...
    }:
    {
      environment = {
        systemPackages = with pkgs; [
          cbonsai
        ];
      };
    };

  nixosConfiguration =
    { pkgs, ... }:
    {
      environment.systemPackages = with pkgs; [
        asciiquarium
      ];
    };

  darwinConfiguration =
    { pkgs, ... }:
    {
      environment.systemPackages = with pkgs; [
        cmatrix
      ];
    };

}
