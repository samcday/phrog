#[path = "../build_support.rs"]
mod build_support;

use std::{fs, process::Command};

// Exercise pkg-config itself: embedding needs the root library's private
// dependencies, but must not expose private dependencies of shared libraries.
#[test]
fn embeds_only_the_root_library() {
    let dir = tempfile::tempdir().unwrap();
    let root = "prefix=/example\nName: root\nDescription: embedded library\nVersion: 1\nLibs: -L${prefix}/lib -lroot\nLibs.private: -lroot-helper\nRequires.private: shared >= 1\nCflags: -I${prefix}/include\n";
    fs::write(dir.path().join("root.pc"), root).unwrap();
    fs::write(
        dir.path().join("embedded.pc"),
        build_support::embedding_metadata(root),
    )
    .unwrap();
    fs::write(dir.path().join("shared.pc"), "Name: shared\nDescription: shared dependency\nVersion: 1\nLibs: -lpublic\nLibs.private: -lprivate\nRequires.private: internal\n").unwrap();
    fs::write(
        dir.path().join("internal.pc"),
        "Name: internal\nDescription: private dependency\nVersion: 1\nLibs: -linternal\n",
    )
    .unwrap();

    let query = |args: &[&str]| {
        let output = Command::new("pkg-config")
            .args(args)
            .env("PKG_CONFIG_PATH", dir.path())
            .env("PKG_CONFIG_LIBDIR", dir.path())
            .env_remove("PKG_CONFIG_SYSROOT_DIR")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    let recursive = query(&["--libs", "--static", "root"]);
    assert!(recursive.split_whitespace().any(|flag| flag == "-lprivate"));
    assert!(recursive
        .split_whitespace()
        .any(|flag| flag == "-linternal"));

    let embedded = query(&["--libs", "embedded"]);
    let flags: Vec<_> = embedded.split_whitespace().collect();
    for required in ["-L/example/lib", "-lroot", "-lroot-helper", "-lpublic"] {
        assert!(flags.contains(&required), "missing {required}: {embedded}");
    }
    for private in ["-lprivate", "-linternal"] {
        assert!(
            !flags.contains(&private),
            "unexpected {private}: {embedded}"
        );
    }
    assert_eq!(
        query(&["--cflags", "embedded"]).trim(),
        "-I/example/include"
    );
}
