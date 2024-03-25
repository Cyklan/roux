// build.rs

use std::env;
use std::fs;
use std::path::Path;

// const EMBEDDED_CSS = include_bytes!(env!("OUT_DIR"))

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // copy the public dir to the out dir

    // build tailwind
    let status = std::process::Command::new("npx")
        .args(&[
            "tailwindcss",
            "build",
            "-i",
            "src/index.css",
            "-o",
            Path::new(&out_dir).join("./index.css").to_str().unwrap(),
            "--minify",
        ])
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build tailwind");
    }

    vite_build();

    for entry in fs::read_dir("./templates").unwrap() {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    for entry in fs::read_dir("./public").unwrap() {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn vite_build() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let cp_status = std::process::Command::new("cp")
        .args(&[
            "-r",
            "./public",
            Path::new(&out_dir).join("public").to_str().unwrap(),
        ])
        .status()
        .unwrap();
    if !cp_status.success() {
        panic!("Failed to copy public dir");
    }

    fs::write(Path::new(&out_dir).join("index.html"), HTML).unwrap();
    // run vite build command
    let status = std::process::Command::new("npx")
        .args(&["vite", "build", Path::new(&out_dir).to_str().unwrap()])
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build vite");
    }

    // get file name from out_dir/assets/
    // rename to index.js
    let assets_dir = Path::new(&out_dir).join("dist/assets");
    let mut file_name = String::new();
    for entry in fs::read_dir(assets_dir).unwrap() {
        let entry = entry.unwrap();
        file_name = entry.file_name().into_string().unwrap();
        break;
    }

    let js_file = Path::new(std::env::var_os("OUT_DIR").unwrap().to_str().unwrap())
        .join("dist/assets")
        .join(file_name.clone());
    let new_js_file = Path::new(std::env::var_os("OUT_DIR").unwrap().to_str().unwrap())
        .join("dist/assets")
        .join("index.js");
    let mv_status = std::process::Command::new("mv")
        .args(&[js_file.to_str().unwrap(), new_js_file.to_str().unwrap()])
        .status()
        .unwrap();

    if !mv_status.success() {
        panic!("Failed to rename js file");
    }

    std::process::Command::new("rm")
        .args(&["-r", Path::new(&out_dir).join("public").to_str().unwrap()])
        .status()
        .unwrap();
}

const HTML: &'static str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
  <script type="module" src="./public/index.ts"></script>
</head>
<body>
  
</body>
</html>"#;
