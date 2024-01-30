use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::fs::OpenOptions;

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
    List { all: bool },

    /// complete todo
    Complete { task: String },

    /// show the status of a todo list
    Status { task: bool },

    /// path of the todo file
    Path { path: String },
}

pub fn check_file(file_path: &str) -> Result<File> {
    let todofile = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .with_context(|| format!("Failed to open or create file: {}", file_path))?;

    Ok(todofile)
}

