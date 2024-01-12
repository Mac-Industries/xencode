use clap::{Command, ArgMatches};
use commands::{save, get};

mod commands;
mod database;

fn main() {
    let app = Command::new("xencode")
        .version("0.1.0")
        .author("MacBobby Chibuzor")
        .about("Save and retrieve coding ideas")
        .subcommand([
            commands::save::build_save_command(),
            commands::get::build_get_command(),
        ])
        .get_matches();

    match app.subcommand() {
        Some(subcmd) {
            "save" => save::handle_save_command(subcmd),
            "get" => get::handle_get_command(subcmd),
            _ => unreachable!(),
        }
        None => println!("Please specify a command (save or get)"),
    }
}
