use anyhow::{Context, Result, bail};
use std::{fs, path::PathBuf, process::Command};

use crate::config::Config;

pub fn clean(ctx: &Config) -> Result<()> {
    fs::remove_dir_all(ctx.build_dir.as_path()).context("Failed to remove build directory.")?;
    Ok(())
}

pub fn create_compile(ctx: &Config, src: &PathBuf) -> Result<Command> {
    let mut cmd = ctx.compiler.command();
    cmd.arg("-c");
    cmd.arg(src);
    cmd.arg("-o");
    cmd.arg(ctx.map_src_to_output(src)?);
    cmd.args(ctx.manifest.build.flags.iter());
    cmd.arg(format!("-std={}", ctx.manifest.project.std));

    Ok(cmd)
}

pub fn create_link(ctx: &Config, objs: &Vec<String>) -> Command {
    let mut cmd = ctx.compiler.command();
    cmd.args(objs);
    cmd.arg("-o");
    cmd.arg(ctx.build_dir.join(&ctx.manifest.project.name));

    cmd
}

pub fn execute_cmd(mut cmd: Command) -> Result<()> {
    let status = cmd.status().context("Failed to launch compiler.")?;
    if !status.success() {
        bail!("Compilation failed");
    }

    Ok(())
}
