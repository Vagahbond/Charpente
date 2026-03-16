use std::time::Duration;

use bubbletea_rs::{Cmd, KeyMsg, Model, Msg, quit, tick};
use bubbletea_widgets::textinput;
use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug)]
pub enum CharpenteCliMsg {
    Start,
    Update,
}

impl CharpenteCliMsg {
    fn start() -> Cmd {
        Box::pin(async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            Some(Box::new(Self::Start) as Msg)
        })
    }

    pub fn update() -> Cmd {
        Box::pin(async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            Some(Box::new(Self::Update) as Msg)
        })
    }
}

pub trait CharpenteCliPage {
    fn init() -> (Self, Option<Cmd>)
    where
        Self: Sized;
    fn view(&self) -> String;
    fn update(&mut self, msg: Msg) -> Option<Cmd>;
}

pub struct CharpenteCliModel<T: CharpenteCliPage + Send + 'static> {
    page: T,
}

impl<T: Send + 'static + CharpenteCliPage> CharpenteCliModel<T> {
    pub fn prepare_input(label: &str, placeholder: &str) -> textinput::Model {
        let mut input = textinput::new();
        input.prompt = label.to_string();
        input.placeholder = placeholder.to_string();
        input
    }
}

impl<T: CharpenteCliPage + Send + 'static> Model for CharpenteCliModel<T> {
    fn init() -> (Self, Option<Cmd>) {
        let res = T::init();
        (Self { page: res.0 }, Some(CharpenteCliMsg::start()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if key_msg.key == KeyCode::Char('c')
                && key_msg.modifiers.contains(KeyModifiers::CONTROL)
            {
                return Some(quit());
            }
        }

        self.page.update(msg)
    }

    fn view(&self) -> String {
        self.page.view()
    }
}
