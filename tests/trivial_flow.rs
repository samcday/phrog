pub mod common;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::{LockscreenExt, ShellExt};
use libphosh::LockscreenPage;
use std::sync::atomic::Ordering;

use common::*;
use gtk::prelude::*;
use phrog::lockscreen::Lockscreen;
use phrog::session_object::SessionObject;
use std::time::{Duration, Instant};

async fn wait_for(description: &str, check: impl Fn() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !check() {
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {description}"
        );
        glib::timeout_future(Duration::from_millis(10)).await;
    }
}

#[test]
fn test_trivial_flow() {
    let mut test = test_init(Some(TestOptions {
        num_users: Some(1),
        sessions: Some(vec![SessionObject::new("phosh", "Phosh", "", "", "")]),
        ..Default::default()
    }));

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    test.start(
        "trivial-flow",
        glib::spawn_future_local(clone!(
            #[weak]
            shell,
            async move {
                let (mut vp, _) = ready_rx.recv().await.unwrap();

                let lockscreen = shell
                    .lockscreen_manager()
                    .lockscreen()
                    .unwrap()
                    .downcast::<Lockscreen>()
                    .unwrap();

                wait_for("single-user keypad", || {
                    lockscreen.user_session_page().ready()
                        && lockscreen.page() == LockscreenPage::Unlock
                        && lockscreen.is_sensitive()
                        && get_unlock_status_label(&lockscreen).text() == "Password:"
                })
                .await;

                // A configure event during a transition must not reset the selected
                // page. The GTK4 carousel still reports Info until it has moved.
                // Keep the existing greetd conversation while exercising navigation.
                let notifications = lockscreen.freeze_notify();
                lockscreen.set_page(LockscreenPage::Info);
                wait_for("info page", || lockscreen.page() == LockscreenPage::Info).await;
                lockscreen.set_page(LockscreenPage::Unlock);
                lockscreen.emit_by_name::<()>("configured", &[]);
                wait_for("keypad after reconfigure", || {
                    lockscreen.page() == LockscreenPage::Unlock && lockscreen.is_sensitive()
                })
                .await;

                drop(notifications);

                let (keypad, submit_btn) = get_lockscreen_bits(&lockscreen);

                common::virtual_pointer::wait_for_click_target(&keypad.clone().upcast()).await;
                vp.click_on(&keypad.child_at(1, 3).unwrap()).await; // 0
                vp.click_on(&keypad.child_at(0, 1).unwrap()).await; // 4
                vp.click_on(&keypad.child_at(1, 1).unwrap()).await; // 5
                vp.click_on(&keypad.child_at(0, 0).unwrap()).await; // 1

                vp.click_on(&submit_btn).await;
                glib::timeout_future(Duration::from_millis(50)).await;
            }
        )),
    );

    assert!(test.logged_in.load(Ordering::Relaxed));
}
