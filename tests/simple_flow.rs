pub mod common;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::{LockscreenExt, ShellExt};
use libphosh::LockscreenPage;
use std::sync::atomic::Ordering;

use common::*;
use gtk::gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use phrog::lockscreen::Lockscreen;
use std::time::Duration;

#[test]
fn test_simple_flow() {
    let mut test = test_init(Some(TestOptions {
        last_user: Some("agx".into()),
        ..Default::default()
    }));

    let phosh_settings = Settings::new("sm.puri.phosh.lockscreen");
    phosh_settings.set_boolean("shuffle-keypad", false).unwrap();

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    test.start("simple-flow", glib::spawn_future_local(clone!(@weak shell => async move {
        let (mut vp, _) = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(2000)).await;

        let mut lockscreen = shell.lockscreen_manager().lockscreen().unwrap().downcast::<Lockscreen>().unwrap();
        let usp = lockscreen.imp().user_session_page.get().unwrap();

        // test_init mocked sessions such that GNOME should be the first option in the list, but
        // last-session should be empty and thus the selection should default to Phosh.
        assert_eq!(usp.session().id(), "phosh");

        // Move the mouse to first user row and click on it.
        vp.click_on(usp.imp().box_users.row_at_index(0).as_ref().unwrap()).await;

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
    })));

    assert!(test.logged_in.load(Ordering::Relaxed));
}
