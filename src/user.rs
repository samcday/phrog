use crate::dbus::user::UserProxy;
use futures_util::select;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::Cancellable;
use gtk::glib::{clone, g_warning, spawn_future_local, Object};
use gtk::prelude::FileExt;
use gtk::{gio, glib};
use zbus::export::futures_util::StreamExt;
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

glib::wrapper! {
    pub struct User(ObjectSubclass<imp::User>);
}

impl User {
    pub fn new(conn: zbus::Connection, path: ObjectPath) -> Self {
        let obj: Self = Object::builder()
            .property("path", path.as_str())
            .build();

        let path = OwnedObjectPath::from(path);
        spawn_future_local(clone!(@weak obj => async move {
            let user_proxy = if let Ok(proxy) = UserProxy::builder(&conn)
                .path(&path)
                .expect(&format!("failed to construct UserProxy for {}", path))
                .build()
                .await
            {
                proxy
            } else {
                g_warning!("user", "failed to construct UserProxy for {}", path);
                return;
            };

            if let Ok(v) = user_proxy.user_name().await {
                obj.set_username(v);
            }
            if let Ok(v) = user_proxy.real_name().await {
                obj.set_name(v);
            }
            if let Ok(v) = user_proxy.icon_file().await {
                obj.set_icon_file(v);
            }

            let mut name_stream = user_proxy.receive_real_name_changed().await.fuse();
            let mut username_stream = user_proxy.receive_user_name_changed().await.fuse();
            let mut icon_stream = user_proxy.receive_icon_file_changed().await.fuse();

            loop {
                select! {
                    name = name_stream.next() => if let Some(name) = name {
                        if let Ok(v) = name.get().await {
                            obj.set_name(v);
                        }
                    },
                    username = username_stream.next() => if let Some(username) = username {
                        if let Ok(v) = username.get().await {
                            obj.set_username(v);
                        }
                    },
                    icon = icon_stream.next() => if let Some(icon) = icon {
                        if let Ok(v) = icon.get().await {
                            obj.set_icon_file(v);
                        }
                    },
                }
            }
        }));
        obj
    }

    pub fn load_pixbuf(&self, f: &gio::File) {
        let c = Cancellable::current();
        if let Ok(input) = f.read(c.as_ref()) {
            if let Ok(pixbuf) = Pixbuf::from_stream_at_scale(&input, 32, 32, true, c.as_ref()) {
                self.set_icon_pixbuf(pixbuf);
            }
        }
    }
}

mod imp {
    use gtk::gdk_pixbuf::Pixbuf;
    use gtk::gio::{Cancellable, FileMonitorFlags};
    use gtk::glib::{clone, g_warning, Properties};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};
    use std::cell::RefCell;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::User)]
    pub struct User {
        #[property(get, set)]
        path: RefCell<String>,
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        username: RefCell<String>,
        #[property(get, set)]
        icon_file: RefCell<Option<String>>,
        #[property(get, set)]
        icon_monitor: RefCell<Option<gio::FileMonitor>>,
        #[property(get, set)]
        icon_pixbuf: RefCell<Option<Pixbuf>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for User {
        const NAME: &'static str = "PhrogUser";
        type Type = super::User;
    }

    #[glib::derived_properties]
    impl ObjectImpl for User {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().connect_icon_file_notify(move |user| {
                if let Some(path) = user.icon_file() {
                    let file = gio::File::for_path(&path);
                    user.load_pixbuf(&file);

                    let c = Cancellable::current();
                    match file.monitor(FileMonitorFlags::empty(), c.as_ref()) {
                        Ok(monitor) => user.set_icon_monitor(monitor.clone()),
                        Err(err) => g_warning!("user", "error starting file monitor on {}: {}", path, err),
                    }
                }
            });
            self.obj().connect_icon_monitor_notify(move |user| {
                if let Some(monitor) = user.icon_monitor() {
                    monitor.connect_changed(clone!(@weak user => move |_, f, _, _| {
                        user.load_pixbuf(f);
                    }));
                }
            });
        }
    }
}
