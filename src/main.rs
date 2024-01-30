use anyhow::Context;
use clap::Parser;
use std::fs;
use std::io::prelude::*;
use todo::check_file;
use todo::Cli;
use todo::Commands;
use todo::read_line;

fn main() {
    let args = Cli::parse();
    let file_path = "C:\\Users\\jackwill\\Desktop\\todo.txt";
    match args.command {
        Commands::Add { task } => match check_file(file_path) {
            Ok(mut todofile) => {
                writeln!(todofile, "[{}] {}",  read_line("C:\\Users\\jackwill\\Desktop\\todo.txt",  &task).unwrap(),task)
                    .with_context(|| format!("Failed to write to file: {}", file_path))
                    .expect("Failed to write to file");

                println!("Adding task: {}", task);
            }
            Err(e) => println!("Error: {}", e),
        },
        Commands::Remove { task } => {
            println!("Removing task: {}", task);
        }

        Commands::List {} => {
            let file_path = "C:\\Users\\jackwill\\Desktop\\todo.txt";

            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");

            println!("{contents}");
        }

        Commands::Done { task } => {
            println!("Completing task: {}", task);
        }
    }
}
