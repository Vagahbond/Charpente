use crate::cli::charpente_cli::{CharpenteCliStep, CharpenteInterface};

pub enum ModulesActions {
    List,
    Add,
    Remove,
}

pub struct ListModulesStep {}

pub struct AddModuleStep {}

pub struct RemoveModuleStep {}

impl CharpenteInterface for ListModulesStep {
    fn get_steps() -> Vec<CharpenteCliStep> {
        vec![CharpenteCliStep {
            input: None,
            view: |_| "Listing modules...\n".to_string(),
            update: |_, _| None,
        }]
    }
}
