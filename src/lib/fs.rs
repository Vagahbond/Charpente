use std::{
    fs::{File, create_dir},
    io::Write,
};

pub fn create_modules(dir_name: Option<&str>) -> Result<(), std::io::Error> {
    if let Err(e) = create_dir(dir_name.unwrap_or("modules")) {
        return Err(e);
    }

    let module_str = include_str!("../templates/module.nix");

    let file = File::create(dir_name.unwrap_or("modules/module.nix"));

    if file.is_err() {
        return Err(file.err().unwrap());
    }

    if let Err(e) = file.unwrap().write_all(module_str.as_bytes()) {
        return Err(e);
    }

    Ok(())
}
