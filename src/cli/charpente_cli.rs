use std::marker::PhantomData;

use bubbletea_rs::{Cmd, KeyMsg, Model, Msg, command, quit};
use bubbletea_widgets::textinput;
use crossterm::event::{KeyCode, KeyModifiers};

pub struct CharpenteCliStep {
    pub input: Option<textinput::Model>,
    pub view: fn(steps: &Vec<CharpenteCliStep>) -> String,
    pub update: fn(steps: &Vec<CharpenteCliStep>, msg: Msg) -> Option<Cmd>,
}

pub type CliInterface = Vec<CharpenteCliStep>;

pub trait CharpenteInterface {
    fn get_steps() -> Vec<CharpenteCliStep>;
}

pub struct CharpenteModel<T: CharpenteInterface + Send> {
    _interface: PhantomData<T>,
    steps: Vec<CharpenteCliStep>,
    current_step: usize,
}

pub fn prepare_input(label: &str, placeholder: &str) -> textinput::Model {
    let mut input = textinput::new();
    input.prompt = label.to_string();
    input.placeholder = placeholder.to_string();
    input
}

impl<T: CharpenteInterface + Send + 'static> Model for CharpenteModel<T> {
    fn init() -> (Self, Option<Cmd>) {
        (
            Self {
                _interface: PhantomData,
                steps: T::get_steps(),
                current_step: 0,
            },
            // This is to ensure that the model is initialized before the view is rendered
            Some(command::printf("".to_string())),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if self.current_step >= self.steps.len() {
            return Some(quit());
        }

        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }

                KeyCode::Enter => {
                    let step = self.current_step.clone();
                    self.current_step += 1;

                    (self.steps.get_mut(step).unwrap().update)(&self.steps, msg);

                    if step > 0
                        && self.steps[step].input.is_some()
                        && self.steps[step].input.as_ref().unwrap().focused()
                    {
                        self.steps[step].input.as_mut().unwrap().blur();
                    }

                    if step + 1 < self.steps.len() - 1 && self.steps[step + 1].input.is_some() {
                        return Some(self.steps[step + 1].input.as_mut().unwrap().focus());
                    }
                }

                _ => {
                    if self.current_step < self.steps.len() - 1
                        && self.steps[self.current_step].input.is_some()
                        && self.steps[self.current_step]
                            .input
                            .as_ref()
                            .unwrap()
                            .focused()
                    {
                        return self.steps[self.current_step]
                            .input
                            .as_mut()
                            .unwrap()
                            .update(msg);
                    }
                }
            }
        }
        None
    }

    fn view(&self) -> String {
        let step = self.current_step;

        if step >= self.steps.len() {
            return "Good bye!".to_string();
        }

        let mut res = (self.steps[step].view)(&self.steps);
        if step < self.steps.len() - 1 && self.steps[step].input.is_some() {
            res.push_str(self.steps[step].input.as_ref().unwrap().view().as_str());

            res.push_str("\n");
        }

        res.push_str("\nPress 'ctrl+c' to quit.\n");

        res
    }
}
