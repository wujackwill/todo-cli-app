use anyhow::Context;
use todo_cli_app::*;
use std::env;
use std::io::{BufRead, BufReader};
use clap::Parser;
use std::fs::{self, File};
use std::io::prelude::*;

fn main() {
    let args = Cli::parse();
    let file_path = match args.file{
        Some(file) => file,
        None => String::from(format!("{}/todo.txt", env::var("HOME").unwrap())),
    };

    match args.command {
        Commands::Init { path } => {
                let file_path = match path {
                    Some(custom_path) => {
                        format!("{}/todo.txt", custom_path)
                    }
                    None => {
                        format!("{}/todo.txt", env::var("HOME").unwrap())
                    }
                };

                if fs::metadata(&file_path).is_ok() {
                    println!("File exists at: {:?}", file_path);
                } else {
                    println!("File does not exist at: {:?}", file_path);
                    let _= check_file(&file_path);
                    println!("File created at: {:?}", file_path)
                }

        }
        Commands::Add { task } => match check_file(&file_path) {
            Ok(mut todofile) => {
                writeln!(todofile, "[{}] {}",  read_line(&file_path,  &task).unwrap(),task)
                    .with_context(|| format!("Failed to write to file: {}", file_path))
                    .expect("Failed to write to file");

                println!("Adding task: {}", task);
            }
            Err(e) => println!("Error: {}", e),
        },
        Commands::RM { number } => match check_file(&file_path) {
            Ok(..) => {
                let mut lines = BufReader::new(File::open(&file_path).unwrap()).lines();
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

            let contents =
                fs::read_to_string(&file_path).expect("Should have been able to read the file");

            println!("{contents}");
        }

        Commands::Done { number } => match check_file(&file_path){
            Ok(..) => {

                let mut lines = BufReader::new(File::open(&file_path).unwrap()).lines();
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
                fs::write(&file_path, contents).expect("Unable to write file");
            }
            Err(e) => println!("Error: {}", e),
        }

    }
}
