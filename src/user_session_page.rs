use crate::session_object::SessionObject;
use gtk::{glib, ListBoxRow};
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

    pub fn select_user(&self, username: &str) {
        for w in self.imp().box_users.children() {
            if let Ok(row) = w.downcast::<ActionRow>() {
                if row.subtitle().filter(|v| v == username).is_some() {
                    self.imp().box_users.select_row(Some(&row));
                    return;
                }
            }
        }
        self.imp().box_users.select_row(self.imp().box_users.children().first().and_then(|v| v.downcast_ref::<ListBoxRow>()));
    }

    pub fn select_session(&self, name: &str) {
        for (idx, session) in self.imp().sessions.get().unwrap().iter::<SessionObject>().flatten().enumerate() {
            if session.id() == name {
                self.imp().row_sessions.set_selected_index(idx as _);
                return;
            }
        }
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
    use crate::session_object::SessionObject;
    use crate::sessions;
    use glib::subclass::InitializingObject;
    use gtk::gio::ListStore;
    use gtk::glib::subclass::Signal;
    use gtk::glib::clone;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate, Image, ListBox};
    use libhandy::prelude::*;
    use libhandy::ActionRow;
    use std::cell::OnceCell;
    use std::sync::OnceLock;
    use futures_util::select;
    use crate::shell::Shell;
    use crate::dbus::accounts::AccountsProxy;
    use crate::user::User;
    use futures_util::StreamExt;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/mobi/phosh/phrog/lockscreen-user-session.ui")]
    pub struct UserSessionPage {
        #[template_child]
        pub box_users: TemplateChild<ListBox>,

        #[template_child]
        pub row_sessions: TemplateChild<libhandy::ComboRow>,

        users: OnceCell<ListStore>,
        pub sessions: OnceCell<ListStore>,
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

    impl ObjectImpl for UserSessionPage {
        fn constructed(&self) {
            self.parent_constructed();

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

            let users = ListStore::new::<User>();

            self.box_users.bind_model(Some(&users), |v| {
                let user = v.downcast_ref::<User>().unwrap();
                let row = ActionRow::builder()
                    .activatable(true)
                    .build();
                user.bind_property("username", &row, "subtitle").build();
                user.bind_property("name", &row, "title").build();
                let image = Image::new();
                row.add_prefix(&image);
                image.show();
                user.bind_property("icon-pixbuf", &image, "pixbuf").build();
                row.upcast()
            });

            self.users.set(users.clone()).unwrap();
            glib::spawn_future_local(clone!(@weak self as this => async move {
                let accounts_proxy = AccountsProxy::new(&conn).await.unwrap();

                for path in accounts_proxy.list_cached_users().await.unwrap() {
                    users.append(&User::new(conn.clone(), path.into()));
                }

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
            SIGNALS.get_or_init(|| vec![Signal::builder("login").build()])
        }
    }

    impl WidgetImpl for UserSessionPage {}
    impl ContainerImpl for UserSessionPage {}
    impl BoxImpl for UserSessionPage {}
}
