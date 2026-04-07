use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
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
    pub fn new(proj_root: &PathBuf) -> Result<Self, String> {
        let manifest_file = proj_root.join("build.toml");

        if !manifest_file.exists() {
            return Err("No build.toml manifest file in directory.".to_string());
        }

        let content = match fs::read_to_string(manifest_file) {
            Ok(content) => content,
            Err(_) => {
                return Err("Failed to read manifest file.".to_string());
            }
        };

        match toml::from_str(&content.as_str()) {
            Ok(manifest) => Ok(manifest),
            Err(_) => Err("Failed to parse manifest file.".to_string()),
        }
    }
}
