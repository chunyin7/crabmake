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
