use std::process::Command;

fn main() {
    if cfg!(feature = "no-build") {
        println!("cargo:warning=skipping web assets build");
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
