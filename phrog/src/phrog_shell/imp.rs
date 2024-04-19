use gtk::glib;
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt};
use phosh_dm::ShellExt;
use phosh_dm::subclass::shell::ShellImpl;

#[derive(Default)]
pub struct PhrogShell;

#[glib::object_subclass]
impl ObjectSubclass for PhrogShell {
    const NAME: &'static str = "PhrogShell";
    type Type = super::PhrogShell;
    type ParentType = phosh_dm::Shell;
}

impl ObjectImpl for PhrogShell {

}

impl ShellImpl for PhrogShell {
    fn setup(&self) {
        println!("PhrogShell setup.");
        self.obj().panels_create();
        self.obj().setup_primary_monitor_signal_handlers();
    }
}
