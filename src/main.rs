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
use crate::keypad_shuffle::ShuffleKeypadQuickSetting;

pub const APP_ID: &str = "com.samcday.phrog";

extern "C" {
    fn hdy_init();
    // fn cui_init(v: core::ffi::c_int);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

mod keypad_shuffle {
    use gtk::glib;

    glib::wrapper! {
        pub struct ShuffleKeypadQuickSetting(ObjectSubclass<imp::ShuffleKeypadQuickSetting>)
            @extends libphosh::QuickSetting, gtk::Button;
    }

    mod imp {
        use gtk::{CompositeTemplate, glib};
        use gtk::gio::Settings;
        use gtk::glib::clone;
        use gtk::glib::subclass::InitializingObject;
        use gtk::prelude::{ButtonExt, InitializingWidgetExt, SettingsExt, SettingsExtManual};
        use gtk::subclass::prelude::*;
        use libphosh::prelude::{QuickSettingExt, StatusIconExt};
        use libphosh::subclass::quick_setting::QuickSettingImpl;

        #[derive(CompositeTemplate, Default)]
        #[template(resource = "/com/samcday/phrog/shuffle-keypad-quick-setting.ui")]
        pub struct ShuffleKeypadQuickSetting {
            #[template_child]
            pub info: TemplateChild<libphosh::StatusIcon>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for ShuffleKeypadQuickSetting {
            const NAME: &'static str = "PhrogKeypadShuffleQuickSetting";
            type Type = super::ShuffleKeypadQuickSetting;
            type ParentType = libphosh::QuickSetting;

            fn class_init(klass: &mut Self::Class) {
                Self::bind_template(klass);
            }

            fn instance_init(obj: &InitializingObject<Self>) {
                obj.init_template();
            }
        }

        impl ShuffleKeypadQuickSetting {
            fn update(&self) {
                self.info.set_icon_name(if self.obj().is_active() {
                    "view-refresh-symbolic"
                } else {
                    "view-app-grid-symbolic"
                });
                self.info.set_info(if self.obj().is_active() {
                    "Shuffled keypad"
                } else {
                    "Unshuffled keypad"
                })
            }
        }
        impl ObjectImpl for ShuffleKeypadQuickSetting {
            fn constructed(&self) {
                self.parent_constructed();

                let settings = Settings::new("sm.puri.phosh.lockscreen");
                settings.bind("shuffle-keypad", self.obj().as_ref(), "active").build();

                self.obj().connect_active_notify(clone!(@weak self as this => move |qs| {
                    this.update();
                }));
                self.update();

                self.obj().connect_clicked(move |btn| {
                    settings.set_boolean("shuffle-keypad", !settings.boolean("shuffle-keypad")).unwrap();
                });
            }
        }
        impl WidgetImpl for ShuffleKeypadQuickSetting {}
        impl ContainerImpl for ShuffleKeypadQuickSetting {}
        impl BinImpl for ShuffleKeypadQuickSetting {}
        impl ButtonImpl for ShuffleKeypadQuickSetting {}
        impl QuickSettingImpl for ShuffleKeypadQuickSetting {}
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
    });

    shell.set_locked(true);

    gtk::main();
}
