use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct UserSessionPage(ObjectSubclass<imp::UserSessionPage>)
        @extends gtk::Widget, gtk::Box;
}

impl UserSessionPage {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use crate::session_object::SessionObject;
    use crate::{sessions, users};
    use glib::subclass::InitializingObject;
    use gtk::gio::ListStore;
    use gtk::glib::subclass::Signal;
    use gtk::glib::{clone, Properties};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate, ListBox};
    use libhandy::prelude::*;
    use libhandy::ActionRow;
    use std::cell::OnceCell;
    use std::sync::OnceLock;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/samcday/phrog/lockscreen-user-session.ui")]
    // #[properties(wrapper_type = super::UserSessionPage)]
    pub struct UserSessionPage {
        #[template_child]
        pub box_users: TemplateChild<ListBox>,

        #[template_child]
        pub row_sessions: TemplateChild<libhandy::ComboRow>,

        sessions: OnceCell<ListStore>,
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

    // #[glib::derived_properties]
    impl ObjectImpl for UserSessionPage {
        fn constructed(&self) {
            self.parent_constructed();

            for user in users::users() {
                self.box_users.add(
                    &ActionRow::builder()
                        .title(user.1)
                        .subtitle(user.0)
                        .activatable(true)
                        .build(),
                );
            }
            self.box_users.show_all();

            self.box_users.connect_row_activated(clone!(@weak self as this => move |_, row| {
            let username = row.downcast_ref::<ActionRow>().unwrap().subtitle().unwrap();
            let session_idx = this.row_sessions.selected_index() as u32;
            let sessions = this.sessions.get().unwrap();
            let session = sessions.item(session_idx).clone().and_downcast::<SessionObject>().unwrap();
            this.obj().emit_by_name::<()>("login", &[&username, &session]);
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
                    .param_types([String::static_type(), SessionObject::static_type()])
                    .build()]
            })
        }
    }

    impl WidgetImpl for UserSessionPage {}

    impl ContainerImpl for UserSessionPage {}

    impl BoxImpl for UserSessionPage {}
}
