use std::{io::ErrorKind, process::Command};

fn main() {
    let build_dir = std::env::current_dir().unwrap().join("build");
    match std::fs::create_dir(&build_dir) {
        Ok(_) => {}
        Err(err) if err.kind() == ErrorKind::AlreadyExists => {}
        err => err.unwrap(),
    }

    if cfg!(not(feature = "build-assets")) {
        println!("cargo:warning=skipping web assets build");
        println!("cargo:rerun-if-changed=");
        return;
    }

    let status = Command::new("npm").args(["ci"]).status().unwrap();
    assert!(status.success());

    let status = Command::new("npm").args(["run", "build"]).status().unwrap();
    assert!(status.success());

    println!("cargo:rerun-if-changed=package-lock.json");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=svelte.config.js");
    println!("cargo:rerun-if-changed=tsconfig.json");
    println!("cargo:rerun-if-changed=vite.config.ts");

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=static");
}
