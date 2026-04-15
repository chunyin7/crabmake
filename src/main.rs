use crate::{
    commands::{clean, compile, init, run},
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
        /// language of the project
        lang: String,
        /// language standard to use
        std: Option<String>,
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
    let commands = Commands::parse();

    match commands {
        Commands::Init { name, lang, std } => handle(init(&name, &lang, &std)),
        Commands::Build { release } => {
            let ctx = handle(Config::new(release));
            handle(compile(&ctx));
        }
        Commands::Clean {} => {
            let ctx = handle(Config::new(false));
            handle(clean(&ctx))
        }
        Commands::Run { release } => {
            let ctx = handle(Config::new(release));
            handle(compile(&ctx));
            handle(run(&ctx.bin))
        }
    }
}
