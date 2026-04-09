use anyhow::{Context, Result, bail};
use std::{fs, path::PathBuf, process::Command};

use crate::{
    config::Config,
    file::{convert_srcs, is_stale},
};

pub fn clean(ctx: &Config) -> Result<()> {
    fs::remove_dir_all(ctx.build_dir.as_path()).context("Failed to remove build directory.")?;
    Ok(())
}

pub fn compile(ctx: &Config) -> Result<PathBuf> {
    let srcs = convert_srcs(ctx)?;
    let stale = srcs
        .iter()
        .filter(|src| {
            let obj = match ctx.map_src_to_obj(src) {
                Ok(val) => val,
                Err(_) => {
                    return true;
                }
            };
            is_stale(&src, &obj).unwrap_or(true)
        })
        .collect::<Vec<_>>();
    let (mut cmds, objs): (Vec<_>, Vec<_>) = stale
        .iter()
        .map(|src| create_compile(ctx, src))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip();
    cmds.iter_mut()
        .zip(stale.iter())
        .enumerate()
        .map(|(i, (cmd, src))| {
            println!(
                "Compiling {} ({}/{})",
                src.to_string_lossy(),
                i + 1,
                stale.len()
            );
            execute_cmd(cmd)
        })
        .collect::<Result<Vec<_>>>()?;

    let (mut cmd, bin) = create_link(ctx, &objs);
    println!("Linking");
    execute_cmd(&mut cmd)?;

    Ok(bin)
}

pub fn run(bin: &PathBuf) -> Result<()> {
    let status = Command::new(bin).status().context(format!(
        "Failed to execute binary: {}",
        bin.to_string_lossy()
    ))?;

    if !status.success() {
        bail!("Failed to execute binary: {}", bin.to_string_lossy())
    }

    Ok(())
}

fn create_compile(ctx: &Config, src: &PathBuf) -> Result<(Command, PathBuf)> {
    let mut cmd = Command::new(&ctx.compiler.path);
    cmd.arg("-c");
    cmd.arg(src);
    cmd.arg("-o");

    let obj = ctx.map_src_to_obj(src)?;
    if let Some(parent) = obj.parent() {
        fs::create_dir_all(parent).context(format!(
            "Failed to create output directory: {}",
            parent.to_string_lossy()
        ))?;
    }

    cmd.arg(&obj);
    cmd.args(ctx.manifest.build.flags.iter());
    cmd.arg(format!("-std={}", ctx.manifest.project.std));
    ctx.manifest.build.include_dirs.iter().for_each(|dir| {
        cmd.arg("-I");
        cmd.arg(ctx.proj_root.join(dir));
    });
    cmd.arg("-MMD");
    cmd.arg("-MF");
    cmd.arg(ctx.map_src_to_dep(src)?);

    Ok((cmd, obj))
}

fn create_link(ctx: &Config, objs: &Vec<PathBuf>) -> (Command, PathBuf) {
    let mut cmd = Command::new(&ctx.compiler.path);
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
