pub mod common;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use gtk::{glib, Bin, Button, Grid, Window};

use common::*;
use gtk::gio::Settings;
use gtk::prelude::*;
use std::time::Duration;
use glib::clone;
use libphosh::LockscreenPage;
use libphosh::prelude::{LockscreenExt, ShellExt};
use serde::{Deserialize, Serialize};
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
    async fn emergency_numbers_changed(signal_ctxt: &zbus::SignalContext<'_>) -> zbus::Result<()>;

    async fn get_emergency_contacts(&self) -> Vec<EmergencyContact> {
        // Not used currently, maybe we expand test later to check this.
        vec![EmergencyContact{
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


struct CallFixture {}

#[zbus::interface(interface = "org.gnome.Calls.Call")]
impl CallFixture {
    fn accept(&self) {}
    fn hangup(&self) {}
    fn send_dtmf(&self, _tone: &str) {}

    #[zbus(property)]
    fn can_dtmf(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn display_name(&self) -> String {
        "Sam".into()
    }

    #[zbus(property)]
    fn encrypted(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn id(&self) -> String {
        "sammyboi".into()
    }

    #[zbus(property)]
    fn image_path(&self) -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/samcday.jpeg")
            .display()
            .to_string()
    }

    #[zbus(property)]
    fn inbound(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn protocol(&self) -> String {
        String::new()
    }

    #[zbus(property)]
    fn state(&self) -> u32 {
        3
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
    test.start("emergency-calls", glib::spawn_future_local(clone!(@weak shell => async move {
        let (mut vp, _) = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(1500)).await;

        let dialled_numbers: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        dbus_session.object_server().at("/org/gnome/Calls", zbus::fdo::ObjectManager{}).await.unwrap();
        dbus_session
            .object_server()
            .at("/org/gnome/Calls", EmergencyCallsFixture{ dialled_numbers: Arc::clone(&dialled_numbers) })
            .await
            .expect("failed to serve /org/gnome/Calls");
        dbus_session
            .request_name("org.gnome.Calls")
            .await
            .expect("failed to request name");

        shell.activate_action("power.toggle-menu", None);
        glib::timeout_future(Duration::from_millis(1000)).await;
        shell.activate_action("emergency.toggle-menu", None);
        glib::timeout_future(Duration::from_millis(1000)).await;

        // Extreme hacks follow. Avert your eyes, ye weak of stomach.
        // The last toplevel should be the emergency menu.
        let emergency_window = Window::list_toplevels().iter().last().cloned().unwrap();
        assert_eq!("PhoshEmergencyMenu", emergency_window.type_().name());

        // Now traverse the widget hiearchy to get the keypad grid.
        let swipe_away_bin = emergency_window.downcast::<Bin>().unwrap().child().unwrap();
        let clamp = swipe_away_bin.downcast::<Bin>().unwrap().child().unwrap();
        let _box = clamp.downcast::<Bin>().unwrap().child().unwrap();
        let carousel = _box.downcast::<gtk::Box>().unwrap().children().get(1).cloned().unwrap();
        let inner = carousel.downcast::<Bin>().unwrap().child().unwrap();
        let _box = inner.downcast::<gtk::Container>().unwrap().children().first().cloned().unwrap();
        let dialpad = _box.downcast::<gtk::Box>().unwrap().children().get(1).cloned().unwrap();
        let clamp = dialpad.downcast::<gtk::Container>().unwrap().children().first().cloned().unwrap();
        let _box = clamp.downcast::<Bin>().unwrap().child().unwrap();
        let keypad = _box.clone().downcast::<gtk::Box>().unwrap().children().get(1).cloned().unwrap();
        let grid = keypad.downcast::<Bin>().unwrap().child().unwrap().downcast::<Grid>().unwrap();
        let button_parent = _box.downcast::<gtk::Box>().unwrap().children().get(2).cloned().unwrap();
        let button = button_parent.downcast::<gtk::Box>().unwrap().children().first().cloned().unwrap().downcast::<Button>().unwrap();
        // Egregious code heresy ends here. You can open your eyes, now.

        // Punch in the new easy to remember emergency number to summon good-looking drivers in
        // nice ambulances with fast response times.
        for digit in [0,1,1,8,9,9,9,8,8,1, 9,9,9, 1,1,9, 7,2,5] {
            vp.click_on(&keypad_digit(&grid, digit)).await;
            glib::timeout_future(Duration::from_millis(10)).await;
        }
        glib::timeout_future(Duration::from_millis(1000)).await;
        vp.click_on(&keypad_digit(&grid, 3)).await;

        // Click the call button.
        glib::timeout_future(Duration::from_millis(500)).await;
        vp.click_on(&button).await;

        glib::timeout_future(Duration::from_millis(500)).await;
        assert_eq!(vec!["01189998819991197253"], *dialled_numbers.lock().unwrap());

        shell.activate_action("power.toggle-menu", None);

        // Ring ring ...
        glib::timeout_future(Duration::from_millis(500)).await;
        dbus_session.object_server().at("/org/gnome/Calls/Call/1", CallFixture{}).await.unwrap();

        shell.lockscreen_manager().lockscreen().unwrap().set_page(LockscreenPage::Info);
        glib::timeout_future(Duration::from_millis(2000)).await;

        fade_quit();
    })));
}
