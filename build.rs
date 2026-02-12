use std::path::PathBuf;

fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/phrog.gresources.xml",
        "phrog.gresource",
    );

    let schema_path =
        PathBuf::from(std::env::var("HOME").unwrap()).join(".local/share/glib-2.0/schemas");
    std::fs::create_dir_all(&schema_path).expect("failed to create schema dir");

    let phrog_gschema = PathBuf::from("data/mobi.phosh.phrog.gschema.xml");
    let dest_path = schema_path.join(phrog_gschema.file_name().unwrap());
    std::fs::copy(&phrog_gschema, dest_path).expect("failed to copy phrog schema file");
    println!("cargo::rerun-if-changed={}", phrog_gschema.display());

    std::process::Command::new("glib-compile-schemas")
        .arg(&schema_path)
        .spawn()
        .and_then(|mut v| v.wait())
        .expect("failed to run glib-compile-schemas");
}
