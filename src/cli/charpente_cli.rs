use std::marker::PhantomData;

use bubbletea_rs::{Cmd, KeyMsg, Model, Msg, command, quit};
use bubbletea_widgets::textinput;
use crossterm::event::{KeyCode, KeyModifiers};

pub struct CharpenteCliStep {
    pub input: Option<textinput::Model>,
    pub view: fn() -> String,
    pub update: fn(steps: &Vec<CharpenteCliStep>) -> Option<Cmd>,
}

pub type CliInterface = Vec<CharpenteCliStep>;

pub trait CharpenteInterface {
    fn get_steps() -> Vec<CharpenteCliStep>;
}

pub struct CharpenteModel<T: CharpenteInterface + Send + 'a> {
    _interface: &'a PhantomData<T>,
    steps: Vec<CharpenteCliStep>,
    current_step: usize,
}

impl<T: CharpenteInterface + Send + 'a> Model for CharpenteModel<T> {
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
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }

                KeyCode::Char('q') => {
                    return Some(quit());
                }

                _ => {}
            }
        }
        (self.steps.get_mut(self.current_step).unwrap().update)(msg)
    }

    fn view(&self) -> String {
        (self.steps.get(self.current_step).unwrap().view)()
    }
}
