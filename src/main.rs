use crate::{command::Commander, manifest::Manifest};
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
    let manifest = match Manifest::new() {
        Ok(val) => val,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    };

    let commander = match Commander::new(manifest) {
        Ok(val) => val,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    };
}
