use anyhow::{Context, Result, bail};
use std::{fs, path::PathBuf, process::Command};

use crate::{
    config::Config,
    file::{convert_srcs, is_stale, parse_dep_file},
};

pub fn clean(ctx: &Config) -> Result<()> {
    fs::remove_dir_all(ctx.build_dir.as_path()).context("Failed to remove build directory.")?;
    Ok(())
}

pub fn compile(ctx: &Config) -> Result<()> {
    let srcs = convert_srcs(ctx)?;
    let units: Vec<(PathBuf, PathBuf, PathBuf)> = srcs
        .into_iter()
        .map(|src| -> Result<_> {
            let obj = ctx.map_src_to_obj(&src)?;
            let dep = ctx.map_src_to_dep(&src)?;
            Ok((src, obj, dep))
        })
        .collect::<Result<Vec<_>>>()?;
    let stale = units
        .iter()
        .filter(|(_, obj, dep)| {
            let deps = match parse_dep_file(ctx, dep) {
                Ok(val) => val,
                Err(_) => return true,
            };
            deps.iter().any(|dep| is_stale(&dep, &obj).unwrap_or(true))
        })
        .collect::<Vec<_>>();
    stale
        .iter()
        .enumerate()
        .map(|(i, (src, obj, dep))| -> Result<_> {
            let mut cmd = create_compile(ctx, src, obj, dep)?;
            println!(
                "Compiling {} ({}/{})",
                src.to_string_lossy(),
                i + 1,
                stale.len()
            );
            execute_cmd(&mut cmd)
        })
        .collect::<Result<Vec<_>>>()?;

    let objs: Vec<&PathBuf> = units.iter().map(|(_, obj, _)| obj).collect::<Vec<_>>();
    if !stale.is_empty()
        || !ctx.bin.exists()
        || objs
            .iter()
            .any(|obj| is_stale(obj, &ctx.bin).unwrap_or(true))
    {
        let mut cmd = create_link(ctx, &objs);
        println!("Linking");
        execute_cmd(&mut cmd)?;
    }

    Ok(())
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

fn create_compile(ctx: &Config, src: &PathBuf, obj: &PathBuf, dep: &PathBuf) -> Result<(Command)> {
    let mut cmd = Command::new(&ctx.compiler.path);
    cmd.arg("-c");
    cmd.arg(src);
    cmd.arg("-o");

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
    cmd.arg(dep);

    Ok(cmd)
}

fn create_link(ctx: &Config, objs: &Vec<&PathBuf>) -> Command {
    let mut cmd = Command::new(&ctx.compiler.path);
    cmd.args(objs);
    cmd.arg("-o");
    cmd.arg(&ctx.bin);

    cmd
}

fn execute_cmd(cmd: &mut Command) -> Result<()> {
    let status = cmd.status().context("Failed to launch compiler.")?;
    if !status.success() {
        bail!("Compilation failed");
    }

    Ok(())
}
