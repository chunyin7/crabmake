use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct ProjectInfo {
    name: String,
    lang: String,
    std: String,
    version: String,
}

#[derive(Deserialize)]
struct BuildInfo {
    srcs: Vec<String>,
    include_dirs: Vec<String>,
    flags: Vec<String>,
}

#[derive(Deserialize)]
pub struct Manifest {
    project: ProjectInfo,
    build: BuildInfo,
}

impl Manifest {
    pub fn new() -> Option<Self> {
        let working_dir = Path::new(".");
        let manifest_file = working_dir.join("build.toml");

        if !manifest_file.exists() {
            return None;
        }

        let content = match fs::read_to_string(manifest_file) {
            Ok(content) => content,
            Err(_) => {
                return None;
            }
        };

        match toml::from_str(&content.as_str()) {
            Ok(manifest) => manifest,
            Err(_) => None,
        }
    }
}
