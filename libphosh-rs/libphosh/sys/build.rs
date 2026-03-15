use std::path::{Path, PathBuf};

#[cfg(not(docsrs))]
use std::process;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    if std::env::var("CARGO_FEATURE_STATIC").as_deref() == Ok("1") {
        let build_internal_mode =
            std::env::var("PHROG_LIBPHOSH_BUILD_INTERNAL").unwrap_or_else(|_| "always".to_string());
        std::env::set_var(
            "SYSTEM_DEPS_LIBPHOSH_0_45_BUILD_INTERNAL",
            &build_internal_mode,
        );
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();

    if let Err(s) = system_deps::Config::new()
        .add_build_internal("libphosh-0.45", move |name, version| {
            println!("cargo:rustc-link-lib=static=phosh-{version}");

            let mut path = std::env::var("PHOSH_SRC").ok();

            if path.is_none() {
                let phosh_path = Path::new(&std::env::current_dir().unwrap())
                    .join("../../../phosh")
                    .canonicalize()
                    .ok();
                if let Some(phosh_path) = phosh_path {
                    if phosh_path.exists() && phosh_path.join("meson.build").exists() {
                        path = Some(phosh_path.display().to_string());
                    }
                }
            }

            let path = path.expect("$PHOSH_SRC was not set and no ../../../phosh tree was found");
            let build_dir = Path::new(&out_dir).join("_phosh-build");

            println!("cargo:rerun-if-changed={path}");

            let status = std::process::Command::new("meson")
                .arg("setup")
                .arg("--reconfigure")
                .arg("-Dbindings-lib=true")
                .arg("-Dtests=false")
                .arg("-Dphoc_tests=disabled")
                .arg("-Dquick-setting-plugins=false")
                .arg(&path)
                .arg(&build_dir)
                .status()
                .expect("meson setup failed");
            assert!(status.success());

            let status = std::process::Command::new("meson")
                .arg("install")
                .arg("-C")
                .arg(&build_dir)
                .arg("--destdir=install")
                .status()
                .expect("meson install failed");
            assert!(status.success());

            println!("cargo:rustc-link-search=native={}/src", build_dir.display());

            let schema_path =
                PathBuf::from(std::env::var("HOME").unwrap()).join(".local/share/glib-2.0/schemas");
            std::fs::create_dir_all(&schema_path).expect("failed to create schema dir");

            copy_schema_file(
                &build_dir,
                &schema_path,
                &["mobi.phosh.shell.enums.xml", "sm.puri.phosh.enums.xml"],
                "phosh enums",
            );
            copy_schema_file(
                &build_dir,
                &schema_path,
                &["mobi.phosh.shell.gschema.xml", "sm.puri.phosh.gschema.xml"],
                "phosh schema",
            );

            std::process::Command::new("glib-compile-schemas")
                .arg(&schema_path)
                .spawn()
                .and_then(|mut child| child.wait())
                .expect("failed to run glib-compile-schemas");

            system_deps::Library::from_internal_pkg_config(
                format!("{}/meson-private", build_dir.display()),
                name,
                version,
            )
        })
        .probe()
    {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}

#[cfg(not(docsrs))]
fn copy_schema_file(build_dir: &Path, schema_path: &Path, names: &[&str], kind: &str) {
    let source = names
        .iter()
        .map(|name| build_dir.join("data").join(name))
        .find(|path| path.exists())
        .unwrap_or_else(|| {
            panic!(
                "failed to find {} in {} (tried: {})",
                kind,
                build_dir.join("data").display(),
                names.join(", ")
            )
        });

    let dest_path = schema_path.join(source.file_name().unwrap());
    std::fs::write(
        &dest_path,
        std::fs::read(&source).unwrap_or_else(|_| panic!("failed to read {kind}")),
    )
    .unwrap_or_else(|_| panic!("failed to write {kind}"));

    println!("cargo:rerun-if-changed={}", source.display());
}
