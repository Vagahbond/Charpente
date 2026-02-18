use crate::cli::{
    charpente_cli::{CharpenteCliStep, CharpenteInterface, prepare_input},
    const_str,
};

use crate::lib::fs::create_modules;

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
                    let modules_dir_name = if steps[MODULES_DIR_INDEX]
                        .input
                        .as_ref()
                        .unwrap()
                        .value()
                        .is_empty()
                    {
                        None
                    } else {
                        Some(
                            steps[MODULES_DIR_INDEX]
                                .input
                                .as_ref()
                                .unwrap()
                                .value()
                                .clone(),
                        )
                    };

                    if let Err(e) = create_modules(modules_dir_name.as_deref()) {
                        panic!("Failed to create modules directory: {}", e);
                    }
                    None
                },
            },
            CharpenteCliStep {
                input: None,
                view: |_| "Scaffolding hosts directory...\n".to_string(),
                update: |_, _| None,
            },
            CharpenteCliStep {
                input: None,
                view: |_| {
                    format!(
                        "Add this to your flake inputs!\n\n{}\n\n",
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
                        "Adding the following content to your flake outputs!\n\n{}\n\n",
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
