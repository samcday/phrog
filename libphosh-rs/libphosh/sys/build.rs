#[cfg(not(docsrs))]
mod build_support;
#[cfg(not(docsrs))]
mod native_source;

#[cfg(not(docsrs))]
use std::{collections::HashSet, env, path::PathBuf, process::Command};

#[cfg(docsrs)]
fn main() {}

#[cfg(not(docsrs))]
fn main() {
    println!("cargo:rerun-if-env-changed=PHOSH_SRC");
    println!("cargo:rerun-if-env-changed=PHROG_LIBPHOSH_BUILD_INTERNAL");
    match env::var("PHROG_LIBPHOSH_BUILD_INTERNAL").as_deref() {
        Ok("always") => build_bundled(),
        Err(env::VarError::NotPresent) | Ok("never") => {
            // The GTK3 and experimental GTK4 libraries share the same pkg-config
            // name and ABI version. A version check alone can select GTK3 here.
            let library = pkg_config::Config::new()
                // GTK can be in Requires.private. Inspect it without emitting
                // these static flags; system-deps below emits the dynamic link.
                .statik(true)
                .cargo_metadata(false)
                .probe("libphosh-0.45")
                .expect("an installed GTK4 libphosh is required; see docs/gtk4.md");
            assert!(
                library.libs.iter().any(|name| name == "gtk-4")
                    && !library.libs.iter().any(|name| name == "gtk-3"),
                "libphosh-0.45 resolves to GTK3; select a GTK4 libphosh with PKG_CONFIG_PATH, or use cargo vendored-phosh build (see docs/gtk4.md)"
            );
            system_deps::Config::new()
                .probe()
                .expect("system libphosh-0.45 is required (or set PHROG_LIBPHOSH_BUILD_INTERNAL=always)");
        }
        _ => panic!("PHROG_LIBPHOSH_BUILD_INTERNAL must be always or never; implicit fallback is not supported"),
    }
}

#[cfg(not(docsrs))]
fn run(command: &mut Command) {
    let status = command
        .status()
        .unwrap_or_else(|err| panic!("{command:?}: {err}"));
    assert!(status.success(), "{command:?} failed: {status}");
}

#[cfg(not(docsrs))]
fn build_bundled() {
    assert_eq!(env::var("HOST").unwrap(), env::var("TARGET").unwrap(),
        "bundled libphosh currently supports native builds only; use system libphosh for cross builds");
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let source = env::var_os("PHOSH_SRC")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest.join("../../../phosh"));
    let source = source
        .canonicalize()
        .expect("Phosh source tree not found; set PHOSH_SRC");
    assert!(
        source.join("meson.build").is_file(),
        "PHOSH_SRC must point to a Phosh source tree"
    );
    println!("cargo:rerun-if-changed={}", source.display());
    let mut configuration = format!("source={source:?}\n");
    for name in [
        "CC",
        "CFLAGS",
        "CPPFLAGS",
        "LDFLAGS",
        "PKG_CONFIG_PATH",
        "PKG_CONFIG_LIBDIR",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
        configuration.push_str(&format!("{name}={:?}\n", env::var_os(name)));
    }
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let staged_source = output.join("phosh-source");
    native_source::stage(&source, &staged_source).expect("failed to stage Phosh sources");
    println!("cargo:rerun-if-env-changed=PHROG_VENDOR_OFFLINE");
    match env::var("PHROG_VENDOR_OFFLINE").as_deref() {
        Ok("1") => (),
        Err(env::VarError::NotPresent) | Ok("0") => {
            run(Command::new("meson")
                .args(["subprojects", "download", "--sourcedir"])
                .arg(&staged_source)
                .args(["gvc", "libcall-ui"]));
        }
        _ => panic!("PHROG_VENDOR_OFFLINE must be 0 or 1"),
    }
    let build = output.join("phosh");
    let inputs = output.join("phosh-build-inputs");
    // Meson caches the source directory, compiler and environment flags at setup.
    // Recreate only our generated build tree when those inputs change.
    if build.exists() && std::fs::read_to_string(&inputs).ok().as_ref() != Some(&configuration) {
        std::fs::remove_dir_all(&build).expect("failed to reset the native build directory");
    }
    run(Command::new("meson")
        .args([
            "setup",
            "--reconfigure",
            "--backend=ninja",
            "--wrap-mode=nodownload",
            "--prefix=/usr",
            // Do not load plugins belonging to the system Phosh ABI.
            "--libdir=lib/phrog",
            "-Dbindings-lib=true",
            "-Dtests=false",
            "-Dphoc_tests=disabled",
            "-Dlockscreen-plugins=false",
            "-Dquick-setting-plugins=false",
        ])
        .arg(&build)
        .arg(&staged_source));
    run(Command::new("meson")
        .args(["compile", "-C"])
        .arg(&build)
        .args([
            "phosh-0.45:static_library",
            "phosh-tool",
            "glib-compile-schemas",
        ]));

    std::fs::write(inputs, configuration).expect("failed to record native build inputs");

    // Promote only libphosh's private dependencies. A recursive --static probe
    // also exposes private dependencies of shared libraries (e.g. PulseAudio's
    // libpulsecommon), which the executable must not link directly.
    let metadata_dir = build.join("meson-uninstalled");
    let metadata = std::fs::read_to_string(metadata_dir.join("libphosh-0.45-uninstalled.pc"))
        .expect("failed to read Meson's uninstalled libphosh metadata");
    let shared_metadata = build_support::embedding_metadata(&metadata);
    std::fs::write(metadata_dir.join("phrog-libphosh.pc"), shared_metadata)
        .expect("failed to write embedding metadata");

    // This build's metadata must win over any installed libphosh.
    let mut paths = vec![metadata_dir];
    if let Some(existing) = env::var_os("PKG_CONFIG_PATH") {
        paths.extend(env::split_paths(&existing));
    }
    env::set_var("PKG_CONFIG_PATH", env::join_paths(paths).unwrap());
    let lib = pkg_config::Config::new()
        .cargo_metadata(false)
        .print_system_libs(false)
        .statik(false)
        .probe("phrog-libphosh")
        .expect("failed to read bundled libphosh pkg-config metadata");
    // pkg-config can repeat transitive flags many times. Preserve their
    // order while avoiding GCC's per-argument/environment size limit.
    let mut seen_paths = HashSet::new();
    for path in &lib.link_paths {
        if !seen_paths.insert(path) {
            continue;
        }
        println!("cargo:rustc-link-search=native={}", path.display());
    }
    let mut seen_libs = HashSet::new();
    for name in &lib.libs {
        if !seen_libs.insert(name) {
            continue;
        }
        let bundled = lib
            .link_paths
            .iter()
            .any(|path| path.starts_with(&build) && path.join(format!("lib{name}.a")).is_file());
        assert!(
            name != "phosh-0.45" || bundled,
            "bundled libphosh archive not found"
        );
        println!(
            "cargo:rustc-link-lib={}{}",
            if bundled { "static=" } else { "" },
            name
        );
    }
    let mut seen_args = HashSet::new();
    for args in &lib.ld_args {
        if !seen_args.insert(args) {
            continue;
        }
        println!("cargo:rustc-link-arg=-Wl,{}", args.join(","));
    }
}
