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
}

pub fn handle_save_command(matches: &ArgMatches) {
    let description = matches.get_one::<String>("description").unwrap();

    // Ensure database table exists.
    if create_table().is_err() {
        eprintln!("Failed to create database table. Please check for errors.");
        return;
    }


    if save_idea(description).is_err() {
        eprintln!("Failed to save idea. Please check the database and try again.");
        return;
    }

    println!("Idea saved successfully!");
}
