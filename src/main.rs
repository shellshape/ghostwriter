mod commands;
mod util;

use clap::{Parser, Subcommand};
use commands::Command;
use env_logger::Env;
use std::ops::Deref;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Record a sequence of file changes.
    Record(commands::record::Record),
    /// Reploay a recorded sequence of file changes.
    Replay(commands::replay::Replay),
}

impl Deref for Commands {
    type Target = dyn Command;

    fn deref(&self) -> &Self::Target {
        match self {
            Commands::Record(cmd) => cmd,
            Commands::Replay(cmd) => cmd,
        }
    }
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .expect("Failed building logger");

    args.command.run().map_err(|e| e.to_string())
}
