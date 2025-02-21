pub mod common;

use std::fs::{File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::ShellExt;

use common::*;
use std::time::Duration;

#[test]
fn test_first_run() {
    let first_run_dir = tempfile::tempdir().unwrap();

    let mut touch = first_run_dir.path().to_owned();
    touch.push("firstran");

    let mut script_path = first_run_dir.path().to_owned();
    script_path.push("firstrun");

    {
        let mut script = File::create(&script_path).unwrap();
        script.set_permissions(Permissions::from_mode(0o755)).unwrap();
        script.write_all(format!(
            "#!/bin/bash
            set -uexo pipefail
            foot bash -c 'touch {touch}; echo This is a first-run wizard; while [[ -f {touch} ]]; do sleep 0.5; done; exit'",
            touch = touch.display()
        ).as_bytes()).unwrap();
    }

    let mut test = test_init(Some(TestOptions {
        first_run: Some(script_path.display().to_string()),
        ..Default::default()
    }));

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    test.start("first-run", glib::spawn_future_local(clone!(@weak shell => async move {
        let _ = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(2000)).await;

        // The first-run script should have started.
        assert!(touch.exists());
        glib::timeout_future(Duration::from_millis(2000)).await;

        // Delete the marker, the script will exit.
        std::fs::remove_file(touch).unwrap();
        glib::timeout_future(Duration::from_millis(2000)).await;

        // The first-run script should have completed, and the shell should now be locked.
        assert!(shell.is_locked());

        gtk::main_quit();
    })));
}
