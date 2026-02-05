use bubbletea_rs::{Error, Program};

mod cli;

pub use cli::help::*;

use crate::cli::{charpente_cli::CharpenteModel, init::InitStep};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let mut program: Result<Program<_>, Error> = Err(Error::ProgramPanic(String::from(
        "Something happened while initializing the program.",
    )));

    match args[1].as_str() {
        "init" => {
            program = Program::<CharpenteModel<InitStep>>::builder()
                .alt_screen(false)
                .build();
        }

        "modules" => {}
        "hosts" => {}
        "help" | _ => {}
    }

    if let Ok(cmd) = program {
        cmd.run().await;
    }
}
