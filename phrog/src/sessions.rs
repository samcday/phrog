use crate::session_object::SessionObject;
use glob::glob;
use gtk::gio::DesktopAppInfo;
use gtk::glib::g_warning;
use gtk::prelude::*;
use std::collections::HashMap;

pub fn sessions() -> Vec<SessionObject> {
    let mut sessions = HashMap::new();
    session_list(
        "/usr/share/wayland-sessions/*.desktop",
        "wayland",
        &mut sessions,
    );
    session_list("/usr/share/xsessions/*.desktop", "x11", &mut sessions);
    sessions.values().cloned().collect()
}

fn session_list(path: &str, session_type: &str, sessions: &mut HashMap<String, SessionObject>) {
    for f in match glob(path) {
        Err(e) => {
            g_warning!("sessions", "couldn't check sessions in {}: {}", path, e);
            return;
        }
        Ok(iter) => iter,
    }
    .flatten()
    {
        let info = if let Some(info) = DesktopAppInfo::from_filename(&f) {
            info
        } else {
            g_warning!("session", "Unable to parse session file {:?}", f);
            continue;
        };
        // Use the Name= entry as key for de-duplication.
        let name = info.name();
        if sessions.contains_key(name.as_str()) {
            continue;
        }
        sessions.insert(
            name.to_string(),
            SessionObject::new(
                &name,
                session_type,
                &info
                    .commandline()
                    .map_or(String::new(), |v| v.to_string_lossy().to_string()),
                &info
                    .string("DesktopNames")
                    .and_then(|v| Some(v.trim_end_matches(";").replace(";", ":")))
                    .unwrap_or(String::new()),
            ),
        );
    }
}
