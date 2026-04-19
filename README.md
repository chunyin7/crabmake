# crabmake

A minimal C/C++ Cargo inspired build tool written in Rust

> [!NOTE]
> This is meant for small projects of my own as an ergonomic substitute for make/cmake and is not guaranteed to be good for production.

## Features

- Minimal CLI interface: `init`, `build`, `run`, `clean`, `compdb`
- Project scaffolding with a hello-world starter
- TOML-based project configuration
- Incremental builds using dependency files
- Parallel compilation
- C and C++ support

## Usage

```
crabmake init <name> <lang> [std]   # scaffold a new project
crabmake build              # debug build (no optimisation, includes debug symbols)
crabmake build --release    # release build (-O2)
crabmake run                # build and run (debug)
crabmake run --release      # build and run (release)
crabmake clean              # remove build artifacts
crabmake compdb             # generate compile_commands.json
```

## Creating a project

```
crabmake init myapp c           # C project, defaults to -std=c17
crabmake init myapp c++         # C++ project, defaults to -std=c++17
crabmake init myapp c c11       # override the language standard
crabmake init . c               # scaffold into the current directory
```

`lang` must be `c` or `c++`. `std` is optional; when omitted it defaults to `c17` or `c++17` based on the language.

This generates:

- `build.toml` with a default manifest (`srcs = ["src/main.c"]` or `src/main.cpp`)
- `src/main.c` or `src/main.cpp` containing a hello-world stub

You can then `cd` into the project and run `crabmake run`.

## Manifest

`crabmake init` generates this file for you, but you can also write one by hand. Create a `build.toml` in your project root:

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

## Compile Commands

`crabmake compdb` generates a `compile_commands.json` in the project root. This is a [compilation database](https://clang.llvm.org/docs/JSONCompilationDatabase.html) used by tools like clangd and clang-tidy for code intelligence (autocomplete, diagnostics, go-to-definition, etc.).

```
crabmake compdb
```

## Building from source

```
cargo build --release
```
