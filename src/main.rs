use anyhow::Context;
use std::io::{BufRead, BufReader};
use clap::Parser;
use std::fs::{self, File};
use std::io::prelude::*;
use todo::*;

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
        Commands::RM { number } => match check_file(file_path) {
            Ok(..) => {
                let mut lines = BufReader::new(File::open(file_path).unwrap()).lines();
                let mut contents = String::new();
                let mut i = 1;
                while let Some(line) = lines.next() {
                    if i != number {
                        contents.push_str(&line.unwrap());
                        contents.push_str("\n");
                    }
                    i += 1;
                }
                fs::write(file_path, contents).expect("Unable to write file");

            }
            Err(e) => println!("Error: {}", e),
        },

        Commands::List {} => {
            let file_path = "C:\\Users\\jackwill\\Desktop\\todo.txt";

            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");

            println!("{contents}");
        }

        Commands::Done { number } => match check_file(file_path){
            Ok(..) => {

                let mut lines = BufReader::new(File::open(file_path).unwrap()).lines();
                let mut contents = String::new();
                let mut i = 1;
                while let Some(line) = lines.next() {
                    if i != number {
                        contents.push_str(&line.unwrap());
                        contents.push_str("\n");
                    }else{
                        // change [] to [x]
                        let mut line = line.unwrap();
                        line.replace_range(1..2, "x");
                        contents.push_str(&line);
                        contents.push_str("\n");

                    }
                    i += 1;
                }
                fs::write(file_path, contents).expect("Unable to write file");
            }
            Err(e) => println!("Error: {}", e),
        }



        Commands::Init { path } => {
            let initial_file_path = path.clone();

            // Append "todo.txt" to the end of the file_path
            let file_path = format!("{}\\{}", initial_file_path, "todo.txt");

            // Create a new File at the specified path
            let _todofile = File::create(&file_path);
            println!("Creating file: {}", file_path);
        }
    }
}
