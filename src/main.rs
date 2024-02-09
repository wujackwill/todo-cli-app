use anyhow::Context;
use clap::Parser;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::{env, io};
use todo_cli_app::*;
use regex::Regex;

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

        Commands::List {} => match check_file(&file_path) {
            Ok(_todofile) => {
                let contents =
                    fs::read_to_string(&file_path).expect("Should have been able to read the file");

                println!("{contents}");
            }
            Err(e) => println!("Error: {}", e),
        },

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
                        // If yes, replace the content between [number] and the next space
                        // If not, add [number] to the beginning of the new task
                        if let Some(idx) = line.find(&format!("[{}]", number)) {
                            let prefix = &line[..idx];
                            let suffix = new_task.trim();
                            new_contents.push(format!("{}[{}] {}", prefix, number, suffix));
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
        Commands::SYNC {} => {
            // Read the file and create a buffered reader
            let file = File::open(&file_path).expect("Unable to open file");
            let reader = BufReader::new(file);
            let mut contents = String::new();
            let mut line_number = 0;
            let re_skip = Regex::new(r"^\[x\]").unwrap();
            let re_num = Regex::new(r"^\[(\d+)\]").unwrap();

            // Iterate over each line in the file
            for line in reader.lines() {
                // Unwrap the line or exit if there's an error
                let line = line.expect("Unable to read line");

                if re_skip.is_match(&line) {
                    // If line starts with "[x]", append it to contents without modifying
                    contents.push_str(&line);
                } else {
                    // If the line doesn't start with "[x]", increment line number and prepend it with the new line number
                    line_number += 1;
                    let line_with_number = if let Some(caps) = re_num.captures(&line) {
                        // If the line already has a number, replace it with the new line number
                        format!("[{}] {}", line_number, &line[caps[1].len() + 2..].trim_start())
                    } else {
                        // If the line doesn't have a number, add the new line number
                        format!("[{}] {}", line_number, line.trim_start())
                    };
                    contents.push_str(&line_with_number);
                }

                // Add a newline character to the end of the line
                contents.push('\n');
            }
            fs::write(&file_path, contents).expect("Unable to write file");

            let contents =
                fs::read_to_string(&file_path).expect("Should have been able to read the file");

            println!("{contents}");
        }
    }
}
