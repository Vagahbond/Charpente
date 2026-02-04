use bubbletea_rs::{Cmd, KeyMsg, Model, Msg, quit};
use bubbletea_widgets::textinput;
use crossterm::event::{KeyCode, KeyModifiers};

enum InitStep {
    Intro,
    ModulesDir,
    HostsDir,
    Scaffolding,
    FlakeSection,
    Done,
}

pub struct InitModel {
    modules_dir_name: textinput::Model,
    hosts_dir_name: textinput::Model,
    current_step: InitStep,
}

impl Model for InitModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut modules_dir_name = textinput::new();
        let mut hosts_dir_name = textinput::new();

        modules_dir_name.prompt = "Use a custom modules directory? ".to_string();
        hosts_dir_name.prompt = "Use a custom hosts directory? ".to_string();

        modules_dir_name.placeholder = "default: 'modules'".to_string();
        hosts_dir_name.placeholder = "default: 'hosts'".to_string();

        (
            Self {
                current_step: InitStep::Intro,
                modules_dir_name,
                hosts_dir_name,
            },
            None,
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }

                KeyCode::Char('q') => {
                    return Some(quit());
                }

                KeyCode::Enter => match self.current_step {
                    InitStep::Intro => {
                        self.current_step = InitStep::ModulesDir;
                        return None;
                    }

                    InitStep::ModulesDir => {
                        self.current_step = InitStep::HostsDir;
                        return Some(self.modules_dir_name.focus());
                    }

                    InitStep::HostsDir => {
                        self.current_step = InitStep::Scaffolding;
                        self.modules_dir_name.blur();
                        return Some(self.hosts_dir_name.focus());
                    }

                    InitStep::Scaffolding => {
                        self.current_step = InitStep::FlakeSection;
                        self.hosts_dir_name.blur();
                        return None;
                    }

                    InitStep::FlakeSection => {
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

        if self.modules_dir_name.focused() {
            res.push_str(self.modules_dir_name.view().as_str());
            res.push_str("\n");
        }

        if self.hosts_dir_name.focused() {
            res.push_str(self.hosts_dir_name.view().as_str());
            res.push_str("\n");
        }

        return res;
    }
}
