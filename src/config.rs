use anyhow::{Context, Error, Result, bail};
use std::{path::PathBuf, process::Command};

use crate::manifest::Manifest;

pub enum Compiler {
    CC,
    CPP,
}

impl Compiler {
    pub fn command(&self) -> Command {
        match self {
            Compiler::CC => Command::new("cc"),
            Compiler::CPP => Command::new("cpp"),
        }
    }

    pub fn from_lang(lang: &str) -> Result<Self> {
        match lang {
            "c" => Ok(Self::CC),
            "c++" => Ok(Self::CPP),
            _ => {
                bail!("Invalid language: {lang}")
            }
        }
    }
}

pub struct Config {
    pub proj_root: PathBuf,
    pub build_dir: PathBuf,
    pub compiler: Compiler,
    pub manifest: Manifest,
}

impl Config {
    pub fn new() -> Result<Self> {
        let proj_root = std::env::current_dir().context("Failed to read working directory.")?;
        let manifest = Manifest::new(&proj_root)?;
        let compiler = Compiler::from_lang(manifest.project.lang.as_str())?;
        let build_dir = proj_root.join("build");

        Ok(Self {
            proj_root,
            build_dir,
            manifest,
            compiler,
        })
    }

    pub fn map_src_to_output(&self, src: &PathBuf) -> Result<PathBuf> {
        let relative = src.strip_prefix(&self.proj_root).context(format!(
            "Failed to map source file {} to output file.",
            src.to_string_lossy()
        ))?;
        Ok(self.build_dir.join(relative).with_extension("o"))
    }
}
