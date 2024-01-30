use anyhow::Context;
use clap::Parser;
use std::io::prelude::*;
use todo::check_file;
use todo::Cli;
use todo::Commands;

fn main() {
    let args = Cli::parse();
    let file_path = "/mnt/c/Users/jackwill/Desktop/todo.txt";
    match args.command {
        Commands::Add { task } => match check_file(file_path) {
            Ok(mut todofile) => {
                writeln!(todofile, "{}", task)
                    .with_context(|| format!("Failed to write to file: {}", file_path))
                    .expect("Failed to write to file");
                println!("Adding task: {}", task);
            }
            Err(e) => println!("Error: {}", e),
        },
        Commands::Remove { task } => {
            println!("Removing task: {}", task);
        }

        Commands::List { all } => {
            println!("Listing all tasks: {}", all);
        }

        Commands::Complete { task } => {
            println!("Completing task: {}", task);
        }

        Commands::Status { task } => {
            println!("Status of task: {}", task);
        }
        Commands::Path { path } => {
            println!("Path of todo file: {}", path)
        }
    }
}
