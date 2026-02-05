use std::collections::HashMap;

use bubbletea_rs::{Cmd, KeyMsg, Model, Msg, command, quit, tick};
use bubbletea_widgets::{TextInput, textinput};
use crossterm::event::{KeyCode, KeyModifiers};

use crate::cli::{
    charpente_cli::{CharpenteCliStep, CharpenteInterface, CharpenteModel},
    const_str,
};

pub struct InitStep {}

impl CharpenteInterface for InitStep {
    fn get_steps() -> Vec<CharpenteCliStep> {
        vec![
            CharpenteCliStep {
                input: None,
                view: || "Welcome to charpente-cli!\n".to_string(),
                update: || None,
            },
            CharpenteCliStep {
                input: Some(textinput::new()),
                view: || "Welcome to charpente-cli!\n".to_string(),
                update: || None,
            },
        ]
    }
}
/*
impl Model for Carousel {
    fn init() -> (Self, Option<Cmd>) {
        let mut modules_dir_name = textinput::new();
        let mut hosts_dir_name = textinput::new();

        modules_dir_name.prompt = "Use a custom modules directory? ".to_string();
        hosts_dir_name.prompt = "Use a custom hosts directory? ".to_string();

        modules_dir_name.placeholder = "default: './modules'".to_string();
        hosts_dir_name.placeholder = "default: './hosts'".to_string();

        (
            Self {
                current_step: InitStep::Intro,
                modules_dir_name,
                hosts_dir_name,
            },
            Some(command::printf("".to_string())),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {

                KeyCode::Enter => match self.current_step {
                    InitStep::Intro => {
                        self.current_step = InitStep::ModulesDir;
                    }

                    InitStep::ModulesDir => {
                        self.current_step = InitStep::HostsDir;
                        self.modules_dir_name.blur();
                        return Some(self.hosts_dir_name.focus());
                    }

                    InitStep::HostsDir => {
                        self.current_step = InitStep::Hostname;
                        self.hosts_dir_name.blur();
                        return None;
                    }

                    InitStep::Hostname => {
                        self.current_step = InitStep::ScaffoldingHosts;
                        return None;
                    }

                    InitStep::ScaffoldingModules => {
                        self.current_step = InitStep::ScaffoldingHosts;
                        return None;
                    }

                    InitStep::ScaffoldingHosts => {
                        self.current_step = InitStep::FlakeInput;
                        return None;
                    }

                    InitStep::FlakeInput => {
                        self.current_step = InitStep::FlakeOutput;
                        return None;
                    }

                    InitStep::FlakeOutput => {
                        self.current_step = InitStep::Done;
                        return None;
                    }

                    InitStep::Done => {
                        return Some(quit());
                    }
                },

                _ => {
                    if self.modules_dir_name.focused() {
                        return self.modules_dir_name.update(msg);
                    }
                    if self.hosts_dir_name.focused() {
                        return self.hosts_dir_name.update(msg);
                    }
                }
            }
        }
        None
    }

    fn view(&self) -> String {
        let mut res = String::from("Welcome to charpente-cli!\n");

        res.push_str("Let's set Charpente up in your nix configuration.\n\n");

        res.push_str("Press 'q' or 'ctrl+c' to quit.\n");
        res.push_str("Press 'enter' to continue.\n");

        if self.current_step == InitStep::ModulesDir {
            res.push_str(self.modules_dir_name.view().as_str());
            res.push_str("\n");
        }

        if self.current_step == InitStep::HostsDir {
            res.push_str(self.hosts_dir_name.view().as_str());
            res.push_str("\n");
        }

        if self.current_step == InitStep::ScaffoldingModules {
            res.push_str(
                format!(
                    "Creating {} with a paceholder module...\n",
                    self.modules_dir_name.value(),
                )
                .as_str(),
            );
        }

        if self.current_step == InitStep::ScaffoldingHosts {
            res.push_str(
                format!(
                    "Creating {} with a paceholder host...\n",
                    self.hosts_dir_name.value(),
                )
                .as_str(),
            );
        }

        if self.current_step == InitStep::FlakeInput {
            let modules_override = if self.modules_dir_name.value().is_empty() {
                None
            } else {
                Some(self.modules_dir_name.value().clone())
            };

            let hosts_override = if self.hosts_dir_name.value().is_empty() {
                None
            } else {
                Some(self.hosts_dir_name.value().clone())
            };

            res.push_str(
                format!(
                    "Add the following content to your flake inputs!:\n\n{}\n\n",
                    const_str::init::flake_output(modules_override, hosts_override, hostname)
                )
                .as_str(),
            );
        }

        return res;
    }
}
 */
