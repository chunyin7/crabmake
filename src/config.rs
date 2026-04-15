use anyhow::{Context, Result, bail};
use std::{path::PathBuf, process::Command};
use which::which;

use crate::manifest::Manifest;

pub struct Compiler {
    pub path: PathBuf,
}

impl Compiler {
    pub fn from_lang(lang: &str) -> Result<Self> {
        let name = match lang {
            "c" => "cc",
            "c++" | "cpp" => "c++",
            _ => {
                bail!("Invalid language: {lang}")
            }
        };
        let path = which(name).context(format!("Compiler {name} not found."))?;
        Ok(Self { path })
    }
}

pub struct Config {
    pub proj_root: PathBuf,
    pub build_dir: PathBuf,
    pub output_dir: PathBuf,
    pub compiler: Compiler,
    pub manifest: Manifest,
    pub bin: PathBuf,
    pub release: bool,
}

impl Config {
    pub fn new(release: bool) -> Result<Self> {
        let proj_root = std::env::current_dir().context("Failed to read working directory.")?;
        let manifest = Manifest::new(&proj_root)?;
        let compiler = Compiler::from_lang(manifest.project.lang.as_str())?;
        let build_dir = proj_root.join("build");
        let output_dir = if release {
            build_dir.join("release")
        } else {
            build_dir.join("debug")
        };
        let bin = output_dir.join(&manifest.project.name);

        Ok(Self {
            proj_root,
            build_dir,
            output_dir,
            manifest,
            compiler,
            bin,
            release,
        })
    }

    fn src_to_relative(&self, src: &PathBuf) -> Result<PathBuf> {
        let relative = src
            .strip_prefix(&self.proj_root)
            .context(format!(
                "Failed to map source file {} to output file.",
                src.to_string_lossy()
            ))?
            .to_owned();
        Ok(relative)
    }

    pub fn map_src_to_obj(&self, src: &PathBuf) -> Result<PathBuf> {
        let relative = self.src_to_relative(src)?;
        Ok(self.output_dir.join(relative).with_extension("o"))
    }

    pub fn map_src_to_dep(&self, src: &PathBuf) -> Result<PathBuf> {
        let relative = self.src_to_relative(src)?;
        Ok(self.output_dir.join(relative).with_extension("d"))
    }
}
