use bubbletea_rs::{Error, Program, quit};

mod cli;

pub use cli::help::*;
pub use cli::init::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let mut program: Result<Program<InitModel>, Error> = Err(Error::ProgramPanic(String::from(
        "Something happened while initializing the program.",
    )));

    match args[1].as_str() {
        "init" => {
            program = Program::<InitModel>::builder().alt_screen(false).build();
        }

        "modules" => {
            program = Program::<InitModel>::builder().build();
        }
        "hosts" => {
            program = Program::<InitModel>::builder().build();
        }
        "help" | _ => {
            program = Program::<InitModel>::builder().build();
        }
    }

    if let Ok(cmd) = program {
        cmd.run().await;
    }
}
