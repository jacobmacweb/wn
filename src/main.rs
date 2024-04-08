use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        // The path of the root directory
        dir: PathBuf,

        // What should the default branch be called
        #[arg(short, long)]
        branch: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { dir, branch } => {
            println!("Initializing a new project in {:?}, {:?}", dir, branch);
        }
    }
}
