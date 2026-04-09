use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::config::Config;
use glob::glob;

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

pub fn convert_srcs(ctx: &Config) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for s in &ctx.manifest.build.srcs {
        if is_glob(&s) {
            let entries = glob(&s).context(format!("Failed to unwrap glob pattern: {s}"))?;
            for entry in entries {
                let path = entry.context(format!("Unreadable path for glob result: {s}"))?;
                paths.push(ctx.proj_root.join(path));
            }
        } else {
            paths.push(ctx.proj_root.join(s));
        }
    }

    Ok(paths)
}

pub fn is_stale(src: &Path, obj: &Path) -> Result<bool> {
    match fs::metadata(obj) {
        Ok(meta) => {
            let src_modified = fs::metadata(src)?.modified()?;
            let obj_modified = meta.modified()?;
            Ok(src_modified > obj_modified)
        }
        Err(_) => Ok(true),
    }
}

pub fn parse_dep_file(ctx: &Config, src: &PathBuf) -> Result<Vec<PathBuf>> {
    let dep = ctx.map_src_to_dep(src)?;
    let raw = fs::read_to_string(dep)?;
    // replace all continuations
    let joined = raw.replace("\\\n", " ");
    let deps_str = joined
        .splitn(2, ": ")
        .nth(1)
        .context("Malformed depfile.")?
        .trim();

    let mut deps: Vec<PathBuf> = Vec::new();
    let mut cur = String::new();
    let mut deps_chars = deps_str.chars();
    while let Some(c) = deps_chars.next() {
        match c {
            '\\' => {
                if let Some(next) = deps_chars.next() {
                    cur.push(next);
                }
            }
            ' ' => {
                deps.push(ctx.proj_root.join(&cur));
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    deps.push(ctx.proj_root.join(cur));

    Ok(deps)
}
