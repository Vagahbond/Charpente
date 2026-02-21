use crate::{
    cli::{
        charpente_cli::{CharpenteCliStep, CharpenteInterface, prepare_input},
        const_str,
    },
    lib::{
        hosts::create_hosts,
        modules::{create_charpente_modules, create_modules},
    },
};

pub struct InitStep {}

const MODULES_DIR_INDEX: usize = 2;
const HOSTS_DIR_INDEX: usize = 3;
const HOSTNAME_INDEX: usize = 4;

impl CharpenteInterface for InitStep {
    fn get_steps() -> Vec<CharpenteCliStep> {
        vec![
            CharpenteCliStep {
                input: None,
                view: |_| "Welcome to charpente-cli!\n\nPress 'enter' to continue.\n".to_string(),
                update: |_, _| None,
            },
            CharpenteCliStep {
                input: None,
                view: |_| "Let's set Charpente up in your nix configuration.\n\n".to_string(),
                update: |_, _| None,
            },
            CharpenteCliStep {
                input: Some(prepare_input("Modules directory: ", "default: './modules'")),
                view: |_| "Use a custom modules directory?\n".to_string(),
                update: |_, _| None,
            },
            CharpenteCliStep {
                input: Some(prepare_input("Hosts directory: ", "default: './hosts'")),
                view: |_| "Use a custom hosts directory?\n".to_string(),
                update: |_, _| None,
            },
            CharpenteCliStep {
                input: Some(prepare_input("Hostname: ", "default: 'potatoe'")),
                view: |_| "What hostname do you want to use?\n".to_string(),
                update: |_, _| None,
            },
            CharpenteCliStep {
                input: None,
                view: |_| "Scaffolding modules directory...\n".to_string(),
                update: |steps, _| {
                    let modules_dir_name = steps[MODULES_DIR_INDEX].input.as_ref();

                    if modules_dir_name.is_none()
                        || modules_dir_name.as_ref().unwrap().value().is_empty()
                    {
                        if let Err(e) = create_modules(None) {
                            panic!("Failed to create modules directory: {}", e);
                        }
                    } else {
                        if let Err(e) =
                            create_modules(Some(modules_dir_name.unwrap().value().as_str()))
                        {
                            panic!("Failed to create modules directory: {}", e);
                        }
                    }

                    None
                },
            },
            CharpenteCliStep {
                input: None,
                view: |_| "Scaffolding hosts directory...\n".to_string(),
                update: |steps, _| {
                    let hosts_dir_name = steps[HOSTS_DIR_INDEX].input.as_ref().unwrap().value();
                    let hostname = steps[HOSTNAME_INDEX].input.as_ref().unwrap().value();

                    let _host_dir_name = if hosts_dir_name.is_empty() {
                        None
                    } else {
                        Some(hosts_dir_name.as_str())
                    };

                    let _hostname = if hostname.is_empty() {
                        None
                    } else {
                        Some(hostname.as_str())
                    };

                    if let Err(e) = create_hosts(_host_dir_name, _hostname) {
                        panic!("Failed to create hosts directory: {}", e);
                    }

                    None
                },
            },
            CharpenteCliStep {
                input: None,
                view: |_| "Creating charpenteModules.nix...\n".to_string(),
                update: |_, _| {
                    if let Err(e) = create_charpente_modules(None) {
                        panic!("Failed to create charpenteModules.nix: {}", e);
                    }

                    None
                },
            },
            CharpenteCliStep {
                input: None,
                view: |_| {
                    format!(
                        "Add this to your flake inputs!\nOnly keep follow statements that are relevant to your config.\n\n{}\n\n",
                        const_str::init::FLAKE_INPUT
                    )
                },
                update: |_, _| None,
            },
            CharpenteCliStep {
                input: None,
                view: |s| {
                    let modules_override = if s[MODULES_DIR_INDEX]
                        .input
                        .as_ref()
                        .unwrap()
                        .value()
                        .is_empty()
                    {
                        None
                    } else {
                        Some(s[MODULES_DIR_INDEX].input.as_ref().unwrap().value().clone())
                    };

                    let hosts_override = if s[HOSTS_DIR_INDEX]
                        .input
                        .as_ref()
                        .unwrap()
                        .value()
                        .is_empty()
                    {
                        None
                    } else {
                        Some(s[HOSTS_DIR_INDEX].input.as_ref().unwrap().value().clone())
                    };

                    format!(
                        "Adding the following content to your flake outputs!\nRemove the configurations you do not need!\n\n{}\n\n",
                        const_str::init::flake_output(
                            modules_override,
                            hosts_override,
                            s[HOSTNAME_INDEX].input.as_ref().unwrap().value()
                        )
                    )
                },
                update: |_, _| None,
            },
        ]
    }
}
