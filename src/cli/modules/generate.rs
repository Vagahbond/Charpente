use std::{path::Path, pin::Pin};

use bubbletea_rs::{Cmd, Msg, println, quit};
use bubbletea_widgets::{TextInput, textinput};

use crate::{
    cli::charpente_cli::{CharpenteCliMsg, CharpenteCliPage},
    lib::{
        defaults::DEFAULT_MODULES_DIR,
        modules::{Modules, create_modules_file, generate_modules_list, scan_modules},
    },
};

pub struct GenModulesPage {
    modules_dir_name_input: TextInput,
    scanned_modules: Modules,
    modules_written: bool,
}

impl CharpenteCliPage for GenModulesPage {
    fn init() -> (Self, Option<Cmd>) {
        (
            Self {
                modules_dir_name_input: textinput::new(),
                scanned_modules: Modules::new(),
                modules_written: false,
            },
            None,
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if self.modules_dir_name_input.value().is_empty() {
            if !Path::new(DEFAULT_MODULES_DIR).exists() {
                return Some(self.modules_dir_name_input.focus());
            } else {
                self.modules_dir_name_input.set_value(DEFAULT_MODULES_DIR);
                return Some(CharpenteCliMsg::update());
            }
        }

        if self.scanned_modules.is_empty() {
            if let Ok(modules) = scan_modules(self.modules_dir_name_input.value()) {
                self.scanned_modules = modules;
                return Some(CharpenteCliMsg::update());
            } else {
                return Some(println("Something went wrong !".to_string()));
            }
        }

        if !self.modules_written {
            if let Err(e) = create_modules_file(self.scanned_modules.clone()) {
                return Some(println(format!("Something went wrong : {}", e)));
            } else {
                self.modules_written = true;
                return Some(CharpenteCliMsg::update());
            }
        }

        return Some(quit());
    }

    fn view(&self) -> String {
        let mut res = String::from("Let's generate your charpenteModules.nix !\n");

        if self.modules_dir_name_input.focused() {
            res.push_str(self.modules_dir_name_input.view().as_str());
            res.push_str("\n");
        }

        if !self.modules_dir_name_input.value().is_empty() {
            res.push_str("Scanning modules...\n");
        }

        if !self.scanned_modules.is_empty() {
            res.push_str("Your modules have been scanned.\n");
            res.push_str(
                format!(
                    "Scanned {} modules and {} submodules in total.",
                    self.scanned_modules.len(),
                    self.scanned_modules
                        .iter()
                        .map(|(_, sub)| sub.as_ref().unwrap_or(&Vec::new()).len())
                        .sum::<usize>()
                )
                .as_str(),
            );
            res.push_str("\n");
            res.push_str("Writing charpenteModules.nix...\n");
        }

        if self.modules_written {
            res.push_str("charpenteModules.nix has been written.\nExiting...\n");
        }

        return res;
    }
}
