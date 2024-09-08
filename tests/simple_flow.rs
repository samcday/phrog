mod common;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::ShellExt;
use libphosh::prelude::WallClockExt;
use libphosh::WallClock;
use std::sync::atomic::{AtomicBool, Ordering};

use greetd_ipc::codec::SyncCodec;
use input_event_codes::*;
use phrog::shell::Shell;
use std::sync::Arc;
use std::time::Duration;
use gtk::prelude::ListBoxExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use common::*;
use wayland_client::Connection;

#[test]
fn test_simple_flow() {
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
        // Move the mouse to first user row and click on it.
        let lockscreen = unsafe { phrog::lockscreen::INSTANCE.as_mut().unwrap() };
        let usp = lockscreen.imp().user_session_page.get().unwrap();

        vp.click_on(usp.imp().box_users.selected_row().as_ref().unwrap()).await;

        // wait for keypad page to slide in
        glib::timeout_future(Duration::from_millis(500)).await;

        // type password (uh, literally)
        let keys = [
            KEY_P!(),
            KEY_A!(),
            KEY_S!(),
            KEY_S!(),
            KEY_W!(),
            KEY_O!(),
            KEY_R!(),
            KEY_D!(),
            KEY_ENTER!(),
        ];
        for key in keys {
            kb.keypress(key).await;
            glib::timeout_future(Duration::from_millis(100)).await;
        }
    }));

    let _recording = start_recording("simple-flow");
    gtk::main();

    assert!(ready_called.load(Ordering::Relaxed));
    assert!(logged_in.load(Ordering::Relaxed));
}
