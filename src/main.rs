use clap::{Parser, Subcommand};
use minizensical::{Config, DEFAULT_PREVIEW_ADDR, build_site, init_project, serve_site};
use std::path::PathBuf;
use std::process;

#[derive(Debug, Parser)]
#[command(name = "rustring")]
#[command(about = "A tiny Rust static site generator inspired by zensical.")]
#[command(version)]
#[command(long_about = "Build a static documentation site from Markdown files.

Takes a zensical.toml configuration file and a docs/ directory of Markdown
files, and produces a static site/ output directory with full-text search,
navigation, syntax highlighting, and theme support.")]
#[command(after_help = "Examples:
  rustring init           Create a new project
  rustring build          Build the site once
  rustring serve          Preview with live reload")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a new project with a default zensical.toml and docs/index.md.
    Init {
        #[arg(long, default_value = "zensical.toml")]
        config: PathBuf,
    },
    /// Build the static site from Markdown sources.
    Build {
        #[arg(long, default_value = "zensical.toml")]
        config: PathBuf,
    },
    /// Start a local preview server with live reload.
    Serve {
        #[arg(long, default_value = "zensical.toml")]
        config: PathBuf,
        #[arg(long, default_value = DEFAULT_PREVIEW_ADDR)]
        addr: String,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("rustring: {error}");
        let msg = error.to_string();
        if msg.contains("failed to read")
            || msg.contains("No such file")
            || msg.contains("cannot find")
        {
            eprintln!("HINT: Run 'rustring init' to create a new project.");
        }
        process::exit(1);
    }
}

fn run() -> minizensical::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { config } => init_project(&config),
        Commands::Build { config } => {
            let config = Config::load(config)?;
            build_site(&config)
        }
        Commands::Serve { config, addr } => {
            let config = Config::load(config)?;
            serve_site(&config, &addr)
        }
    }
}
