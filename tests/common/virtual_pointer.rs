use gtk::{glib, Widget};
use std::time::{Duration, Instant, SystemTime};
use gtk::prelude::*;
use wayland_client::protocol::wl_pointer::ButtonState;
use wayland_client::protocol::wl_registry;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::{delegate_noop, Connection, Dispatch, EventQueue, QueueHandle};
use wayland_protocols_wlr::virtual_pointer::v1::client::zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1;
use wayland_protocols_wlr::virtual_pointer::v1::client::zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1;

#[derive(Default)]
struct State {
    seat: Option<WlSeat>,
    pointer_manager: Option<ZwlrVirtualPointerManagerV1>,
}

pub struct VirtualPointer {
    ts: SystemTime,
    state: State,
    event_queue: EventQueue<State>,
    ptr: ZwlrVirtualPointerV1,
    width: u32,
    height: u32,
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
                "zwlr_virtual_pointer_manager_v1" => {
                    state.pointer_manager =
                        Some(registry.bind::<ZwlrVirtualPointerManagerV1, _, _>(name, 1, qh, ()));
                }
                _ => {}
            }
        }
    }
}

impl VirtualPointer {
    pub fn new(conn: Connection, width: u32, height: u32) -> Self {
        let mut event_queue = conn.new_event_queue();
        let mut state: State = Default::default();
        let _ = conn.display().get_registry(&event_queue.handle(), ());
        let ts = SystemTime::now();
        event_queue.roundtrip(&mut state).unwrap();

        let ptr = state
            .pointer_manager
            .as_ref()
            .unwrap()
            .create_virtual_pointer(state.seat.as_ref(), &event_queue.handle(), ());

        Self {
            ts,
            event_queue: conn.new_event_queue(),
            state,
            ptr,
            width,
            height,
        }
    }

    pub async fn click_on(&self, widget: &impl IsA<Widget>) {
        let window = widget.window().unwrap();
        let (_, mut x, y) = window.origin();
        x += window.width() / 2;
        self.move_to(x as _, y as _).await;
    }

    pub async fn move_to(&self, x: u32, y: u32) {
        self.ptr.motion_absolute(
            self.ts.elapsed().unwrap().as_millis() as _,
            x,
            y + 75,
            self.width,
            self.height,
        );
        self.ptr.frame();
        self.event_queue.flush().unwrap();

        let mut cur_y = y as f64 + 75.0;
        while cur_y > y as f64 {
            cur_y -= 0.5;
            self.ptr
                .motion(self.ts.elapsed().unwrap().as_millis() as _, 0.0, -0.5);
            self.ptr.frame();
            self.event_queue.flush().unwrap();
            glib::timeout_future(Duration::from_millis(1)).await;
        }

        glib::timeout_future(Duration::from_millis(250)).await;
        self.click();
    }

    pub fn click(&self) {
        // I found this magic 272 constant in moverest/wl-kbptr sources.
        // TODO: demystify this voodoo number
        self.ptr.button(
            self.ts.elapsed().unwrap().as_millis() as _,
            272,
            ButtonState::Pressed,
        );
        self.ptr.frame();
        self.ptr.button(
            self.ts.elapsed().unwrap().as_millis() as _,
            272,
            ButtonState::Released,
        );
        self.ptr.frame();
        self.event_queue.flush().unwrap();
    }
}

delegate_noop!(State: ignore WlSeat);
delegate_noop!(State: ignore ZwlrVirtualPointerManagerV1);
delegate_noop!(State: ignore ZwlrVirtualPointerV1);