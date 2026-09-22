pub mod common;

use common::virtual_pointer::wait_for_click_target;
use common::*;
use gtk::glib::{self, clone, translate::*};
use gtk::prelude::*;
use gtk::{gio::Settings, Widget};
use libphosh::prelude::{LockscreenExt, ShellExt};
use libphosh::LockscreenPage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
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
    dialled_numbers: Arc<Mutex<Vec<String>>>,
}

#[zbus::interface(name = "org.gnome.Calls.EmergencyCalls")]
impl EmergencyCallsFixture {
    async fn get_emergency_contacts(&self) -> Vec<EmergencyContact> {
        vec![EmergencyContact {
            source: 0,
            name: "Test contact".into(),
            id: "test-contact".into(),
            properties: HashMap::new(),
        }]
    }

    async fn call_emergency_contact(&mut self, id: String) {
        self.dialled_numbers.lock().unwrap().push(id);
    }
}

#[derive(Default)]
struct CallRequests {
    accepted: usize,
    hung_up: usize,
}

struct CallFixture {
    state: u32,
    requests: Arc<Mutex<CallRequests>>,
}

#[zbus::interface(name = "org.gnome.Calls.Call")]
impl CallFixture {
    async fn accept(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        self.requests.lock().unwrap().accepted += 1;
        self.state = 1; // ACTIVE, matching the Calls D-Bus API.
        self.state_changed(&emitter).await.unwrap();
    }

    async fn hangup(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        self.requests.lock().unwrap().hung_up += 1;
        self.state = 7; // DISCONNECTED
        self.state_changed(&emitter).await.unwrap();
    }

    fn send_dtmf(&self, _tone: &str) {}

    #[zbus(property)]
    fn can_dtmf(&self) -> bool {
        false
    }
    #[zbus(property)]
    fn display_name(&self) -> &str {
        "Test caller"
    }
    #[zbus(property)]
    fn encrypted(&self) -> bool {
        false
    }
    #[zbus(property)]
    fn id(&self) -> &str {
        "test-caller"
    }
    #[zbus(property)]
    fn image_path(&self) -> &str {
        ""
    }
    #[zbus(property)]
    fn inbound(&self) -> bool {
        true
    }
    #[zbus(property)]
    fn protocol(&self) -> &str {
        ""
    }
    #[zbus(property)]
    fn state(&self) -> u32 {
        self.state
    }
}

async fn wait_for<T>(description: &str, mut check: impl FnMut() -> Option<T>) -> T {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(value) = check() {
            return value;
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {description}"
        );
        glib::timeout_future(Duration::from_millis(10)).await;
    }
}

fn find_widget(root: &Widget, matches: &impl Fn(&Widget) -> bool) -> Option<Widget> {
    if matches(root) {
        return Some(root.clone());
    }
    let mut child = root.first_child();
    while let Some(widget) = child {
        if let Some(found) = find_widget(&widget, matches) {
            return Some(found);
        }
        child = widget.next_sibling();
    }
    None
}

fn named(root: &Widget, name: &str) -> Widget {
    find_widget(root, &|widget| {
        widget.buildable_id().as_deref() == Some(name)
    })
    .unwrap_or_else(|| panic!("missing {name} below {}", root.type_().name()))
}

// GtkPlain layer surfaces are not GtkWindows and aren't in Window::list_toplevels.
// Watch their public configure signal instead of reading private manager structs.
struct SurfaceWatch {
    signal: u32,
    hook: std::ffi::c_ulong,
    widget: Box<glib::WeakRef<Widget>>,
}

impl SurfaceWatch {
    fn new() -> Self {
        unsafe extern "C" fn configured(
            _hint: *mut glib::gobject_ffi::GSignalInvocationHint,
            n_values: u32,
            values: *const glib::gobject_ffi::GValue,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            if n_values > 0 {
                let object = glib::gobject_ffi::g_value_get_object(values);
                let object: Borrowed<glib::Object> = from_glib_borrow(object);
                if matches!(
                    object.type_().name(),
                    "PhoshEmergencyMenu" | "PhoshPowerMenu"
                ) {
                    let weak = &*(data as *const glib::WeakRef<Widget>);
                    weak.set(object.downcast_ref::<Widget>());
                }
            }
            glib::ffi::GTRUE
        }

        let widget = Box::new(glib::WeakRef::new());
        // The boxed hook data stays at a fixed address until Drop removes the hook.
        // Signal values are borrowed only for the callback; the stored widget is weak.
        let (signal, hook) = unsafe {
            let type_ = glib::Type::from_name("PhoshLayerSurface").unwrap();
            let signal =
                glib::gobject_ffi::g_signal_lookup(c"configured".as_ptr(), type_.into_glib());
            assert_ne!(signal, 0);
            let hook = glib::gobject_ffi::g_signal_add_emission_hook(
                signal,
                0,
                Some(configured),
                widget.as_ref() as *const glib::WeakRef<Widget> as glib::ffi::gpointer,
                None,
            );
            assert_ne!(hook, 0);
            (signal, hook)
        };
        Self {
            signal,
            hook,
            widget,
        }
    }
}

impl Drop for SurfaceWatch {
    fn drop(&mut self) {
        unsafe {
            glib::gobject_ffi::g_signal_remove_emission_hook(self.signal, self.hook);
        }
    }
}

#[test]
fn test_emergency_calls() {
    // Requires Phoc. test_init creates private session/system buses: every Calls
    // request below reaches this fixture, never a modem or the host Calls service.
    let mut test = test_init(None);
    Settings::new("sm.puri.phosh.emergency-calls")
        .set_boolean("enabled", true)
        .unwrap();

    let ready_rx = test.ready_rx.clone();
    let shell = test.shell.clone();
    let dbus_session = test.session_dbus_conn.clone();
    test.start(
        "emergency-calls",
        glib::spawn_future_local(clone!(
            #[weak]
            shell,
            async move {
                let (mut vp, _) = ready_rx.recv().await.unwrap();
                let dialled_numbers = Arc::new(Mutex::new(Vec::new()));
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
                            dialled_numbers: dialled_numbers.clone(),
                        },
                    )
                    .await
                    .unwrap();
                dbus_session.request_name("org.gnome.Calls").await.unwrap();
                wait_for("emergency action to become enabled", || {
                    shell
                        .is_action_enabled("emergency.toggle-menu")
                        .then_some(())
                })
                .await;

                let watch = SurfaceWatch::new();
                shell.activate_action("power.toggle-menu", None);
                let power_menu = wait_for("power menu configure", || {
                    watch
                        .widget
                        .upgrade()
                        .filter(|w| w.type_().name() == "PhoshPowerMenu")
                })
                .await;
                let emergency_button = named(&power_menu, "btn_emergency_call");
                wait_for("emergency button allocation", || {
                    (emergency_button.width() > 0).then_some(())
                })
                .await;
                wait_for_click_target(&emergency_button).await;
                vp.click_on(&emergency_button).await;
                let menu = wait_for("emergency menu configure", || {
                    watch
                        .widget
                        .upgrade()
                        .filter(|w| w.type_().name() == "PhoshEmergencyMenu")
                })
                .await;
                assert!(
                    !power_menu.is_visible(),
                    "opening the dialler must close the power menu"
                );
                let dialpad = find_widget(&menu, &|w| w.type_().name() == "CuiDialpad").unwrap();
                let entry = named(&dialpad, "keypad_entry")
                    .downcast::<gtk::Entry>()
                    .unwrap();
                let keypad = named(&dialpad, "keypad");
                let grid = find_widget(&keypad, &|w| w.is::<gtk::Grid>())
                    .unwrap()
                    .downcast::<gtk::Grid>()
                    .unwrap();
                let dial = named(&dialpad, "dial");
                wait_for("dialpad allocation", || {
                    (grid.width() > 0 && dial.width() > 0).then_some(())
                })
                .await;

                // Exercise actual pointer input, entry contents, and the D-Bus request.
                // This fictional number is served exclusively by our private fixture.
                let number = "01189998819991197253";
                for (index, digit) in number.char_indices() {
                    vp.click_on(&keypad_digit(&grid, digit.to_digit(10).unwrap() as i32))
                        .await;
                    wait_for("dialled digit to appear", || {
                        (entry.text() == number[..=index]).then_some(())
                    })
                    .await;
                }
                assert!(
                    dialled_numbers.lock().unwrap().is_empty(),
                    "typing alone must not place a call"
                );
                wait_for_click_target(&dial).await;
                vp.click_on(&dial).await;
                wait_for("emergency request", || {
                    (!dialled_numbers.lock().unwrap().is_empty()).then_some(())
                })
                .await;
                assert_eq!(*dialled_numbers.lock().unwrap(), vec![number]);

                wait_for("emergency menu to close", || {
                    (!menu.is_visible()).then_some(())
                })
                .await;
                // A real incoming state is 5; the former fixture used 3 (DIALING).
                // #100 separately tracks pinning the info page during active calls.
                let lockscreen = shell.lockscreen_manager().lockscreen().unwrap();
                lockscreen.set_page(LockscreenPage::Info);
                let requests = Arc::new(Mutex::new(CallRequests::default()));
                let call_path = "/org/gnome/Calls/Call/1";
                dbus_session
                    .object_server()
                    .at(
                        call_path,
                        CallFixture {
                            state: 5,
                            requests: requests.clone(),
                        },
                    )
                    .await
                    .unwrap();
                let call_display = named(lockscreen.upcast_ref(), "call_display");
                let navigation = named(lockscreen.upcast_ref(), "deck");
                wait_for("incoming call page", || {
                    navigation
                        .property::<Option<glib::Object>>("visible-page")
                        .filter(|p| {
                            p.property::<Option<String>>("tag").as_deref()
                                == Some("box_call_display")
                        })
                })
                .await;
                wait_for("incoming call model", || {
                    call_display.property::<Option<glib::Object>>("call")
                })
                .await;
                let answer = named(&call_display, "answer");
                wait_for("answer button", || {
                    (answer.is_visible() && answer.is_sensitive() && answer.width() > 0)
                        .then_some(())
                })
                .await;
                wait_for_click_target(&answer).await;
                vp.click_on(&answer).await;
                wait_for("Accept request", || {
                    (requests.lock().unwrap().accepted == 1).then_some(())
                })
                .await;
                let hang_up = named(&call_display, "hang_up");
                wait_for("active call controls", || {
                    (!answer.is_visible() && hang_up.is_visible()).then_some(())
                })
                .await;
                wait_for_click_target(&hang_up).await;
                vp.click_on(&hang_up).await;
                wait_for("Hangup request", || {
                    (requests.lock().unwrap().hung_up == 1).then_some(())
                })
                .await;
                dbus_session
                    .object_server()
                    .remove::<CallFixture, _>(call_path)
                    .await
                    .unwrap();
                wait_for("call page removal", || {
                    navigation
                        .property::<Option<glib::Object>>("visible-page")
                        .filter(|p| {
                            p.property::<Option<String>>("tag").as_deref()
                                != Some("box_call_display")
                        })
                })
                .await;
                assert_eq!(
                    *dialled_numbers.lock().unwrap(),
                    vec![number],
                    "incoming calls must not dial again"
                );
                fade_quit();
            }
        )),
    );
}
