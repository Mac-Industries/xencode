use crate::database::database::{get_all_ideas, get_idea};
use clap::{Arg, ArgMatches, Command};

pub fn build_get_command() -> Command {
    Command::new("get").about("Retrieve a saved idea").arg(
        Arg::new("id")
            .short('i')
            .long("id")
            .value_name("ID")
            .required(true)
            .help("ID of the idea to retrieve"),
    )
}

pub fn handle_get_command(subcmd: &ArgMatches) {
    if let Some(id) = subcmd.value_of("id") {
        if let Ok(idea) = get_idea(id) {
            println!("Idea {}:", id);
            println!("Description: {}", idea.description);
            if let Some(content) = idea.content {
                println!("Content:\n{}", content);
            }
        } else {
            println!("Idea with ID {} not found.", id);
        }
    } else {
        if let Ok(ideas) = get_all_ideas() {
            if ideas.is_empty() {
                println!("No ideas saved yet.");
            } else {
                println!("All saved ideas:");
                for idea in ideas {
                    println!("- ID: {}", idea.id);
                    println!("  Description: {}", idea.description);
                }
            }
        } else {
            println!("Failed to retrieve ideas.");
        }
    }
}
