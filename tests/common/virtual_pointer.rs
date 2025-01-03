use gtk::prelude::*;
use gtk::{glib, Widget};
use std::time::{Duration, SystemTime};
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
    event_queue: EventQueue<State>,
    ptr: ZwlrVirtualPointerV1,
    x: u32,
    y: u32,
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

        let x = width / 2;
        let y = height / 2;
        ptr.motion_absolute(ts.elapsed().unwrap().as_millis() as _, x, y, width, height);
        ptr.frame();
        event_queue.flush().unwrap();

        Self {
            ts,
            event_queue: conn.new_event_queue(),
            ptr,
            x,
            y,
        }
    }

    pub async fn click_on(&mut self, widget: &impl IsA<Widget>) {
        let (mut x, y) = widget
            .translate_coordinates(&widget.toplevel().unwrap(), 0, 0)
            .unwrap();
        x += widget.allocated_width() / 2;
        self.click_at(x as _, y as _).await;
    }

    pub async fn click_at(&mut self, x: u32, y: u32) {
        while self.x != x || self.y != y {
            fn step(distance: f64) -> f64 {
                let abs = distance.abs();
                let step = distance * (1. / abs);
                (if abs == 0. {
                    0.
                } else if abs > 10. {
                    step * 3.
                } else {
                    step
                })
                .floor()
            }
            let distance_x = x as f64 - self.x as f64;
            let distance_y = y as f64 - self.y as f64;

            let step_x = step(distance_x);
            let step_y = step(distance_y);
            self.ptr
                .motion(self.ts.elapsed().unwrap().as_millis() as _, step_x, step_y);
            self.ptr.frame();
            self.event_queue.flush().unwrap();
            self.x = (self.x as f64 + step_x) as _;
            self.y = (self.y as f64 + step_y) as _;
            glib::timeout_future(Duration::from_millis(1)).await;
        }

        glib::timeout_future(Duration::from_millis(150)).await;
        self.click().await;
    }

    pub async fn click(&self) {
        // I found this magic 272 constant in moverest/wl-kbptr sources.
        // TODO: demystify this voodoo number
        self.ptr.button(
            self.ts.elapsed().unwrap().as_millis() as _,
            272,
            ButtonState::Pressed,
        );
        self.ptr.frame();
        self.event_queue.flush().unwrap();
        glib::timeout_future(Duration::from_millis(50)).await;
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
