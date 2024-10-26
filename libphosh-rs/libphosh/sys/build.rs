use std::fmt::format;
use std::path::{Path, PathBuf};
#[cfg(not(docsrs))]
use std::process;

mod build_version;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    if let Ok(val) = std::env::var("CARGO_FEATURE_STATIC") {
        if val == "1" {
            std::env::set_var("SYSTEM_DEPS_LIBPHOSH_0_42_BUILD_INTERNAL", "always");
        }
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();

    if let Err(s) = system_deps::Config::new()
        .add_build_internal("libphosh-0.42", move |name, version| {
            // We're going to build and statically link phosh.
            println!("cargo:rustc-link-lib=static={}", format!("phosh-{}", version));

            // $PHOSH_SRC controls where to find a Phosh Meson project root.
            let mut path = std::env::var("PHOSH_SRC").ok();

            // If there's a ../../../phosh directory containing a meson.build, use that dir.
            // This supports the use-case that an embedding application has a vendored copy of
            // libphosh-rs alongside phosh.
            if path.is_none() {
                let phosh_path = Path::new(&std::env::current_dir().unwrap()).join("../../../phosh").canonicalize().unwrap();
                if phosh_path.exists() && phosh_path.join("meson.build").exists() {
                    path = Some(phosh_path.display().to_string());
                }
            }

            // TODO: if a path has not been provided, use our subproject?

            if path.is_none() {
                panic!("$PHOSH_SRC was not set");
            }

            let build_dir = Path::new(&out_dir).join("_phosh-build");
            let path = path.unwrap();
            println!("building Phosh from {} in build-dir: {:?}", &path, &build_dir);

            println!("cargo::rerun-if-changed={}", &path);

            let status = std::process::Command::new("meson")
                .arg("setup")
                .arg("--reconfigure")
                .arg("-Dbindings-lib=true")
                .arg(&path)
                .arg(&build_dir)
                .status().expect("meson setup failed");
            assert!(status.success());

            let status = std::process::Command::new("meson")
                .arg("install")
                .arg("-C")
                .arg(&build_dir)
                .arg("--destdir=install")
                .status().expect("meson install failed");
            assert!(status.success());

            // Set search path to private build.
            println!("cargo:rustc-link-search=native={}/src", build_dir.display());

            // Write phosh gschema to home dir now. Sorta pollute-y, sure, but also very convenient.
            let schema_path = PathBuf::from(std::env::var("HOME").unwrap()).join(".local/share/glib-2.0/schemas");
            std::fs::create_dir_all(&schema_path).expect("failed to create schema dir");

            let phosh_enums = build_dir.join("data/sm.puri.phosh.enums.xml");
            let dest_path = schema_path.join(phosh_enums.file_name().unwrap());
            std::fs::write(&dest_path, std::fs::read(phosh_enums).expect("failed to read phosh enums"))
                .expect("failed to write phosh enums");
            println!("cargo::rerun-if-changed={}", dest_path.display());

            let phosh_gschema = build_dir.join("data/sm.puri.phosh.gschema.xml");
            let dest_path = schema_path.join(phosh_gschema.file_name().unwrap());
            std::fs::write(&dest_path, std::fs::read(phosh_gschema).expect("failed to read schema"))
                .expect("failed to write phosh schema");
            println!("cargo::rerun-if-changed={}", dest_path.display());

            std::process::Command::new("glib-compile-schemas")
                .arg(&schema_path)
                .spawn()
                .and_then(|mut v| v.wait())
                .expect("failed to run glib-compile-schemas");

            system_deps::Library::from_internal_pkg_config(
                    format!("{}/meson-private", build_dir.display()),
                    name,
                    version)
        })
        .probe()
    {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}
