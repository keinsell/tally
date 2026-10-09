use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/ui/counter.blp");
    println!("cargo:rerun-if-changed=data/style.css");
    println!("cargo:rerun-if-changed=data/counter.gresource.xml");
    println!(
        "cargo:rerun-if-changed=data/icons/hicolor/scalable/apps/dev.keinsell.Counter.svg"
    );
    println!(
        "cargo:rerun-if-changed=data/icons/hicolor/symbolic/apps/dev.keinsell.Counter-symbolic.svg"
    );
    println!("cargo:rerun-if-env-changed=COUNTER_RESOURCES_FILE");

    // When building through meson, the GResource bundle is already compiled
    if let Ok(resource_file) = env::var("COUNTER_RESOURCES_FILE") {
        println!("cargo:rustc-env=COUNTER_RESOURCES_FILE={resource_file}");
        return;
    }

    // Standalone fallback: compile the blueprint and bundle it ourselves
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let stage = Path::new(&out_dir).join("stage");
    fs::create_dir_all(&stage).expect("could not create staging directory");

    let ui = stage.join("counter.ui");
    run(
        "blueprint-compiler",
        &[
            "compile",
            "src/ui/counter.blp",
            "--output",
            ui.to_str().unwrap(),
        ],
    );

    fs::copy("data/style.css", stage.join("style.css")).expect("could not copy style.css");
    let icons_stage = stage.join("icons/hicolor/scalable/apps");
    fs::create_dir_all(&icons_stage).expect("could not create icon staging directory");
    fs::copy(
        "data/icons/hicolor/scalable/apps/dev.keinsell.Counter.svg",
        icons_stage.join("dev.keinsell.Counter.svg"),
    )
    .expect("could not copy app icon");
    let symbolic_stage = stage.join("icons/hicolor/symbolic/apps");
    fs::create_dir_all(&symbolic_stage).expect("could not create symbolic icon staging directory");
    fs::copy(
        "data/icons/hicolor/symbolic/apps/dev.keinsell.Counter-symbolic.svg",
        symbolic_stage.join("dev.keinsell.Counter-symbolic.svg"),
    )
    .expect("could not copy symbolic app icon");
    fs::copy(
        "data/counter.gresource.xml",
        stage.join("counter.gresource.xml"),
    )
    .expect("could not copy counter.gresource.xml");

    let bundle = Path::new(&out_dir).join("counter.gresource");
    run(
        "glib-compile-resources",
        &[
            "--target",
            bundle.to_str().unwrap(),
            "--sourcedir",
            stage.to_str().unwrap(),
            stage.join("counter.gresource.xml").to_str().unwrap(),
        ],
    );

    println!(
        "cargo:rustc-env=COUNTER_RESOURCES_FILE={}",
        bundle.display()
    );
}

fn run(program: &str, args: &[&str]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("failed to run {program}: {e}"));
    assert!(status.success(), "{program} failed");
}
