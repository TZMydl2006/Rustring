use clap::{Parser, Subcommand};
use minizensical::{Config, build_site};
use std::path::PathBuf;
use std::process;

#[derive(Debug, Parser)]
#[command(name = "minizensical")]
#[command(about = "A tiny Rust static site generator inspired by zensical.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Build {
        #[arg(long, default_value = "zensical.toml")]
        config: PathBuf,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

fn run() -> minizensical::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { config } => {
            let config = Config::load(config)?;
            build_site(&config)
        }
    }
}
