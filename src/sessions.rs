use crate::session_object::SessionObject;
use glib::{warn, KeyFile, KeyFileFlags};
use glob::glob;
use lazy_static::lazy_static;
use std::{collections::HashMap, env, path::Path, path::PathBuf};

static G_LOG_DOMAIN: &str = "phrog-sessions";

lazy_static! {
    // Snippet copied from https://github.com/apognu/tuigreet

    static ref XDG_DATA_DIRS: Vec<PathBuf> = {
        let envvar = env::var("XDG_DATA_DIRS");

        let value = match envvar {
            Ok(var) if !var.is_empty() => var,
            _ => "/usr/local/share:/usr/share".to_string(),
        };

        env::split_paths(&value).filter(|p| p.is_absolute()).collect()
    };
}

pub fn sessions() -> Vec<SessionObject> {
    let mut sessions = HashMap::new();

    for dir in XDG_DATA_DIRS.iter() {
        let wl_dir = dir.join("wayland-sessions/*.desktop");
        let x11_dir = dir.join("xsessions/*.desktop");

        session_list(&wl_dir, "wayland", &mut sessions);

        session_list(&x11_dir, "x11", &mut sessions);
    }

    sessions.values().cloned().collect()
}

fn session_list(path: &Path, session_type: &str, sessions: &mut HashMap<String, SessionObject>) {
    let keyfile_locale_string = |key_file: &KeyFile, key: &str| {
        key_file
            .locale_string("Desktop Entry", key, None)
            .or_else(|_| key_file.string("Desktop Entry", key))
            .ok()
    };

    for f in match glob(path.to_str().unwrap()) {
        Err(e) => {
            warn!("couldn't check sessions in {}: {}", path.display(), e);
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

        let key_file = KeyFile::new();
        if let Err(err) = key_file.load_from_file(&f, KeyFileFlags::NONE) {
            warn!("Unable to parse session file {:?}: {}", f, err);
            continue;
        }

        let name = match keyfile_locale_string(&key_file, "Name") {
            Some(value) => value,
            None => {
                warn!("Missing Name in session file {:?}", f);
                continue;
            }
        };
        let command = key_file
            .string("Desktop Entry", "Exec")
            .ok()
            .unwrap_or_default();
        let desktop_names = key_file
            .string("Desktop Entry", "DesktopNames")
            .ok()
            .map(|value| value.trim_end_matches(';').replace(';', ":"))
            .unwrap_or_default();
        sessions.insert(
            id.clone(),
            SessionObject::new(&id, &name, session_type, &command, &desktop_names),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::session_list;
    use crate::session_object::SessionObject;
    use gtk::prelude::ObjectExt;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn session_list_reads_desktop_keyfile() {
        let temp = tempdir().expect("create tempdir");
        let session_dir = temp.path().join("wayland-sessions");
        fs::create_dir_all(&session_dir).expect("create session dir");

        let desktop_entry = "[Desktop Entry]\n\
Name=Phosh\n\
Comment=Phone Shell\n\
Comment=This session logs you into Phosh\n\
Exec=phosh-session\n\
Type=Application\n\
DesktopNames=Phosh;GNOME;\n";

        let desktop_path = session_dir.join("phosh.desktop");
        fs::write(&desktop_path, desktop_entry).expect("write desktop file");

        let mut sessions: HashMap<String, SessionObject> = HashMap::new();
        let pattern = session_dir.join("*.desktop");
        session_list(&pattern, "wayland", &mut sessions);

        let session = sessions.get("phosh").expect("session added");
        let name: String = session.property("name");
        let session_type: String = session.property("session-type");
        let command: String = session.property("command");
        let desktop_names: String = session.property("desktop-names");

        assert_eq!(name, "Phosh");
        assert_eq!(session_type, "wayland");
        assert_eq!(command, "phosh-session");
        assert_eq!(desktop_names, "Phosh:GNOME");
    }

    #[test]
    fn session_list_accepts_keyfile_without_type() {
        let temp = tempdir().expect("create tempdir");
        let session_dir = temp.path().join("wayland-sessions");
        fs::create_dir_all(&session_dir).expect("create session dir");

        let desktop_entry = "[Desktop Entry]\n\
Name=Phosh\n\
Comment=Phone Shell\n\
Exec=phosh-session\n\
DesktopNames=Phosh;GNOME;\n";

        let desktop_path = session_dir.join("phosh.desktop");
        fs::write(&desktop_path, desktop_entry).expect("write desktop file");

        let mut sessions: HashMap<String, SessionObject> = HashMap::new();
        let pattern = session_dir.join("*.desktop");
        session_list(&pattern, "wayland", &mut sessions);

        assert!(sessions.contains_key("phosh"));
    }
}
