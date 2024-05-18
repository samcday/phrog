use glib::Object;
use gtk::glib;
use gtk::glib::{Cast, CastNone};
use gtk::prelude::ListModelExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::traits::ListBoxExt;
use libhandy::ActionRow;
use libhandy::prelude::ActionRowExt;
use libhandy::traits::ComboRowExt;
use crate::session_object::SessionObject;

glib::wrapper! {
    pub struct UserSessionPage(ObjectSubclass<imp::UserSessionPage>)
        @extends gtk::Widget, gtk::Box;
}

impl UserSessionPage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn session(&self) -> SessionObject {
        let session_idx = self.imp().row_sessions.selected_index() as u32;
        let sessions = self.imp().sessions.get().unwrap();
        sessions.item(session_idx).clone().and_downcast::<SessionObject>().unwrap()
    }

    pub fn username(&self) -> Option<String> {
        self.imp().box_users.selected_row()
            .and_then(|row| row.downcast_ref::<ActionRow>().unwrap().subtitle())
            .and_then(|str| Some(str.to_string()))
    }
}

mod imp {
    use crate::session_object::SessionObject;
    use crate::{sessions, users};
    use glib::subclass::InitializingObject;
    use gtk::gio::ListStore;
    use gtk::glib::subclass::Signal;
    use gtk::glib::clone;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate, ListBox};
    use libhandy::prelude::*;
    use libhandy::ActionRow;
    use std::cell::OnceCell;
    use std::sync::OnceLock;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/samcday/phrog/lockscreen-user-session.ui")]
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

            for user in users::users() {
                let row = ActionRow::builder()
                        .title(user.1)
                        .subtitle(user.0)
                        .activatable(true)
                        .build();
                self.box_users.add(&row);
            }
            self.box_users.select_row(self.box_users.row_at_index(0).as_ref());
            self.box_users.show_all();

            self.box_users.connect_row_activated(clone!(@weak self as this => move |_, _| {
                this.obj().emit_by_name::<()>("login", &[]);
            }));

            self.sessions
                .set(ListStore::new::<SessionObject>())
                .unwrap();
            self.sessions
                .get()
                .unwrap()
                .extend_from_slice(&sessions::sessions());

            self.row_sessions.bind_name_model(
                Some(self.sessions.get().unwrap()),
                Some(Box::new(|v| {
                    v.downcast_ref::<SessionObject>().unwrap().name()
                })),
            );
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![Signal::builder("login")
                    .build()]
            })
        }
    }

    impl WidgetImpl for UserSessionPage {}

    impl ContainerImpl for UserSessionPage {}

    impl BoxImpl for UserSessionPage {}
}
