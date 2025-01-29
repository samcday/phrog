use crate::session_object::SessionObject;
use gtk::glib;
use gtk::glib::{Cast, CastNone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::traits::ListBoxExt;
use libhandy::prelude::ActionRowExt;
use libhandy::traits::ComboRowExt;
use libhandy::ActionRow;

glib::wrapper! {
    pub struct UserSessionPage(ObjectSubclass<imp::UserSessionPage>)
        @extends gtk::Widget, gtk::Box;
}

impl Default for UserSessionPage {
    fn default() -> Self {
        Self::new()
    }
}

impl UserSessionPage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn session(&self) -> SessionObject {
        let session_idx = self.imp().row_sessions.selected_index() as u32;
        let sessions = self.imp().sessions.get().unwrap();
        sessions
            .item(session_idx)
            .clone()
            .and_downcast::<SessionObject>()
            .unwrap()
    }

    pub fn username(&self) -> Option<String> {
        self.imp()
            .box_users
            .selected_row()
            .and_then(|row| row.downcast_ref::<ActionRow>().unwrap().subtitle())
            .map(|str| str.to_string())
    }
}

mod imp {
    use crate::dbus::accounts::AccountsProxy;
    use crate::session_object::SessionObject;
    use crate::shell::Shell;
    use crate::user::User;
    use crate::{sessions, APP_ID};
    use futures_util::select;
    use futures_util::StreamExt;
    use glib::subclass::InitializingObject;
    use gtk::gio::{ListStore, Settings};
    use gtk::glib::subclass::Signal;
    use gtk::glib::{clone, closure_local};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate, Image, ListBox, ListBoxRow};
    use libhandy::prelude::*;
    use libhandy::ActionRow;
    use std::cell::{Cell, OnceCell};
    use std::sync::OnceLock;
    use glib::Properties;

    #[derive(CompositeTemplate, Default, Properties)]
    #[properties(wrapper_type = super::UserSessionPage)]
    #[template(resource = "/mobi/phosh/phrog/lockscreen-user-session.ui")]
    pub struct UserSessionPage {
        #[template_child]
        pub box_users: TemplateChild<ListBox>,

        #[template_child]
        pub row_sessions: TemplateChild<libhandy::ComboRow>,

        users: OnceCell<ListStore>,
        pub sessions: OnceCell<ListStore>,

        #[property(get, set)]
        ready: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UserSessionPage {
        const NAME: &'static str = "PhrogUserSessionPage";
        type Type = super::UserSessionPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.set_css_name("phrog-user-session-page");
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for UserSessionPage {
        fn constructed(&self) {
            self.parent_constructed();

            let settings = Settings::new(APP_ID);
            let shell = libphosh::Shell::default().downcast::<Shell>().unwrap();
            let conn = shell.imp().dbus_connection.clone().into_inner().unwrap();

            self.box_users
                .connect_row_activated(clone!(@weak self as this => move |_, _| {
                    this.obj().emit_by_name::<()>("login", &[]);
                }));

            self.sessions
                .set(ListStore::new::<SessionObject>())
                .unwrap();

            let session_list = sessions::sessions();
            self.sessions
                .get()
                .unwrap()
                .extend_from_slice(&session_list);

            self.row_sessions.bind_name_model(
                Some(self.sessions.get().unwrap()),
                Some(Box::new(|v| {
                    v.downcast_ref::<SessionObject>().unwrap().name()
                })),
            );
            let last_session = settings.string("last-session");
            for (idx, session) in self
                .sessions
                .get()
                .unwrap()
                .iter::<SessionObject>()
                .flatten()
                .enumerate()
            {
                if session.id() == last_session {
                    self.row_sessions.set_selected_index(idx as _);
                    break;
                }
            }

            let users = ListStore::new::<User>();
            let last_user = settings.string("last-user");

            self.box_users.bind_model(Some(&users), clone!(@weak self as this, @strong last_user => @default-panic, move |v| {
                let user = v.downcast_ref::<User>().unwrap();
                let row = ActionRow::builder().activatable(true).build();
                user.bind_property("username", &row, "subtitle").build();
                user.bind_property("name", &row, "title").build();
                let image = Image::new();
                row.add_prefix(&image);
                image.show();
                user.bind_property("icon-pixbuf", &image, "pixbuf").build();

                user.connect_closure("loaded", false, closure_local!(@strong this, @strong last_user, @strong row => move |obj: glib::Object| {
                    if let Ok(user) = obj.downcast::<User>() {
                        if user.username() == last_user {
                            this.box_users.select_row(Some(&row));
                        }
                    }
                }));

                row.upcast()
            }));

            self.users.set(users.clone()).unwrap();
            glib::spawn_future_local(clone!(@weak self as this, @strong last_user => async move {
                let accounts_proxy = AccountsProxy::new(&conn).await.unwrap();

                for path in accounts_proxy.list_cached_users().await.unwrap() {
                    users.append(&User::new(conn.clone(), path.into()));
                }

                // The initial user list has been populated. Select the first item in the list to
                // ensure something is selected.
                // This will be overridden by the "loaded" signal handler, if the appropriate user
                // matching the last-user setting was discovered.
                this.box_users.select_row(
                    this
                        .box_users
                        .children()
                        .first()
                        .and_then(|v| v.downcast_ref::<ListBoxRow>()),
                );

                this.obj().set_ready(true);

                let mut added_stream = accounts_proxy.receive_user_added().await.unwrap();
                let mut deleted_stream = accounts_proxy.receive_user_deleted().await.unwrap();

                loop {
                    select! {
                        added = added_stream.next() => if let Some(added) = added {
                            if let Some(path) = added.args().ok().map(|v| v.user) {
                                users.append(&User::new(conn.clone(), path));
                            }
                        },
                        deleted = deleted_stream.next() => if let Some(deleted) = deleted {
                            if let Some(path) = deleted.args().ok().map(|v| v.user) {
                                for (idx, user) in users.iter::<User>().flatten().enumerate() {
                                    if user.path() == path.as_str() {
                                        users.remove(idx as _);
                                        break;
                                    }
                                }
                            }
                        },
                    }
                }
            }));
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("login").build(),
                ]
            })
        }
    }

    impl WidgetImpl for UserSessionPage {}
    impl ContainerImpl for UserSessionPage {}
    impl BoxImpl for UserSessionPage {}
}
