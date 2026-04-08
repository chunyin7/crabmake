use crate::config::Config;
use clap::Parser;

mod commands;
mod config;
mod file;
mod manifest;
mod util;

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
    Build {},

    /// clean build artifacts
    #[command()]
    Clean {},

    #[command()]
    Run {},
}

fn main() {
    let ctx = match Config::new() {
        Ok(val) => val,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    };

    let commands = Commands::parse();

    match commands {
        Commands::Init { name } => { /* todo */ }
        Commands::Build {} => { /* todo */ }
        Commands::Clean {} => { /* todo */ }
        Commands::Run {} => { /* todo */ }
    }
}
