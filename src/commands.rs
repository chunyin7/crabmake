use anyhow::{Context, Result, bail};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{fs, io::Write, os::unix::ffi::OsStrExt, path::PathBuf, process::Command, sync::Mutex};

use crate::{
    config::Config,
    file::{convert_srcs, is_stale, parse_dep_file},
    manifest::Manifest,
};

pub fn init(name: &String, lang: &String, std: &Option<String>) -> Result<()> {
    let cur_dir = std::env::current_dir().context("Failed to read working directory.")?;
    let proj_root = if name == "." {
        cur_dir
    } else {
        let proj_root = cur_dir.join(name);
        fs::create_dir(&proj_root).context("Failed to create project directory.")?;
        proj_root
    };

    let (main_filename, main_content) = match lang.as_str() {
        "c" => (
            "main.c",
            "#include <stdio.h>\n\nint main(void) {\n    printf(\"Hello, world!\\n\");\n    return 0;\n}\n",
        ),
        "cpp" | "c++" => (
            "main.cpp",
            "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, world!\\n\";\n    return 0;\n}\n",
        ),
        other => bail!("Unsupported language: {}. Expected 'c' or 'cpp'.", other),
    };

    let mut manifest = Manifest::default(&name, &lang, &std)?;
    manifest.build.srcs.push(format!("src/{}", main_filename));

    let manifest_path = proj_root.join("build.toml");
    let manifest_content = toml::to_string(&manifest).context("Failed to serialize manifest")?;
    std::fs::write(manifest_path, manifest_content).context("Failed to write build.toml")?;

    let src_dir = proj_root.join("src");
    fs::create_dir_all(&src_dir).context("Failed to create src directory.")?;
    let main_path = src_dir.join(main_filename);
    fs::write(&main_path, main_content).context("Failed to write main source file.")?;

    Ok(())
}

pub fn clean(ctx: &Config) -> Result<()> {
    match fs::remove_dir_all(ctx.build_dir.as_path()) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e).context("Failed to remove build directory."),
    }
}

pub fn compdb(ctx: &Config) -> Result<()> {
    let srcs = convert_srcs(ctx)?;
    let entries = srcs
        .into_iter()
        .map(|src| -> Result<_> {
            let obj = ctx.map_src_to_obj(&src)?;
            let dep = ctx.map_src_to_dep(&src)?;
            let cmd = create_compile(ctx, &src, &obj, &dep)?;
            let args = std::iter::once(cmd.get_program())
                .chain(cmd.get_args())
                .map(|s| s.to_string_lossy().to_owned())
                .collect::<Vec<_>>();
            let entry = serde_json::json!({
                "file": src,
                "directory": ctx.proj_root,
                "arguments": args,
                "output": obj,
            });
            Ok(entry)
        })
        .collect::<Result<Vec<_>>>()?;
    let compile_commands = ctx.proj_root.join("compile_commands.json");
    fs::write(&compile_commands, serde_json::to_string_pretty(&entries)?)?;
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
