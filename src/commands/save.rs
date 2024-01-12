use std::fs;

use clap::{Arg, ArgMatches, Command};

use crate::database::database::{create_table, save_idea};

pub fn build_save_command() -> Command {
    Command::new("save")
        .about("Save a new idea")
        .arg(
            Arg::new("description")
                .short('d')
                .long("description")
                .value_name("DESCRIPTION")
                .required(true)
                .help("Brief description of the idea"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .required(true)
                .help("File containing the detailed idea"),
        )
}

pub fn handle_save_command(matches: &ArgMatches) {
    let description = matches.value_of("description").unwrap();
    let file_path = matches.value_of("file").unwrap();

    // Ensure database table exists.
    if create_table().is_err() {
        eprintln!("Failed to create database table. Please check for errors.");
        return;
    }

    let content = match file_path {
        Some(path) => match fs::read_to_string(path) {
            Ok(content) => Some(content),
            Err(err) => {
                eprintln!("Failed to read file: {}", err);
                None
            }
        },
        None => None,
    };

    if save_idea(description, content).is_err() {
        eprintln!("Failed to save idea. Please check the database and try again.");
        return;
    }

    println!("Idea saved successfully!");
}
