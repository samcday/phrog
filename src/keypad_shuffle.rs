use gtk::glib;

glib::wrapper! {
    pub struct ShuffleKeypadQuickSetting(ObjectSubclass<imp::ShuffleKeypadQuickSetting>)
        @extends libphosh::QuickSetting, gtk::Button;
}

mod imp {
    use gtk::gio::Settings;
    use gtk::glib::clone;
    use gtk::glib::subclass::InitializingObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};
    use libphosh::prelude::{QuickSettingExt, StatusIconExt};
    use libphosh::subclass::quick_setting::QuickSettingImpl;
    use crate::shell::Shell;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/mobi/phosh/phrog/shuffle-keypad-quick-setting.ui")]
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
            libphosh::Shell::default().downcast::<Shell>().unwrap().set_keypad_shuffle_qs(self.obj().clone());

            let settings = Settings::new("sm.puri.phosh.lockscreen");
            settings
                .bind("shuffle-keypad", self.obj().as_ref(), "active")
                .build();

            self.obj()
                .connect_active_notify(clone!(@weak self as this => move |qs| {
                    this.update();
                }));
            self.update();

            self.obj().connect_clicked(move |btn| {
                settings
                    .set_boolean("shuffle-keypad", !settings.boolean("shuffle-keypad"))
                    .unwrap();
            });
        }
    }
    impl WidgetImpl for ShuffleKeypadQuickSetting {}
    impl ContainerImpl for ShuffleKeypadQuickSetting {}
    impl BinImpl for ShuffleKeypadQuickSetting {}
    impl ButtonImpl for ShuffleKeypadQuickSetting {}
    impl QuickSettingImpl for ShuffleKeypadQuickSetting {}
}
