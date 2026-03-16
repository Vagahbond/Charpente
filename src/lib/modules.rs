use std::{
    fs::{File, create_dir_all},
    io::Write,
};

use crate::lib::defaults::{DEFAULT_MODULE_NAME, DEFAULT_MODULES_DIR};

pub type Modules = Vec<(String, Option<Vec<String>>)>;

/*
 * Create a module directory and a module file
 */
pub fn create_module(dir_name: Option<&str>) -> Result<(), std::io::Error> {
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

/*
 * Create a charpenteModules.nix file from a module structure
 */
pub fn create_modules_file(modules: Modules) -> Result<(), std::io::Error> {
    let file = File::create("charpenteModules.nix");

    if file.is_err() {
        return Err(file.err().unwrap());
    }

    let string = generate_modules_list(modules).unwrap();

    if let Err(e) = file.unwrap().write(string.as_bytes()) {
        return Err(e);
    }

    Ok(())
}

/*
 * Generate a list of modules
 */
pub fn generate_modules_list(modules: Modules) -> Result<String, std::io::Error> {
    let _modules = modules;

    let mut list = String::from("{\n");

    for (name, sub_modules) in _modules {
        let sub_modules_str = if sub_modules.is_some() {
            format!(
                "[\n{}\n  ]",
                sub_modules
                    .unwrap()
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join("\n    ")
            )
        } else {
            "{}".to_string()
        };

        list.push_str(format!("  {} = {};\n", name, sub_modules_str).as_str());
    }

    list.push_str("}");

    Ok(list)
}

pub fn scan_modules(dir_name: String) -> Result<Modules, std::io::Error> {
    let mut modules = Modules::new();

    let dir = std::fs::read_dir(dir_name)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            modules.push((
                path.file_name().unwrap().to_str().unwrap().to_string(),
                Some(Vec::new()),
            ));

            for sub_entry in std::fs::read_dir(path).unwrap() {
                let sub_entry = sub_entry?;
                let sub_path = sub_entry.path();

                if sub_path.is_dir() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Sub-directories are not supported",
                    ));
                } else if sub_path.is_file() {
                    let file_name = sub_path.file_name().unwrap().to_str().unwrap();

                    if file_name.ends_with(".nix") {
                        let name = file_name.split(".").collect::<Vec<&str>>()[0];
                        modules
                            .last_mut()
                            .unwrap()
                            .1
                            .as_mut()
                            .unwrap()
                            .push(name.to_string());
                    }
                }
            }
        } else if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(".nix") {
                let name = file_name.split(".").collect::<Vec<&str>>()[0];
                modules.push((name.to_string(), None));
            }
        }
    }

    Ok(modules)
}
