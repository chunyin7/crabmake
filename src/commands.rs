use glob::glob;
use std::{fs, path::PathBuf, process::Command};

use crate::{context::Context, manifest::Manifest};

pub fn clean(ctx: &Context) -> Result<(), String> {
    match fs::remove_dir_all(ctx.build_dir.as_path()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to remove build directory.".to_string()),
    }
}

pub fn create_compile(ctx: &Context, src: &PathBuf) -> Result<Command, String> {
    let mut cmd = ctx.compiler.command();
    cmd.arg("-c");
    cmd.arg(src);
    cmd.arg("-o");
    cmd.arg(ctx.map_src_to_output(src)?);
    cmd.args(ctx.manifest.build.flags.iter());
    cmd.arg(format!("-std={}", ctx.manifest.project.std));

    Ok(cmd)
}

pub fn create_link(ctx: &Context, objs: &Vec<String>) -> Result<Command, String> {
    let mut cmd = ctx.compiler.command();
    cmd.args(objs);
    cmd.arg("-o");
    cmd.arg(ctx.build_dir.join(&ctx.manifest.project.name));

    Ok(cmd)
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
