use crate::context::Context;
use clap::Parser;

mod commands;
mod context;
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
    let ctx = match Context::new() {
        Ok(val) => val,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    };
}
