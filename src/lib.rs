use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "todo-cli-app")]
#[command(about = "Yet another todo CLI app written in Rust", long_about = None)]
pub struct Cli {
    /// The path to the file to read/write!
    #[arg(short, long)]
    pub file: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new todo file
    Init { path: Option<String> },

    /// add tasks
    Add { task: String },

    /// remove tasks
    RM { number: usize },

    /// list tasks
    List {},

    /// complete tasks
    Done { number: usize },

    /// clear all tasks
    Clear {},

    /// sort tasks
    SORT {},

    /// edit a task
    EDIT { number: usize },

    /// add tags for the tasks sync from other device
    SYNC {},
}

pub fn check_file(file_path: &str) -> Result<File> {
    let todofile = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .with_context(|| format!("Failed to open or create file: {}", file_path))?;

    Ok(todofile)
}

pub fn read_line(path: &str, target_string: &str) -> Option<u32> {
    let file = BufReader::new(File::open(path).expect("Unable to open file"));
    let mut current_line_number = 1;

    for line in file.lines() {
        current_line_number += 1;

        if let Ok(content) = line {
            if content.contains(target_string) {
                return Some(current_line_number);
            }
        }
    }
    if current_line_number == 0 {
        return Some(0);
    } else {
        return Some(current_line_number);
    }
}
