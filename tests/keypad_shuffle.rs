mod common;

use gtk::glib;
use gtk::glib::{clone, g_warning};
use libphosh::prelude::ShellExt;
use libphosh::prelude::WallClockExt;
use libphosh::WallClock;
use std::sync::atomic::{AtomicBool, Ordering};

use greetd_ipc::codec::SyncCodec;
use phrog::shell::Shell;
use std::sync::Arc;
use std::time::Duration;
use gtk::gio::Settings;
use gtk::prelude::{ListBoxExt, SettingsExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use input_event_codes::{KEY_ESC, KEY_SPACE};
use common::*;
use wayland_client::Connection;

#[test]
fn keypad_shuffle() {
    let tmp = tempdir::TempDir::new("phrog-test-system-dbus").unwrap();
    let _system_dbus = dbus::system_dbus(tmp.path());

    let _nested_phoc = phrog::init(Some("phoc".into()));

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
        let (vp, kb) = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(2000)).await;

        kb.keypress(KEY_SPACE!()).await;
        glib::timeout_future(Duration::from_millis(2000)).await;

        // Open the top panel and click on keypad shuffle icon
        vp.click_at(10, 10).await;
        glib::timeout_future(Duration::from_millis(500)).await;
        vp.click_on(&unsafe { phrog::keypad_shuffle::INSTANCE.clone() }.unwrap().imp().info.clone()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        assert!(Settings::new("sm.puri.phosh.lockscreen").boolean("shuffle-keypad"));

        kb.keypress(KEY_ESC!()).await;
        glib::timeout_future(Duration::from_millis(500)).await;
        kb.keypress(KEY_SPACE!()).await;

        glib::timeout_future(Duration::from_millis(1000)).await;

        vp.click_at(10, 10).await;
        glib::timeout_future(Duration::from_millis(500)).await;
        vp.click_on(&unsafe { phrog::keypad_shuffle::INSTANCE.clone() }.unwrap().imp().info.clone()).await;
        glib::timeout_future(Duration::from_millis(500)).await;

        assert!(!Settings::new("sm.puri.phosh.lockscreen").boolean("shuffle-keypad"));
        kb.keypress(KEY_ESC!()).await;
        glib::timeout_future(Duration::from_millis(1000)).await;

        gtk::main_quit();
    }));

    let _recording = start_recording("keypad-shuffle");
    gtk::main();

    assert!(ready_called.load(Ordering::Relaxed));
}
