mod commands;
mod parser;

use crate::commands::diff::DiffArgs;
use crate::commands::diff::diff;
use crate::commands::list::{ListArgs, list};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    /// Compare two or more .env files. Example: envy diff .env .env.example .env.production
    Diff(DiffArgs),
    List(ListArgs),
}

#[derive(Parser)]
#[command(about = "A tool for working with .env files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Diff(args) => diff(args),
        Commands::List(args) => list(args),
    }
}
