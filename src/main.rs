mod index;
mod objects;
mod refs;

use std::{io::Write, path::PathBuf};

use clap::{Parser, Subcommand};
use fern::colors::{Color, ColoredLevelConfig};
use log::info;

use crate::index::create_empty;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    // Verbose
    #[arg(short, long)]
    verbose: bool,

    // Brief (no level)
    #[arg(short, long)]
    brief: bool,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        // The path of the root directory
        dir: Option<PathBuf>,

        // What should the default branch be called
        #[arg(short, long)]
        branch: Option<String>,
    },
    Index {
        glob: Option<String>,
    },
    Object {
        // subcommands
        #[command(subcommand)]
        command: ObjectCommands,
    },
}

#[derive(Subcommand)]
enum ObjectCommands {
    Add {
        // subcommand per file/dir/commit
        #[command(subcommand)]
        command: AddCommands,
    },
    Read {
        // The hash of the object to read
        hash: String,
    },
}

#[derive(Subcommand)]
enum AddCommands {
    File {
        // The path of the file to add
        path: PathBuf,
    },
    Dir {
        // The path of the directory to add
        path: PathBuf,
    },
    Commit {
        // The hash of the commit to add
        hash: String,
    },
}

fn main() {
    let cli = Cli::parse();
    setup_logger(cli.verbose, cli.brief).unwrap();

    match &cli.command {
        Commands::Init { dir, branch } => {
            create_empty(dir.clone().as_ref(), branch);
        }
        Commands::Index { glob } => {
            let files = index::get_files(glob.to_owned());
            for file in files {
                info!("{}", file);
            }
        }
        Commands::Object { command } => match command {
            ObjectCommands::Add { command } => match command {
                AddCommands::File { path } => {
                    let data = std::fs::read(path).unwrap();
                    let hash = objects::create::create_file_object(&data);
                    info!("Added file object with hash {}", hash);
                }
                _ => {
                    unimplemented!();
                }
            },
            ObjectCommands::Read { hash } => {
                let (header, data) = objects::read::read_plain_object(hash);
                info!("Header: {:?}", header);
                std::io::stdout().write_all(&data).unwrap();
            }
        },
    }
}

fn setup_logger(verbose: bool, brief: bool) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::White)
        .debug(Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::BrightBlack);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            if brief {
                return out.finish(format_args!("{}", message));
            } else {
                return out.finish(format_args!(
                    "{}: {}",
                    colors.color(record.level()).to_string().to_lowercase(),
                    message
                ));
            }
        })
        .level(if verbose {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Info
        })
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
