use anyhow::Context;
use clap::Parser;
use std::fmt::format;
use std::{env, io};
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use todo_cli_app::*;

fn main() {
    let args = Cli::parse();
    let file_path = match args.file {
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
                let _ = check_file(&file_path);
                println!("File created at: {:?}", file_path)
            }
        }
        Commands::Add { task } => match check_file(&file_path) {
            Ok(mut todofile) => {
                writeln!(
                    todofile,
                    "[{}] {}",
                    read_line(&file_path, &task).unwrap(),
                    task
                )
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

        Commands::Done { number } => match check_file(&file_path) {
            Ok(..) => {
                let mut lines = BufReader::new(File::open(&file_path).unwrap()).lines();
                let mut contents = String::new();
                let mut i = 1;
                while let Some(line) = lines.next() {
                    if i != number {
                        contents.push_str(&line.unwrap());
                        contents.push_str("\n");
                    } else {
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
        },

        Commands::Clear {} => {
            fs::write(&file_path, "").expect("Unable to write file");
        }

        Commands::SORT {} => {
            let contents =
                fs::read_to_string(&file_path).expect("Should have been able to read the file");
            let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
            lines.sort();
            let sorted_contents = lines.join("\n");
            fs::write(&file_path, sorted_contents).expect("Unable to write file");
            // correct number in []

            let mut lines = BufReader::new(File::open(&file_path).unwrap()).lines();
            let mut contents = String::new();
            let mut i = 1;
            // skip the line begins with [x]

            while let Some(line) = lines.next() {
                if line.as_ref().unwrap().starts_with("[x]") {
                    contents.push_str(&line.unwrap());
                    contents.push_str("\n");
                } else {
                    // replace the  [number]
                    let mut line = line.unwrap();
                    line.replace_range(1..2, &i.to_string());
                    contents.push_str(&line);
                    contents.push_str("\n");
                    i += 1;
                }
            }

            fs::write(&file_path, contents).expect("Unable to write file");

            let contents =
                fs::read_to_string(&file_path).expect("Should have been able to read the file");

            println!("{contents}");
        }

        Commands::EDIT { number } => match check_file(&file_path) {
            Ok(_todofile) => {
                let mut lines = BufReader::new(File::open(&file_path).unwrap()).lines();
                let mut new_contents = Vec::new();
                let mut i = 1;

                while let Some(line_result) = lines.next() {
                    let line = line_result.unwrap();
                    if i == number {
                        println!("Editing task: {:?}", line);
                        // let user input new task
                        let mut new_task = String::new();
                        io::stdin()
                            .read_line(&mut new_task)
                            .expect("Failed to read line");

                        // Check if the original line already contains [number]
                        // If yes, keep it unchanged; otherwise, add [number]
                        if line.contains(&format!("[{}]", number)) {
                            new_contents.push(line);
                        } else {
                            new_contents.push(format!("[{}] {}", number, new_task.trim()));
                        }
                    } else {
                        new_contents.push(line);
                    }
                    i += 1;
                }

                // Write the modified contents back to the file
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&file_path)
                    .unwrap();
                for line in new_contents {
                    writeln!(file, "{}", line).unwrap();
                }
            }

            Err(e) => println!("Error: {}", e),
        }

    }
}
