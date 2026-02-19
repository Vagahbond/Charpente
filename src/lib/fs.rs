use std::{
    fs::{File, create_dir_all},
    io::Write,
};

use crate::{
    lib::defaults::{
        DEFAULT_HOSTNAME, DEFAULT_HOSTS_DIR, DEFAULT_MODULE_NAME, DEFAULT_MODULES_DIR,
    },
    templates::host::host_str,
};

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

pub fn create_hosts(dir_name: Option<&str>, hostname: Option<&str>) -> Result<(), std::io::Error> {
    if let Err(e) = create_dir_all(format!(
        "{}/{}",
        dir_name.unwrap_or(DEFAULT_HOSTS_DIR),
        hostname.unwrap_or(DEFAULT_HOSTNAME),
    )) {
        return Err(e);
    }

    let file = File::create(format!(
        "{}/{}/default.nix",
        dir_name.unwrap_or(DEFAULT_HOSTS_DIR),
        hostname.unwrap_or(DEFAULT_HOSTNAME)
    ));

    if file.is_err() {
        return Err(file.err().unwrap());
    }

    if let Err(e) = file
        .unwrap()
        .write_all(host_str(hostname.unwrap_or(DEFAULT_HOSTNAME)).as_bytes())
    {
        return Err(e);
    }

    Ok(())
}

pub fn create_charpente_modules(modules: Option<Vec<String>>) -> Result<(), std::io::Error> {
    let file = File::create("charpenteModules.nix");

    if file.is_err() {
        return Err(file.err().unwrap());
    }

    let modules_list = modules
        .unwrap_or(vec![DEFAULT_MODULE_NAME.to_string()])
        .join("\n  ");

    if let Err(e) = file
        .unwrap()
        .write(format!("[\n  {}\n]", modules_list).as_bytes())
    {
        return Err(e);
    }

    Ok(())
}
