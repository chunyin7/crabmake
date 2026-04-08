use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::{
    fs,
    path::PathBuf,
};

#[derive(Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub lang: String,
    pub std: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct BuildInfo {
    pub srcs: Vec<String>,
    pub include_dirs: Vec<String>,
    pub flags: Vec<String>,
}

#[derive(Deserialize)]
pub struct Manifest {
    pub project: ProjectInfo,
    pub build: BuildInfo,
}

impl Manifest {
    pub fn new(proj_root: &PathBuf) -> Result<Self> {
        let manifest_file = proj_root.join("build.toml");

        if !manifest_file.exists() {
            bail!("No build.toml manifest file in directory.")
        }

        let content = fs::read_to_string(manifest_file).context("Failed to read manifest file.")?;
        let manifest =
            toml::from_str(&content.as_str()).context("Failed to parse manifest file.")?;
        Ok(manifest)
    }
}
