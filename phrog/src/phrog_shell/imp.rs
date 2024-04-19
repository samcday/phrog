use gtk::glib;
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};
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
    fn setup_idle_cb(&self) -> bool {
        println!("hehe.");
        return false;
    }
}
