use std::{
    fs::{File, create_dir_all},
    io::Write,
};

use crate::{
    lib::defaults::{DEFAULT_HOSTNAME, DEFAULT_HOSTS_DIR},
    templates::host::host_str,
};

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
