use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{BufRead, BufReader};


/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "todo")]
#[command(about = "a todo CLI writtin in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// add todo
    Add { task: String },

    /// remove todo
    Remove { task: String },

    /// list todos
    List {},

    /// complete todo
    Done { task: String },

}

pub fn check_file(file_path: &str) -> Result<File> {
    let todofile = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .with_context(|| format!("Failed to open or create file: {}", file_path))?;

    Ok(todofile)
}


// pub fn read_line(path: &str) ->  i32 {
//     let file = BufReader::new(File::open(path).expect("Unable to open file"));
//     let mut cnt  = 0;
//     
//     for _ in file.lines() {
//         cnt = cnt + 1;
//     }
//     return  cnt
// }
//
//
pub fn read_line(path: &str, target_string: &str) -> Option<u32> {
    let file = BufReader::new(File::open(path).expect("Unable to open file"));
    let mut current_line_number = 0;

    for line in file.lines() {
        current_line_number += 1;

        if let Ok(content) = line {
            if content.contains(target_string) {
                return Some(current_line_number);
            }
        }
    }
    if  current_line_number == 0 {
        return Some(0);
    }else{
        return Some(current_line_number);
    }


}
