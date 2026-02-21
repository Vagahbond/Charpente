use std::{
    fs::{File, create_dir_all},
    io::Write,
};

use crate::lib::defaults::{DEFAULT_MODULE_NAME, DEFAULT_MODULES_DIR};

pub type Modules = Option<Vec<(String, Option<Vec<String>>)>>;

pub fn create_modules(dir_name: Option<&str>) -> Result<(), std::io::Error> {
    if let Err(e) = create_dir_all(dir_name.unwrap_or(DEFAULT_MODULES_DIR)) {
        return Err(e);
    }

    let module_str = include_str!("../templates/module.nix");

    let file = File::create(format!(
        "{}/{}.nix",
        dir_name.unwrap_or(DEFAULT_MODULES_DIR),
        DEFAULT_MODULE_NAME
    ));

    if file.is_err() {
        return Err(file.err().unwrap());
    }

    if let Err(e) = file.unwrap().write_all(module_str.as_bytes()) {
        return Err(e);
    }

    Ok(())
}

pub fn create_charpente_modules(modules: Modules) -> Result<(), std::io::Error> {
    let file = File::create("charpenteModules.nix");

    if file.is_err() {
        return Err(file.err().unwrap());
    }

    let _modules = modules.unwrap_or(vec![(DEFAULT_MODULE_NAME.to_string(), None)]);

    let mut list = String::from("{\n");

    for (name, sub_modules) in _modules {
        let sub_modules_str = if sub_modules.is_some() {
            format!("[\n{}\n  ]", sub_modules.unwrap().join(" "))
        } else {
            "{}".to_string()
        };

        list.push_str(format!("  {} = {};\n", name, sub_modules_str).as_str());
    }

    list.push_str("};");

    if let Err(e) = file.unwrap().write(list.as_bytes()) {
        return Err(e);
    }

    Ok(())
}
