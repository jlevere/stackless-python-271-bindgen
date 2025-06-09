# Stackless Python 2.7.1 Rust Bindings Generator

Pre-generated Rust bindings for [Stackless Python 2.7.1](https://github.com/stackless-dev/stackless), automatically built and released via GitHub Actions.

## Why This Exists

Generating Rust bindings for Stackless Python is complex because you need:

- Stackless Python headers (built with Nix on Linux)
- Windows build environment for proper target compatibility
- Rust + bindgen with specific MSVC compiler flags
- You can't run Nix on Windows

This repository solves that by automatically generating and releasing the bindings for you.

## Usage

Download the latest `bindings.rs` from [Releases](../../releases) and include it in your Rust project:

```rust
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(unnecessary_transmutes)]

include!("bindings.rs");
```

## What's Generated

The bindings include core Stackless Python types:

- `PyTaskletObject`, `PyStacklessState` - Stackless-specific objects
- `PyObject`, `PyThreadState`, `PyFrameObject` - Standard Python objects
- `PyIntObject`, `PyStringObject`, `PyDictObject` - Built-in types
- All `Py*` functions from the Python C API

## Build Process

1. **Linux**: Build Stackless Python headers using Nix
2. **Windows**: Generate bindings with bindgen targeting `x86_64-pc-windows-gnu`
3. **Release**: Automatically create GitHub releases when `Cargo.toml` version changes

## Manual Build

If you need to build locally:

```bash
# Set environment variables
export PYTHON_HEADER_PATH=/path/to/stackless/include/python2.7
export BINDGEN_OUTPUT_PATH=./bindings.rs

# Generate bindings
cargo build
```
