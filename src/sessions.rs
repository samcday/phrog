use crate::session_object::SessionObject;
use glib::warn;
use glob::glob;
use gtk::gio::DesktopAppInfo;
use gtk::prelude::*;
use lazy_static::lazy_static;
use std::{
  collections::HashMap,
  path::PathBuf,
  env,
};

static G_LOG_DOMAIN: &str = "phrog-sessions";

lazy_static! {
    // Snippet copied from https://github.com/apognu/tuigreet

    static ref XDG_DATA_DIRS: Vec<PathBuf> = {
        let value = env::var("XDG_DATA_DIRS").unwrap_or("/usr/local/share:/usr/share".to_string());
        env::split_paths(&value).filter(|p| p.is_absolute()).collect()
    };
}

pub fn sessions() -> Vec<SessionObject> {
    let mut sessions = HashMap::new();

    for dir in XDG_DATA_DIRS.iter() {
        let wl_dir = dir.join("wayland-sessions/*.desktop");
        let x11_dir = dir.join("xsessions/*.desktop");

        session_list(
            &wl_dir.into_os_string().into_string().unwrap(),
            "wayland",
            &mut sessions,
        );

        session_list(
            &x11_dir.into_os_string().into_string().unwrap(),
            "x11",
            &mut sessions,
        );        
    };

    sessions.values().cloned().collect()
}

fn session_list(path: &str, session_type: &str, sessions: &mut HashMap<String, SessionObject>) {
    for f in match glob(path) {
        Err(e) => {
            warn!("couldn't check sessions in {}: {}", path, e);
            return;
        }
        Ok(iter) => iter,
    }
    .flatten()
    {
        let id = f.file_stem().unwrap().to_string_lossy().to_string();
        if sessions.contains_key(&id) {
            continue;
        }

        let info = if let Some(info) = DesktopAppInfo::from_filename(&f) {
            info
        } else {
            warn!("Unable to parse session file {:?}", f);
            continue;
        };
        sessions.insert(
            id.clone(),
            SessionObject::new(
                &id,
                info.name().as_ref(),
                session_type,
                &info
                    .commandline()
                    .map_or(String::new(), |v| v.to_string_lossy().to_string()),
                &info
                    .string("DesktopNames")
                    .map(|v| v.trim_end_matches(';').replace(';', ":"))
                    .unwrap_or(String::new()),
            ),
        );
    }
}
