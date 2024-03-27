// build.rs

use std::env;
use std::fs;
use std::path::Path;

// const EMBEDDED_CSS = include_bytes!(env!("OUT_DIR"))

fn main() {
    vite_build();

    for entry in fs::read_dir("./templates").unwrap() {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    for entry in fs::read_dir("./scripts").unwrap() {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn vite_build() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // run vite build command
    let status = std::process::Command::new("npx")
        .args(&[
            "vite",
            "build",
            "--outDir",
            Path::new(&out_dir).to_str().unwrap(),
        ])
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build vite");
    }
}
