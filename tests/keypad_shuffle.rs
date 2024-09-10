#![cfg(feature="keypad-shuffle")]
mod common;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::ShellExt;
use libphosh::prelude::WallClockExt;
use libphosh::WallClock;
use std::sync::atomic::{AtomicBool, Ordering};

use greetd_ipc::codec::SyncCodec;
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
    let tmp = tempdir::TempDir::new("phrog-test-system-dbus").unwrap();
    let _nested_phoc = phrog::init(Some("phoc".into()));
    let _system_dbus = dbus::system_dbus(tmp.path());

    let _conn = async_global_executor::block_on(async move {
        dbus::run_accounts_fixture().await.unwrap()
    });

    let logged_in = Arc::new(AtomicBool::new(false));
    fake_greetd(&logged_in);

    let wall_clock = WallClock::new();
    wall_clock.set_default();
    let shell = Shell::new();
    shell.set_default();
    shell.set_locked(true);

    let settings = Settings::new("sm.puri.phosh.lockscreen");
    settings.set_boolean("shuffle-keypad", false).unwrap();
    let ready_called = Arc::new(AtomicBool::new(false));
    let (ready_tx, ready_rx) = async_channel::bounded(1);
    shell.connect_ready(clone!(@strong ready_called => move |shell| {
        ready_called.store(true, Ordering::Relaxed);

        let (_, _, width, height) = shell.usable_area();
        let vp = VirtualPointer::new(Connection::connect_to_env().unwrap(), width as _, height as _);
        let kb = VirtualKeyboard::new(Connection::connect_to_env().unwrap());
        ready_tx.send_blocking((vp, kb)).expect("notify ready failed");
    }));

    glib::spawn_future_local(clone!(@weak shell => async move {
        let (mut vp, kb) = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(500)).await;

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

        assert!(settings.boolean("shuffle-keypad"));

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

        assert!(!settings.boolean("shuffle-keypad"));

        kb.keypress(KEY_ESC!()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        vp.click_on(&keypad.child_at(1, 1).unwrap()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        gtk::main_quit();
    }));

    let _recording = start_recording("keypad-shuffle");
    gtk::main();

    assert!(ready_called.load(Ordering::Relaxed));
}
