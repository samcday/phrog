pub mod common;

use gtk::glib;
use libphosh::prelude::ShellExt;

use common::*;
use gtk::prelude::*;
use std::time::Duration;

#[test]
fn test_accent_colours() {
    let mut test = test_init(None);

    test.shell.set_locked(true);

    let ready_rx = test.ready_rx.clone();
    let if_settings = test.if_settings.clone();
    test.start(
        "accent-colours",
        glib::spawn_future_local(async move {
            let (_, _) = ready_rx.recv().await.unwrap();
            glib::timeout_future(Duration::from_millis(1500)).await;

            for color in ["red", "slate", "pink", "teal", "red", "purple", "blue"] {
                if_settings.set_string("accent-color", color).unwrap();
                glib::timeout_future(Duration::from_millis(500)).await;
            }

            fade_quit();
        }),
    );
}
