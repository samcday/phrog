pub mod common;

use gtk::glib;
use libphosh::prelude::ShellExt;

use common::*;
use gtk::gio::Settings;
use gtk::prelude::*;
use std::time::Duration;

#[test]
fn test_accent_colours() {
    let mut test = test_init();

    let phosh_settings = Settings::new("sm.puri.phosh.lockscreen");
    phosh_settings.set_boolean("shuffle-keypad", false).unwrap();
    test.shell.set_locked(true);

    let ready_rx = test.ready_rx.clone();
    let if_settings = test.if_settings.clone();
    glib::spawn_future_local(async move {
        let (_, _) = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(1500)).await;

        if_settings.set_string("accent-color", "red").unwrap();
        glib::timeout_future(Duration::from_millis(500)).await;
        if_settings.set_string("accent-color", "slate").unwrap();
        glib::timeout_future(Duration::from_millis(500)).await;
        if_settings.set_string("accent-color", "pink").unwrap();
        glib::timeout_future(Duration::from_millis(500)).await;
        if_settings.set_string("accent-color", "blue").unwrap();
        glib::timeout_future(Duration::from_millis(500)).await;

        fade_quit();
    });

    test.start("accent-colours");
}
