use std::path::PathBuf;

use crate::manifest::Manifest;

pub enum Compiler {
    CC,
    CPP,
}

impl Compiler {
    pub fn command(&self) -> &str {
        match self {
            Compiler::CC => "cc",
            Compiler::CPP => "cpp",
        }
    }

    pub fn from_lang(lang: &str) -> Result<Self, String> {
        match lang {
            "c" => Ok(Self::CC),
            "c++" => Ok(Self::CPP),
            _ => Err(format!("Invalid language: {}", lang)),
        }
    }
}

pub struct Context {
    pub proj_root: PathBuf,
    pub build_dir: PathBuf,
    pub compiler: Compiler,
    pub manifest: Manifest,
}

impl Context {
    pub fn new() -> Result<Self, String> {
        let proj_root = match std::env::current_dir() {
            Ok(val) => val,
            Err(_) => {
                return Err("Failed to read working directory".to_string());
            }
        };
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

    pub fn map_src_to_output(&self, src: &PathBuf) -> Result<PathBuf, String> {
        match src.strip_prefix(&self.proj_root) {
            Ok(relative) => Ok(self.build_dir.join(relative).with_extension("o")),
            Err(_) => Err(format!(
                "Failed to map source {} to output file.",
                src.to_string_lossy()
            )),
        }
    }
}
