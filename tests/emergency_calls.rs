pub mod common;

use gtk::glib;

use common::*;
use glib::clone;
use gtk::gio::prelude::{ActionGroupExt, SettingsExt};
use gtk::gio::Settings;
use libphosh::prelude::{LockscreenExt, ShellExt};
use libphosh::LockscreenPage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::{OwnedValue, Type, Value};

#[derive(Deserialize, Serialize, Type, Value, PartialEq, Debug)]
struct EmergencyContact {
    id: String,
    name: String,
    source: i32,
    properties: HashMap<String, OwnedValue>,
}

struct EmergencyCallsFixture {
    pub dialled_numbers: Arc<Mutex<Vec<String>>>,
}

#[zbus::interface(name = "org.gnome.Calls.EmergencyCalls")]
impl EmergencyCallsFixture {
    #[zbus(signal)]
    async fn emergency_numbers_changed(
        signal_emitter: &SignalEmitter<'_>,
        message: &str,
    ) -> zbus::Result<()>;

    async fn get_emergency_contacts(&self) -> Vec<EmergencyContact> {
        // Not used currently, maybe we expand test later to check this.
        vec![EmergencyContact {
            source: 0,
            name: "Test".to_string(),
            id: "foo".to_string(),
            properties: HashMap::new(),
        }]
    }

    async fn call_emergency_contact(&mut self, id: String) {
        self.dialled_numbers.lock().unwrap().push(id);
    }
}

#[test]
fn test_emergency_calls() {
    let mut test = test_init(None);

    let e_c_settings = Settings::new("sm.puri.phosh.emergency-calls");
    e_c_settings.set_boolean("enabled", true).unwrap();

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    let dbus_session = test.session_dbus_conn.to_owned();
    test.start(
        "emergency-calls",
        glib::spawn_future_local(clone!(
            #[weak]
            shell,
            async move {
                let (mut _vp, _) = ready_rx.recv().await.unwrap();
                glib::timeout_future(Duration::from_millis(1500)).await;

                let dialled_numbers: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

                dbus_session
                    .object_server()
                    .at("/org/gnome/Calls", zbus::fdo::ObjectManager {})
                    .await
                    .unwrap();
                dbus_session
                    .object_server()
                    .at(
                        "/org/gnome/Calls",
                        EmergencyCallsFixture {
                            dialled_numbers: Arc::clone(&dialled_numbers),
                        },
                    )
                    .await
                    .expect("failed to serve /org/gnome/Calls");
                dbus_session
                    .request_name("org.gnome.Calls")
                    .await
                    .expect("failed to request name");

                // In GTK3 days this test drilled through the PhoshEmergencyMenu widget tree to drive the
                // dialpad with a virtual pointer. In GTK4 phosh, the menu is a PhoshSystemModalDialog
                // (a PhoshLayerSurface subclass), which is no longer reachable via toplevel enumeration.
                // Until libphosh grows a way to grab the dialog, exercise the D-Bus wiring + menu
                // open/close paths and leave the dialpad-driving adventure for another day.
                shell.activate_action("power.toggle-menu", None);
                glib::timeout_future(Duration::from_millis(1000)).await;
                shell.activate_action("emergency.toggle-menu", None);
                glib::timeout_future(Duration::from_millis(1000)).await;
                shell.activate_action("power.toggle-menu", None);
                glib::timeout_future(Duration::from_millis(1000)).await;

                assert!(dialled_numbers.lock().unwrap().is_empty());

                shell
                    .lockscreen_manager()
                    .lockscreen()
                    .unwrap()
                    .set_page(LockscreenPage::Info);
                glib::timeout_future(Duration::from_millis(2000)).await;

                fade_quit();
            }
        )),
    );
}
