pub mod common;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::ShellExt;
use libphosh::prelude::WallClockExt;
use libphosh::WallClock;
use std::sync::atomic::{AtomicBool, Ordering};

use phrog::shell::Shell;
use std::sync::Arc;
use std::time::Duration;
use gtk::gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use input_event_codes::*;
use common::*;
use wayland_client::Connection;
use phrog::lockscreen::Lockscreen;
use crate::common::virtual_keyboard::VirtualKeyboard;

#[test]
fn keypad_shuffle() {
    let mut test = test_init();

    let phosh_settings = Settings::new("sm.puri.phosh.lockscreen");
    phosh_settings.set_boolean("shuffle-keypad", false).unwrap();

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    glib::spawn_future_local(clone!(@weak shell => async move {
        let (mut vp, kb) = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(1500)).await;

        kb.keypress(KEY_SPACE!()).await;
        glib::timeout_future(Duration::from_millis(1000)).await;

        // Open top panel
        kb.modifiers(1 << 6);
        kb.keypress(KEY_M!()).await;
        kb.modifiers(0);
        glib::timeout_future(Duration::from_millis(500)).await;

        // click on keypad shuffle icon
        vp.click_on(&shell.keypad_shuffle_qs().unwrap().imp().info.clone()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        assert!(phosh_settings.boolean("shuffle-keypad"));

        // close top panel
        kb.keypress(KEY_ESC!()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        // click on center keypad button for dramatical flair
        let mut lockscreen = shell.lockscreen_manager().lockscreen().unwrap().downcast::<Lockscreen>().unwrap();
        let (keypad, _) = get_lockscreen_bits(&mut lockscreen);
        vp.click_on(&keypad.child_at(1, 1).unwrap()).await;

        glib::timeout_future(Duration::from_millis(1000)).await;

        vp.click_at((shell.usable_area().2 / 2) as _, 0).await;
        glib::timeout_future(Duration::from_millis(500)).await;
        vp.click_on(&shell.keypad_shuffle_qs().unwrap().imp().info.clone()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        assert!(!phosh_settings.boolean("shuffle-keypad"));

        kb.keypress(KEY_ESC!()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        vp.click_on(&keypad.child_at(1, 1).unwrap()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        gtk::main_quit();
    }));

    test.start("keypad-shuffle");

    assert!(test.ready_called.load(Ordering::Relaxed));
}
