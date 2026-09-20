#[path = "../native_source.rs"]
mod native_source;

use std::fs;

#[test]
fn refreshes_sources_and_reuses_only_matching_downloads() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("source");
    let staged = root.path().join("staged");
    fs::create_dir_all(source.join("subprojects")).unwrap();
    for name in ["gvc", "libcall-ui"] {
        fs::write(source.join(format!("subprojects/{name}.wrap")), "pin-1").unwrap();
    }
    fs::write(source.join("obsolete.c"), "old").unwrap();
    std::os::unix::fs::symlink("obsolete.c", source.join("link")).unwrap();
    native_source::stage(&source, &staged).unwrap();
    assert_eq!(
        fs::read_link(staged.join("link")).unwrap(),
        std::path::Path::new("obsolete.c")
    );

    // Simulate downloads in the private source tree, then edit the checkout.
    for name in ["gvc", "libcall-ui"] {
        fs::create_dir_all(staged.join(format!("subprojects/{name}"))).unwrap();
        fs::write(
            staged.join(format!("subprojects/{name}/download")),
            "cached",
        )
        .unwrap();
    }
    fs::remove_file(source.join("obsolete.c")).unwrap();
    fs::write(source.join("new.c"), "new").unwrap();
    fs::write(source.join("subprojects/gvc.wrap"), "pin-2").unwrap();
    native_source::stage(&source, &staged).unwrap();
    assert!(!staged.join("obsolete.c").exists());
    assert_eq!(fs::read_to_string(staged.join("new.c")).unwrap(), "new");
    assert!(!staged.join("subprojects/gvc").exists());
    assert!(staged.join("subprojects/libcall-ui/download").exists());
    assert!(!source.join("subprojects/libcall-ui").exists());

    // A killed build may leave the old tree in its backup location and an
    // incomplete replacement. The next offline build must retain its downloads.
    fs::rename(&staged, staged.with_extension("previous")).unwrap();
    fs::create_dir_all(staged.with_extension("next")).unwrap();
    fs::write(staged.with_extension("next").join("partial"), "incomplete").unwrap();
    native_source::stage(&source, &staged).unwrap();
    assert!(staged.join("subprojects/libcall-ui/download").exists());
    assert!(!staged.join("partial").exists());
    assert!(!staged.with_extension("previous").exists());
}
