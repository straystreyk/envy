mod commands;
mod parser;

use crate::commands::diff::DiffArgs;
use crate::commands::diff::diff;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Diff(DiffArgs),
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Diff(args) => diff(args),
    }
}
