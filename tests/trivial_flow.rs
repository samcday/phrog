pub mod common;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::{LockscreenExt, ShellExt};
use libphosh::LockscreenPage;
use std::sync::atomic::Ordering;

use common::*;
use gtk::gio::Settings;
use gtk::prelude::*;
use phrog::lockscreen::Lockscreen;
use std::time::Duration;

#[test]
fn test_trivial_flow() {
    let mut test = test_init(Some(TestOptions { num_users: Some(1) }));

    let phosh_settings = Settings::new("sm.puri.phosh.lockscreen");
    phosh_settings.set_boolean("shuffle-keypad", false).unwrap();

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    glib::spawn_future_local(clone!(@weak shell => async move {
        let (mut vp, _) = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(2000)).await;

        let mut lockscreen = shell.lockscreen_manager().lockscreen().unwrap().downcast::<Lockscreen>().unwrap();

        assert!(lockscreen.user_session_page().ready());
        assert_eq!(lockscreen.page(), LockscreenPage::Unlock);

        let (keypad, submit_btn) = get_lockscreen_bits(&mut lockscreen);

        vp.click_on(&keypad.child_at(1, 3).unwrap()).await; // 0
        vp.click_on(&keypad.child_at(0, 1).unwrap()).await; // 4
        vp.click_on(&keypad.child_at(1, 1).unwrap()).await; // 5
        vp.click_on(&keypad.child_at(0, 0).unwrap()).await; // 1

        vp.click_on(&submit_btn).await;
        glib::timeout_future(Duration::from_millis(50)).await;
    }));

    test.start("trivial-flow");

    assert!(test.logged_in.load(Ordering::Relaxed));
}
