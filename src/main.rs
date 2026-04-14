use crate::{
    commands::{clean, compile, run},
    config::Config,
};
use anyhow::Result;
use clap::Parser;

mod commands;
mod config;
mod file;
mod manifest;

#[derive(Parser)]
enum Commands {
    /// create a new crabmake project
    #[command()]
    Init {
        /// name of the crabmake project
        name: String,
    },

    /// compile and link
    #[command()]
    Build {
        #[arg(long)]
        release: bool,
    },

    /// clean build artifacts
    #[command()]
    Clean {},

    #[command()]
    Run {
        #[arg(long)]
        release: bool,
    },
}

fn handle<T>(result: Result<T>) -> T {
    match result {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e:#}");
            std::process::exit(1);
        }
    }
}

fn main() {
    let ctx = handle(Config::new());
    let commands = Commands::parse();

    match commands {
        Commands::Init { name } => { /* todo */ }
        Commands::Build { release } => {
            handle(compile(&ctx, release));
        }
        Commands::Clean {} => handle(clean(&ctx)),
        Commands::Run { release } => {
            handle(compile(&ctx, release));
            handle(run(&ctx.bin))
        }
    }
}
