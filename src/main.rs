use crate::manifest::Manifest;
use clap::Parser;

mod command;
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
    Build {},

    /// clean build artifacts
    #[command()]
    Clean {},

    #[command()]
    Run {},
}

fn main() {
    let Some(manifest) = Manifest::new() else {
        eprintln!("No build.toml manifest file.");
        std::process::exit(1);
    };
}
