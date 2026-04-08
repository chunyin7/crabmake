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
