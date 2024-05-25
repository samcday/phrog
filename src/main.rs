mod lockscreen;
mod session_object;
mod sessions;
mod shell;
mod user_session_page;
mod users;

use crate::shell::Shell;
use clap::Parser;
use gtk::{Application, gio};
use gtk::glib::StaticType;
use libphosh::prelude::*;
use libphosh::{QuickSetting, WallClock};
use crate::keypad_shuffle::KeypadShuffleQuickSetting;

pub const APP_ID: &str = "com.samcday.phrog";

extern "C" {
    fn hdy_init();
    // fn cui_init(v: c_int);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

mod keypad_shuffle {
    use gtk::glib;

    glib::wrapper! {
        pub struct KeypadShuffleQuickSetting(ObjectSubclass<imp::KeypadShuffleQuickSetting>)
            @extends libphosh::QuickSetting;
    }

    mod imp {
        use gtk::glib;
        use gtk::subclass::prelude::*;
        use libphosh::subclass::quick_setting::QuickSettingImpl;

        #[derive(Default)]
        pub struct KeypadShuffleQuickSetting;

        #[glib::object_subclass]
        impl ObjectSubclass for KeypadShuffleQuickSetting {
            const NAME: &'static str = "PhrogKeypadShuffleQuickSetting";
            type Type = super::KeypadShuffleQuickSetting;
            type ParentType = libphosh::QuickSetting;
            fn class_init(_klass: &mut Self::Class) {
                println!("hi mom");
            }
        }
        impl ObjectImpl for KeypadShuffleQuickSetting {
            fn constructed(&self) {
                println!("the keypad-shuffle quick setting got constructed");
                self.parent_constructed();
            }
        }
        impl WidgetImpl for KeypadShuffleQuickSetting {}
        impl ContainerImpl for KeypadShuffleQuickSetting {}
        impl BinImpl for KeypadShuffleQuickSetting {}
        impl ButtonImpl for KeypadShuffleQuickSetting {}
        impl QuickSettingImpl for KeypadShuffleQuickSetting {}
    }
}

fn main() {
    let _args = Args::parse();

    gio::resources_register_include!("phrog.gresource")
        .expect("Failed to register resources.");

    gtk::init().unwrap();
    let _app = Application::builder().application_id(APP_ID).build();

    let wall_clock = WallClock::new();
    wall_clock.set_default();

    unsafe {
        // let loglevel =
        //     CString::new(std::env::var("G_MESSAGES_DEBUG").unwrap_or("".to_string())).unwrap();
        // phosh_log_set_log_domains(loglevel.as_ptr());
        hdy_init();
        // cui_init(1);
    }

    let shell = Shell::new();
    shell.set_default();

    shell.connect_ready(|_| {
        println!("Shell is ready");
        gio::IOExtensionPoint::implement("phosh-quick-setting-widget", KeypadShuffleQuickSetting::static_type(), "keypad-shuffle", 10).unwrap();
    });

    shell.set_locked(true);

    gtk::main();
}
