fn get_platform() -> String {
    if std::env::consts::OS == "linux" && std::env::consts::ARCH == "x86_64" {
        "x86_64-linux".to_string()
    } else if std::env::consts::OS == "linux" && std::env::consts::ARCH == "aarch64" {
        "aarch64-linux".to_string()
    } else if std::env::consts::OS == "macos" && std::env::consts::ARCH == "x86_64" {
        "x86_64-darwin".to_string()
    } else if std::env::consts::OS == "macos" && std::env::consts::ARCH == "aarch64" {
        "aarch64-darwin".to_string()
    } else {
        "COULD NOT DETERMINE PLATFORM".to_string()
    }
}

pub fn host_str(hostname: &str) -> String {
    format!(
        r#"
{{
  name = "{}";
  platform = "{}";

  configuration = _ : {{}};
}}
"#,
        hostname,
        get_platform()
    )
}
