# crabmake

A minimal C/C++ Cargo inspired build tool written in Rust

> [!NOTE]
> This is meant for small projects of my own as an ergonomic substitute for make/cmake and is not guaranteed to be good for production.

## Features

- Minimal CLI interface: `build`, `run`, `clean`
- TOML-based project configuration
- Incremental builds using dependency files
- Parallel compilation
- C and C++ support

## Usage

```
crabmake build    # compile and link
crabmake run      # compile, link, and run
crabmake clean    # remove build artifacts
```

## Manifest

Create a `build.toml` in your project root:

```toml
[project]
name = "myapp"
lang = "c"
std = "c99"
version = "0.1.0"

[build]
srcs = ["src/**/*.c"]
include_dirs = ["src"]
flags = ["-Wall", "-Wextra"]
```

## Building from source

```
cargo build --release
```
