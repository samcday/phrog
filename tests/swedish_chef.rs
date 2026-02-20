pub mod common;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::{LockscreenExt, ShellExt};

use common::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use phrog::lockscreen::Lockscreen;
use std::process::Command;
use std::time::Duration;

const PHROG_CHEF_PO: &str = r#"
msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\n"
"Language: se_BORK\n"

msgid "Password:"
msgstr "Bork bork bork:"

msgid "Please wait..."
msgstr "Bork bork bork..."

msgid "Login failed, please try again"
msgstr "Bork bork bork, pleeze-a try bork bork"

msgid "Incorrect password (hint: it's '0')"
msgstr "Bork bork bork (hint: eet's '0')"

msgid "Session"
msgstr "Bork:"
"#;

const PHOSH_CHEF_PO: &str = r#"
msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\n"
"Language: se_BORK\n"

msgid "Slide up to unlock"
msgstr "Sleede-a oopp tu oonlerk"
"#;

#[test]
fn test_swedish_chef_locale() {
    let locale_dir = tempfile::tempdir().unwrap();
    let locale_path = locale_dir.path().join("se_BORK/LC_MESSAGES");
    std::fs::create_dir_all(&locale_path).unwrap();

    let po_path = locale_dir.path().join(format!("{}.po", phrog::TEXT_DOMAIN));
    std::fs::write(&po_path, PHROG_CHEF_PO).unwrap();

    let mo_path = locale_path.join(format!("{}.mo", phrog::TEXT_DOMAIN));
    let msgfmt_status = Command::new("msgfmt")
        .args(["-o", mo_path.to_str().unwrap(), po_path.to_str().unwrap()])
        .status()
        .unwrap();
    assert!(msgfmt_status.success());

    let phosh_po_path = locale_dir.path().join("phosh.po");
    std::fs::write(&phosh_po_path, PHOSH_CHEF_PO).unwrap();

    let phosh_mo_path = locale_path.join("phosh.mo");
    let phosh_msgfmt_status = Command::new("msgfmt")
        .args([
            "-o",
            phosh_mo_path.to_str().unwrap(),
            phosh_po_path.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(phosh_msgfmt_status.success());

    let Some(locale) = detect_available_locale() else {
        eprintln!("skipping swedish chef test: no non-C locale available");
        return;
    };

    let mut test = test_init(Some(TestOptions {
        last_user: Some("agx".into()),
        fake_greetd: Some(true),
        locale_dir: Some(locale_dir.path().to_path_buf()),
        locale: Some(locale),
        language: Some("se_BORK".into()),
        ..Default::default()
    }));

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    test.start(
        "swedish-chef",
        glib::spawn_future_local(clone!(@weak shell => async move {
            let (mut vp, _) = ready_rx.recv().await.unwrap();
            glib::timeout_future(Duration::from_millis(2000)).await;

            let mut lockscreen = shell
                .lockscreen_manager()
                .lockscreen()
                .unwrap()
                .downcast::<Lockscreen>()
                .unwrap();
            let usp = lockscreen.imp().user_session_page.get().unwrap();

            vp.click_on(usp.imp().box_users.row_at_index(0).as_ref().unwrap())
                .await;
            wait_for_unlock_status(
                &lockscreen,
                "Bork bork bork:",
                Duration::from_millis(1500),
            )
            .await;
            assert_eq!(lockscreen.page(), libphosh::LockscreenPage::Unlock);
            glib::timeout_future(Duration::from_millis(1500)).await;

            let (keypad, submit_btn) = get_lockscreen_bits(&mut lockscreen);
            vp.click_on(&keypad_digit(&keypad, 1)).await;
            vp.click_on(&submit_btn).await;

            wait_for_unlock_status(
                &lockscreen,
                "Bork bork bork, pleeze-a try bork bork",
                Duration::from_millis(1500),
            )
            .await;
            assert_eq!(lockscreen.page(), libphosh::LockscreenPage::Unlock);
            glib::timeout_future(Duration::from_millis(1500)).await;

            lockscreen.set_page(libphosh::LockscreenPage::Info);
            glib::timeout_future(Duration::from_millis(2000)).await;

            fade_quit();
        })),
    );
}

fn lockscreen_unlock_status(lockscreen: &Lockscreen) -> String {
    let carousel = lockscreen
        .child()
        .unwrap()
        .downcast::<libhandy::Carousel>()
        .unwrap();
    let keypad_page = carousel
        .children()
        .get(2)
        .unwrap()
        .clone()
        .downcast::<gtk::Box>()
        .unwrap();
    let label = keypad_page
        .children()
        .first()
        .unwrap()
        .clone()
        .downcast::<gtk::Label>()
        .unwrap();
    label.label().to_string()
}

async fn wait_for_unlock_status(lockscreen: &Lockscreen, expected: &str, timeout: Duration) {
    let started = std::time::Instant::now();
    while started.elapsed() < timeout {
        if lockscreen_unlock_status(lockscreen) == expected {
            return;
        }
        glib::timeout_future(Duration::from_millis(20)).await;
    }

    let current = lockscreen_unlock_status(lockscreen);
    panic!(
        "timed out waiting for unlock status {:?}, got {:?}",
        expected, current
    );
}

fn detect_available_locale() -> Option<String> {
    let output = match Command::new("locale").arg("-a").output() {
        Ok(output) => output,
        Err(err) => {
            eprintln!(
                "skipping swedish chef test: unable to run `locale -a`: {err}"
            );
            return None;
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "skipping swedish chef test: `locale -a` failed: {}",
            stderr.trim()
        );
        return None;
    }

    let locales = String::from_utf8_lossy(&output.stdout);

    for candidate in ["en_US.UTF-8", "en_US.utf8"] {
        if locales.lines().any(|line| line == candidate) {
            return Some(candidate.to_string());
        }
    }

    locales
        .lines()
        .map(str::trim)
        .find(|locale| {
            !locale.is_empty()
                && !matches!(locale.to_ascii_uppercase().as_str(), "C" | "POSIX")
                && !locale.to_ascii_uppercase().starts_with("C.")
        })
        .map(ToString::to_string)
}
