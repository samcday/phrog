use std::path::{Path, PathBuf};
#[cfg(not(docsrs))]
use std::process;

mod build_version;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    if let Err(s) = system_deps::Config::new()
        .add_build_internal("libphosh_0", move |_, version| {
            // We're going to build and statically link phosh.
            println!("cargo:rustc-link-lib=static=phosh");

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

            // TODO: ideally setup the meson build dir under cargo out/ but weird shit happens
            // with enums at the moment.
            // let build_dir = Path::new(&out_dir).join("_phosh-build");
            let path = path.unwrap();
            println!("building Phosh from {}", &path);
            let build_dir = Path::new(&path).join("_libphoshrs-build");

            let status = std::process::Command::new("meson")
                .arg("setup")
                .arg("--reconfigure")
                .arg("-Dbindings-lib=true")
                .arg(&path)
                .arg(&build_dir)
                .status().expect("meson setup failed");
            assert!(status.success());

            let status = std::process::Command::new("meson")
                .arg("compile")
                .arg("-C")
                .arg(&build_dir)
                .status().expect("meson compile failed");
            assert!(status.success());

            let build_dir = build_dir.display().to_string();
            // Set search path to private build.
            println!("cargo:rustc-link-search=native={}/src", build_dir);

            system_deps::Library::from_internal_pkg_config(format!("{}/meson-private", build_dir), "libphosh-0", version)
        })
        .probe()
    {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}
