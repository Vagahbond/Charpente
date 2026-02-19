pub mod init {
    pub const FLAKE_INPUT: &str = r#"
  charpente = {
    url = "github:vagahbond/charpente";
    inputs.nixpkgs.follows = "nixpkgs";
    inputs.nix-darwin.follows = "nix-darwin";
  };
"#;

    pub fn flake_output(
        modules_override: Option<String>,
        host_override: Option<String>,
        hostname: String,
    ) -> String {
        format!(
            r#"
  outputs =
    {{
      self,
      charpente,
      ...
    }}@inputs:
    let
      systems = charpente.lib.mkSystems {{
        root = self;

        hosts = [
          {} 
        ];

        modules = import ./charpenteModules.nix;

        {}
        {}
      }};
    in
  {{
    nixosConfigurations = systems.nixosSystems;
    darwinConfigurations = systems.darwinSystems;
  }};
"#,
            hostname,
            if modules_override.is_some() {
                format!("modulesDir = \"{}\";", modules_override.unwrap())
            } else {
                "".to_string()
            },
            if host_override.is_some() {
                format!("hostDir = \"{}\";", host_override.unwrap())
            } else {
                "".to_string()
            }
        )
    }
}
