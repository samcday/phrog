use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox};
use gtk::gio::ListStore;
use libhandy::prelude::*;
use crate::session_object::SessionObject;
use crate::sessions;

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
        // Call "constructed" on parent
        self.parent_constructed();

        let row = libhandy::ActionRow::new();
        row.set_title(Some("Test User"));
        row.set_subtitle(Some("testuser"));
        row.set_activatable(true);
        self.box_users.prepend(&row);
        row.show();

        let row = libhandy::ActionRow::new();
        row.set_title(Some("Test User 2"));
        row.set_subtitle(Some("testuser2"));
        row.set_activatable(true);
        self.box_users.prepend(&row);
        row.show();

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
