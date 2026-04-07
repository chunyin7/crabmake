use glob::glob;
use std::process::Command;

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
}

pub struct Commander {
    compiler: Compiler,
    flags: Vec<String>,
    std: String,
    out: String,
}

impl Commander {
    pub fn new(manifest: Manifest) -> Result<Self, String> {
        let compiler = match manifest.project.lang.as_str() {
            "c" => Compiler::CC,
            "c++" => Compiler::CPP,
            _ => return Err(format!("Invalid language: {}", manifest.project.lang)),
        };

        Ok(Self {
            compiler,
            flags: manifest.build.flags,
            std: manifest.project.std,
            out: manifest.project.name,
        })
    }

    pub fn compile(&self, src: &String) -> Result<Command, String> {
        let mut cmd = Command::new(self.compiler.command());
        cmd.arg("-c");
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(format!("build/{}", src));
        cmd.args(&self.flags);
        cmd.arg(format!("-std={}", self.std));

        Ok(cmd)
    }

    pub fn link(&self, objs: &Vec<String>) -> Result<Command, String> {
        let mut cmd = Command::new(self.compiler.command());
        cmd.args(objs);
        cmd.arg("-o");
        cmd.arg(format!("build/{}", self.out));

        Ok(cmd)
    }
}

fn is_glob(pattern: &String) -> bool {
    let meta_chars = ['*', '?', '[', ']', '{', '}', '!'];
    let mut chars = pattern.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            chars.next();
            continue;
        }

        if meta_chars.contains(&c) {
            return true;
        }
    }

    false
}

pub fn build(manifest: Manifest) -> Result<Command, String> {
    let mut cmd = match manifest.project.lang.as_str() {
        "c++" => Command::new("c++"),
        "c" => Command::new("cc"),
        _ => {
            return Err("Invalid project language".to_string());
        }
    };
    let srcs = manifest.build.srcs.iter().flat_map(|s| {
        if is_glob(s) {
            glob(s)
                .unwrap()
                .filter_map(|r| match r {
                    Ok(path) => Some(path.to_string_lossy().into_owned()),
                    Err(_) => None,
                })
                .collect::<Vec<_>>()
        } else {
            vec![s.clone()]
        }
    });
    cmd.args(srcs);
    cmd.args(["-o", manifest.project.name.as_str()]);
    cmd.args(manifest.build.flags);
    cmd.arg(format!("-std={}", manifest.project.std));

    Ok(cmd)
}
