use glib::{Cast, Object};
use gtk::glib;

static G_LOG_DOMAIN: &str = "phrog";

glib::wrapper! {
    pub struct Shell(ObjectSubclass<imp::Shell>)
        @extends libphosh::Shell, gtk::gio::ActionGroup;
}

impl Shell {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for Shell {
    fn default() -> Self {
        libphosh::Shell::default().downcast::<Shell>().unwrap()
    }
}

mod imp {
    use super::G_LOG_DOMAIN;
    use crate::lockscreen::Lockscreen;
    use crate::session_object::SessionObject;
    use crate::sessions;
    use glib::{clone, spawn_future_local, warn};
    use gtk::gio::Settings;
    use gtk::gio::{spawn_blocking, ListStore};
    use gtk::glib::{Properties, Type};
    use gtk::prelude::StaticType;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::{gdk, glib, CssProvider, StyleContext};
    use libphosh::prelude::ShellExt;
    use libphosh::subclass::shell::ShellImpl;
    use std::cell::RefCell;
    use std::cell::{Cell, OnceCell};
    use std::process::Command;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Shell)]
    pub struct Shell {
        #[property(get, set)]
        fake_greetd: Cell<bool>,

        #[property(get, set)]
        pub sessions: RefCell<Option<ListStore>>,

        provider: Cell<CssProvider>,
        pub dbus_connection: OnceCell<zbus::Connection>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Shell {
        const NAME: &'static str = "PhrogShell";
        type Type = super::Shell;
        type ParentType = libphosh::Shell;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Shell {
        fn constructed(&self) {
            let system_dbus = async_global_executor::block_on(zbus::Connection::system()).unwrap();
            self.dbus_connection.set(system_dbus).unwrap();

            self.parent_constructed();

            if self.obj().sessions().is_none() {
                let sessions_store = ListStore::new::<SessionObject>();
                sessions_store.extend_from_slice(&sessions::sessions());
                self.obj().set_sessions(sessions_store);
            }

            let provider = CssProvider::new();
            provider.load_from_resource("/mobi/phosh/phrog/phrog.css");
            StyleContext::add_provider_for_screen(
                &gdk::Screen::default().unwrap(),
                &provider,
                // Slightly hacky, we want to be above phosh to override some stuff
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 5,
            );
            self.provider.set(provider);

            let settings = Settings::new(crate::APP_ID);

            let shell = self.to_owned();
            glib::idle_add_local_once(move || {
                let first_run = settings.string("first-run");
                if !first_run.is_empty() {
                    spawn_future_local(clone!(@weak shell as this => async move {
                        if let Err(err) = spawn_blocking(|| {
                                Command::new(first_run).spawn().and_then(|mut child| child.wait())
                            }).await
                        {
                            warn!("Failed to execute first-run app: {:?}", err);
                        }
                        this.obj().set_locked(true);
                    }));
                } else {
                    shell.obj().set_locked(true);
                }
            });
        }
    }

    impl ShellImpl for Shell {
        fn get_lockscreen_type(&self) -> Type {
            Lockscreen::static_type()
        }
    }
}
