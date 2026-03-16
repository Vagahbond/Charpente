use crate::lib::modules::Modules;

pub const DEFAULT_MODULES_DIR: &str = "modules";
pub const DEFAULT_MODULE_NAME: &str = "someModule";
pub const DEFAULT_HOSTS_DIR: &str = "hosts";
pub const DEFAULT_HOSTNAME: &str = "potatoe";

pub fn get_default_modules() -> Modules {
    vec![
        (
            "module1".to_string(),
            Some(vec!["module1".to_string(), "module2".to_string()]),
        ),
        ("module2".to_string(), None),
    ]
}
