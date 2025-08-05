//! Skirk - A port scanner written in Rust.
//!
//! This script acts as a binder to connect C code to Rust

use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let object_path = Path::new(&manifest_dir).join("bin/skirk.so");

    if !object_path.exists() {
        panic!("\x1b[34m\
A core component of building skirk was not found. Aborting.\n\
If this installation was from cargo, please report this issue to the repository at https://github.com/aitzazisstuffed/skirk/issues.\
\x1b[0m");
    }

    println!("cargo:rustc-link-arg={}", object_path.display());
    println!("cargo:rerun-if-changed=bin/skirk.so");
}
