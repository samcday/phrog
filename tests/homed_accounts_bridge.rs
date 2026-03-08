pub mod common;

use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use libhandy::prelude::ActionRowExt;
use libphosh::prelude::ShellExt;
use phrog::lockscreen::Lockscreen;
use std::time::Duration;

use common::dbus::{AccountsFixtureOptions, UserFixture};
use common::*;

#[test]
fn test_homed_users_are_cached_and_system_users_hidden() {
    let mut test = test_init(Some(TestOptions {
        accounts_fixture: Some(AccountsFixtureOptions {
            users: vec![
                UserFixture::new("Guido", "agx", "guido.png", true, false),
                UserFixture::new("Greeter", "greetd", "phoshi.png", true, true),
                UserFixture::new("Homed User", "homed", "samcday.jpeg", true, false),
            ],
            cached_users: Some(vec!["agx".into(), "greetd".into()]),
            homed_users: vec!["homed".into()],
            ..Default::default()
        }),
        last_user: Some("agx".into()),
        fake_greetd: Some(true),
        ..Default::default()
    }));

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    test.start(
        "homed-accounts-bridge",
        glib::spawn_future_local(clone!(@weak shell => async move {
            let (_vp, _) = ready_rx.recv().await.unwrap();
            glib::timeout_future(Duration::from_millis(2500)).await;

            let lockscreen = shell
                .lockscreen_manager()
                .lockscreen()
                .unwrap()
                .downcast::<Lockscreen>()
                .unwrap();
            let usp = lockscreen.imp().user_session_page.get().unwrap();

            let rows = usp.imp().box_users.children();
            let usernames = rows
                .iter()
                .filter_map(|row| {
                    row.downcast_ref::<libhandy::ActionRow>()
                        .and_then(|row| row.subtitle())
                        .map(|s| s.to_string())
                })
                .collect::<Vec<_>>();

            assert!(usernames.contains(&"agx".to_string()));
            assert!(usernames.contains(&"homed".to_string()));
            assert!(!usernames.contains(&"greetd".to_string()));

            fade_quit();
        })),
    );
}
