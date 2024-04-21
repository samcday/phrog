use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox};
use gtk::gio::ListStore;
use libhandy::ActionRow;
use libhandy::prelude::*;
use crate::session_object::SessionObject;
use crate::{sessions, users};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/samcday/phrog/lockscreen-user-session.ui")]
pub struct UserSessionPage {
    #[template_child]
    pub box_users: TemplateChild<ListBox>,

    #[template_child]
    pub row_sessions: TemplateChild<libhandy::ComboRow>,
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
            self.box_users.add(&ActionRow::builder().title(user.1).subtitle(user.0).activatable(true).build());
        }
        self.box_users.show_all();

        self.box_users.connect_row_activated(|_, row| {
            println!("user selected: {}",row.downcast_ref::<ActionRow>().unwrap().subtitle().unwrap());
        });

        let session_store = ListStore::new::<SessionObject>();
        session_store.extend_from_slice(&sessions::sessions());

        self.row_sessions.bind_name_model(Some(&session_store), Some(Box::new(|v| {
            v.downcast_ref::<SessionObject>().unwrap().name()
        })));
    }
}

impl WidgetImpl for UserSessionPage {}

impl ContainerImpl for UserSessionPage {}

impl BoxImpl for UserSessionPage {}
