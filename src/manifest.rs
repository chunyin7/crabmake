use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Serialize)]
pub struct ProjectInfo {
    pub name: String,
    pub lang: String,
    pub std: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
pub struct BuildInfo {
    pub srcs: Vec<String>,
    pub include_dirs: Vec<String>,
    pub flags: Vec<String>,
}

#[derive(Deserialize, Serialize)]
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

    pub fn default(name: &String, lang: &String, std: &Option<String>) -> Result<Self> {
        let std = if let Some(s) = std {
            s.clone()
        } else {
            match lang.as_str() {
                "c" => "c17".to_string(),
                "c++" | "cpp" => "c++17".to_string(),
                _ => bail!("Invalid language specified."),
            }
        };

        Ok(Self {
            project: ProjectInfo {
                name: name.clone(),
                lang: lang.clone(),
                std: std,
                version: "0.1.0".to_string(),
            },
            build: BuildInfo {
                srcs: Vec::new(),
                include_dirs: Vec::new(),
                flags: Vec::new(),
            },
        })
    }
}
