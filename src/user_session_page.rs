use crate::session_object::SessionObject;
use gtk::glib;
use gtk::glib::{Cast, CastNone, Object};
use gtk::prelude::ListModelExt;
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
    use crate::session_object::SessionObject;
    use crate::{sessions, APP_ID};
    use glib::subclass::InitializingObject;
    use gtk::gio::{ListStore, Settings};
    use gtk::glib::subclass::Signal;
    use gtk::glib::{clone, g_info, g_warning};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate, Image, ListBox};
    use libhandy::prelude::*;
    use libhandy::ActionRow;
    use std::cell::OnceCell;
    use std::sync::OnceLock;
    use gtk::gdk_pixbuf::Pixbuf;
    use crate::accounts::AccountsProxyBlocking;
    use crate::user::UserProxyBlocking;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/mobi/phosh/phrog/lockscreen-user-session.ui")]
    pub struct UserSessionPage {
        #[template_child]
        pub box_users: TemplateChild<ListBox>,

        #[template_child]
        pub row_sessions: TemplateChild<libhandy::ComboRow>,

        pub sessions: OnceCell<ListStore>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UserSessionPage {
        const NAME: &'static str = "PhrogUserSessionPage";
        type Type = super::UserSessionPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UserSessionPage {
        fn constructed(&self) {
            self.parent_constructed();

            let settings = Settings::new(APP_ID);

            let last_user = settings.string("last-user").to_string();
            let last_session = settings.string("last-session").to_string();

            let conn = zbus::blocking::Connection::system().unwrap();
            let accounts = AccountsProxyBlocking::new(&conn).unwrap();
            for path in accounts.list_cached_users().unwrap() {
                let user = UserProxyBlocking::builder(&conn)
                    .path(path).unwrap().build().unwrap();
                let user_name = user.user_name().unwrap();
                let row = ActionRow::builder()
                    .title(user.real_name().unwrap())
                    .subtitle(&user_name)
                    .activatable(true)
                    .build();
                let pixbuf = Pixbuf::from_file_at_scale(&user.icon_file().unwrap(), 32, 32, true).ok();
                row.add_prefix(&Image::from_pixbuf(pixbuf.as_ref()));
                self.box_users.add(&row);
                // use last-user setting as default for user selection
                if user_name == last_user {
                    g_warning!(
                        "user-session-page",
                        "defaulting user selection to {}",
                        last_user
                    );
                    self.box_users.select_row(Some(&row));
                }
            }
            self.box_users.show_all();
            if self.box_users.selected_row().is_none() {
                self.box_users
                    .select_row(self.box_users.row_at_index(0).as_ref());
            }

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

            // use last-session setting as default for session selection
            for (idx, session) in session_list.iter().enumerate() {
                if session.id() == last_session {
                    g_info!(
                        "user-session-page",
                        "defaulting session selection to {}",
                        session.id()
                    );
                    self.row_sessions.set_selected_index(idx as i32);
                    break;
                }
            }
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
