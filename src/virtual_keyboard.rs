use std::fs::File;
use std::os::fd::AsFd;
use std::os::unix::fs::MetadataExt;
use std::time::SystemTime;

use wayland_client::protocol::wl_keyboard::KeymapFormat::XkbV1;
use wayland_client::protocol::wl_registry;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::{delegate_noop, Connection, Dispatch, EventQueue, QueueHandle};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;

const KEYMAP_FILE: &'static str = "./phosh/tests/data/keymap.txt";

#[derive(Default)]
struct State {
    seat: Option<WlSeat>,
    keyboard_manager: Option<ZwpVirtualKeyboardManagerV1>,
}

pub struct VirtualKeyboard {
    ts: SystemTime,
    state: State,
    kb: ZwpVirtualKeyboardV1,
    event_queue: EventQueue<State>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version: _,
        } = event
        {
            match &interface[..] {
                "wl_seat" => {
                    state.seat = Some(registry.bind::<WlSeat, _, _>(name, 1, qh, ()));
                }
                "zwp_virtual_keyboard_manager_v1" => {
                    state.keyboard_manager =
                        Some(registry.bind::<ZwpVirtualKeyboardManagerV1, _, _>(name, 1, qh, ()));
                }
                _ => {}
            }
        }
    }
}

impl VirtualKeyboard {
    pub fn new(conn: Connection) -> Self {
        let mut event_queue = conn.new_event_queue();
        let mut state: State = Default::default();
        let _ = conn.display().get_registry(&event_queue.handle(), ());
        event_queue.roundtrip(&mut state).unwrap();

        let kb = state
            .keyboard_manager
            .as_ref()
            .unwrap()
            .create_virtual_keyboard(state.seat.as_ref().unwrap(), &event_queue.handle(), ());

        let keymap = File::open(KEYMAP_FILE).unwrap();
        kb.keymap(
            XkbV1.into(),
            keymap.as_fd(),
            keymap.metadata().unwrap().size() as _,
        );

        Self {
            ts: SystemTime::now(),
            state: Default::default(),
            event_queue,
            kb,
        }
    }

    pub async fn keypress(&self, keycode: u32) {
        self.kb
            .key(self.ts.elapsed().unwrap().as_millis() as _, keycode, 1);

        self.event_queue.flush().unwrap();
        self.kb
            .key(self.ts.elapsed().unwrap().as_millis() as _, keycode, 0);
        self.event_queue.flush().unwrap();
    }
}

delegate_noop!(State: ignore WlSeat);
delegate_noop!(State: ignore ZwpVirtualKeyboardManagerV1);
delegate_noop!(State: ignore ZwpVirtualKeyboardV1);
