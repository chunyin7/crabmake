use anyhow::{Context, Result, bail};
use std::{fs, path::PathBuf, process::Command};

use crate::{config::Config, file::convert_srcs};

pub fn clean(ctx: &Config) -> Result<()> {
    fs::remove_dir_all(ctx.build_dir.as_path()).context("Failed to remove build directory.")?;
    Ok(())
}

pub fn compile(ctx: &Config) -> Result<PathBuf> {
    let srcs = convert_srcs(ctx)?;
    let (mut cmds, objs): (Vec<_>, Vec<_>) = srcs
        .iter()
        .map(|src| create_compile(ctx, src))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip();
    cmds.iter_mut()
        .map(|mut cmd| execute_cmd(&mut cmd))
        .collect::<Result<Vec<_>>>()?;

    let (mut cmd, bin) = create_link(ctx, &objs);
    execute_cmd(&mut cmd)?;

    Ok(bin)
}

fn create_compile(ctx: &Config, src: &PathBuf) -> Result<(Command, PathBuf)> {
    let mut cmd = ctx.compiler.command();
    cmd.arg("-c");
    cmd.arg(src);
    cmd.arg("-o");
    let obj = ctx.map_src_to_output(src)?;
    cmd.arg(&obj);
    cmd.args(ctx.manifest.build.flags.iter());
    cmd.arg(format!("-std={}", ctx.manifest.project.std));

    Ok((cmd, obj))
}

fn create_link(ctx: &Config, objs: &Vec<PathBuf>) -> (Command, PathBuf) {
    let mut cmd = ctx.compiler.command();
    cmd.args(objs);
    cmd.arg("-o");
    let bin = ctx.build_dir.join(&ctx.manifest.project.name);
    cmd.arg(&bin);

    (cmd, bin)
}

fn execute_cmd(cmd: &mut Command) -> Result<()> {
    let status = cmd.status().context("Failed to launch compiler.")?;
    if !status.success() {
        bail!("Compilation failed");
    }

    Ok(())
}
