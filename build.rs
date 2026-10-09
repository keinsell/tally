use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Single source of truth for the app id. Overridable via `APP_ID=… cargo
// build`, matching `config::APP_ID` and the Flatpak manifest's build env.
// Falls back to the default so `cargo build` just works.
fn app_id() -> String {
    env::var("APP_ID").unwrap_or_else(|_| "com.keinsell.Tally".to_string())
}

// Every file staged verbatim: (repo path, path inside the staging dir).
// Adding a new asset (e.g. another CSS file) is one line here — the loop
// below handles `rerun-if-changed`, parent dirs and copying uniformly.
fn static_assets(app_id: &str) -> Vec<(PathBuf, PathBuf)> {
    let icon = format!("data/icons/hicolor/scalable/apps/{app_id}.svg");
    let symbolic = format!("data/icons/hicolor/symbolic/apps/{app_id}-symbolic.svg");
    vec![
        (PathBuf::from("data/style.css"), PathBuf::from("style.css")),
        (
            PathBuf::from(icon.clone()),
            PathBuf::from(format!("icons/hicolor/scalable/apps/{app_id}.svg")),
        ),
        (
            PathBuf::from(symbolic.clone()),
            PathBuf::from(format!("icons/hicolor/symbolic/apps/{app_id}-symbolic.svg")),
        ),
        (
            PathBuf::from("data/counter.gresource.xml"),
            PathBuf::from("counter.gresource.xml"),
        ),
    ]
}

fn stage_file(src: &Path, stage: &Path, rel: &Path) {
    println!("cargo:rerun-if-changed={}", src.display());
    let dest = stage.join(rel);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|e| panic!("could not create {}: {e}", parent.display()));
    }
    fs::copy(src, &dest).unwrap_or_else(|e| {
        panic!(
            "could not copy {} -> {}: {e}",
            src.display(),
            dest.display()
        )
    });
}

fn run(program: &str, args: &[&str]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("failed to run {program}: {e}"));
    assert!(status.success(), "{program} failed with {status}");
}

fn main() {
    if let Ok(resource_file) = env::var("COUNTER_RESOURCES_FILE") {
        println!("cargo:rustc-env=COUNTER_RESOURCES_FILE={resource_file}");
        return;
    }

    let app_id = app_id();
    println!("cargo:rerun-if-env-changed=APP_ID");
    println!("cargo:rerun-if-env-changed=COUNTER_RESOURCES_FILE");
    println!("cargo:rerun-if-changed=src/ui/counter.blp");
    println!("cargo:rerun-if-changed=data/counter.gresource.xml");

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

    // Everything else is a verbatim copy into the staging dir, which doubles
    // as `--sourcedir` for `glib-compile-resources`.
    for (src, rel) in static_assets(&app_id) {
        stage_file(&src, &stage, &rel);
    }

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
