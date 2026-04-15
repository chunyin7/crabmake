use anyhow::{Context, Result, bail};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{fs, io::Write, os::unix::ffi::OsStrExt, path::PathBuf, process::Command, sync::Mutex};

use crate::{
    config::Config,
    file::{convert_srcs, is_stale, parse_dep_file},
};

pub fn clean(ctx: &Config) -> Result<()> {
    match fs::remove_dir_all(ctx.build_dir.as_path()) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e).context("Failed to remove build directory."),
    }
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

    let stderr = Mutex::new(std::io::stderr());
    let stdout = Mutex::new(std::io::stdout());
    stale
        .par_iter()
        .map(|(src, obj, dep)| -> Result<_> {
            let mut cmd = create_compile(ctx, src, obj, dep)?;
            {
                let mut out = stdout.lock().unwrap();
                out.write_all(b"Compiling: ")?;
                out.write_all(src.as_os_str().as_bytes())?;
                out.write_all(b"\n")?;
            }
            let output = execute_cmd(&mut cmd)?;
            let mut err = stderr.lock().unwrap();
            err.write_all(&output)?;

            Ok(())
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
        let output = execute_cmd(&mut cmd)?;
        let mut err = stderr.lock().unwrap();
        err.write_all(&output)?;
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

    if ctx.release {
        cmd.arg("-O2");
    } else {
        cmd.arg("-O0");
        cmd.arg("-g");
    }

    Ok(cmd)
}

fn create_link(ctx: &Config, objs: &Vec<&PathBuf>) -> Command {
    let mut cmd = Command::new(&ctx.compiler.path);
    cmd.args(objs);
    cmd.arg("-o");
    cmd.arg(&ctx.bin);

    cmd
}

fn execute_cmd(cmd: &mut Command) -> Result<Vec<u8>> {
    let output = cmd.output().context("Failed to launch compiler.")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Compilation failed:\n{stderr}");
    }

    Ok(output.stderr)
}
