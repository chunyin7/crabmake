use std::path::PathBuf;

use crate::context::Context;
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

pub fn convert_srcs(ctx: &Context) -> Vec<PathBuf> {
    ctx.manifest
        .build
        .srcs
        .iter()
        .flat_map(|s| {
            if is_glob(s) {
                glob(s)
                    .unwrap()
                    .filter_map(|r| match r {
                        Ok(path) => Some(ctx.proj_root.join(path)),
                        Err(_) => None,
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![ctx.proj_root.join(s)]
            }
        })
        .collect::<Vec<_>>()
}
