use std::path::PathBuf;
use std::process::Child;

fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/phrog.gresources.xml",
        "phrog.gresource",
    );

    let schema_file = "mobi.phosh.phrog.gschema.xml";
    let resources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources");
    let schema_path = homedir::my_home().unwrap().unwrap().join(".local/share/glib-2.0/schemas");
    std::fs::create_dir_all(&schema_path).expect("failed to create schema dir");

    std::fs::write(&schema_path.join(schema_file),
                   std::fs::read(resources_dir.join(schema_file))
                       .expect("failed to read schema file")
    ).expect("failed to write schema file");

    std::process::Command::new("glib-compile-schemas")
        .arg(&schema_path)
        .spawn()
        .and_then(|mut v| v.wait())
        .expect("failed to run glib-compile-schemas");
}
