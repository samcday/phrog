use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox};
use gtk::gio::ListModel;
use libhandy::prelude::PreferencesRowExt;
use libhandy::prelude::ActionRowExt;

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
    }
}

impl WidgetImpl for UserSessionPage {}

impl ContainerImpl for UserSessionPage {}

impl BoxImpl for UserSessionPage {}
