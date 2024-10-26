mod common;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::{LockscreenExt, ShellExt};
use libphosh::prelude::WallClockExt;
use libphosh::{LockscreenPage, WallClock};
use std::sync::atomic::{AtomicBool, Ordering};

use phrog::shell::Shell;
use std::sync::Arc;
use std::time::Duration;
use gtk::gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use common::*;
use wayland_client::Connection;
use phrog::lockscreen::Lockscreen;

#[test]
fn test_simple_flow() {
    let tmp = tempfile::tempdir().unwrap();
    let _nested_phoc = phrog::init(Some("phoc".into()));
    let _system_dbus = dbus::system_dbus(tmp.path());

    let settings = Settings::new("sm.puri.phosh.lockscreen");
    settings.set_boolean("shuffle-keypad", false).unwrap();

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
        ready_tx.send_blocking(vp).expect("notify ready failed");
    }));

    glib::spawn_future_local(clone!(@weak shell => async move {
        let mut vp = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(500)).await;
        // Move the mouse to first user row and click on it.
        let mut lockscreen = shell.lockscreen_manager().lockscreen().unwrap().downcast::<Lockscreen>().unwrap();

        let usp = lockscreen.imp().user_session_page.get().unwrap();

        vp.click_on(usp.imp().box_users.selected_row().as_ref().unwrap()).await;

        // wait for keypad page to slide in
        glib::timeout_future(Duration::from_millis(500)).await;

        assert_eq!(lockscreen.page(), LockscreenPage::Unlock);

        let (keypad, submit_btn) = get_lockscreen_bits(&mut lockscreen);

        vp.click_on(&keypad.child_at(1, 3).unwrap()).await; // 0
        vp.click_on(&keypad.child_at(0, 1).unwrap()).await; // 4
        vp.click_on(&keypad.child_at(1, 1).unwrap()).await; // 5
        vp.click_on(&keypad.child_at(0, 0).unwrap()).await; // 1

        vp.click_on(&submit_btn).await;
        glib::timeout_future(Duration::from_millis(50)).await;
    }));

    let _recording = start_recording("simple-flow");
    gtk::main();

    assert!(ready_called.load(Ordering::Relaxed));
    assert!(logged_in.load(Ordering::Relaxed));
}