use bubbletea_rs::{Error, Program};

mod cli;
mod lib;
mod templates;

use crate::cli::{charpente_cli::CharpenteModel, init::InitStep};

fn print_help() {
    println!("Usage: charpente <command>");
    println!("Commands:");
    println!("  init            -> Initialize Charpente in your nix config");
    println!("  modules <arg>   -> Manage modules");
    println!("          list    -> List all modules");
    println!("          add     -> Add a module");
    println!("          remove  -> Remove a module");
    println!("  hosts <arg>     -> Manage hosts");
    println!("        list      -> List all hosts");
    println!("        add       -> Add a host");
    println!("        remove    -> Remove a host");
    println!("  help            -> Print this help message");
}

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
        "help" | _ => {
            print_help();
        }
    }

    if let Ok(cmd) = program {
        cmd.run().await;
    }
}
