use gtk::{Button, Label, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::manager::settings_switcher_manager::{SettingsSwitcherEvent, SettingsSwitcherManager};
use crate::ui::controls::Control;
use crate::ui::pages::{APPEARANCE_PAGE, DISPLAY_PAGE, GENERAL_PAGE, INFO_PAGE, KEYBINDS_PAGE, LOCKSCREEN_PAGE, OVERVIEW_PAGE, STARTUP_PROGRAMS_PAGE, WALLPAPER_PAGE};

pub struct Navigation {
    settings_switcher_manager: SettingsSwitcherManager,
    settings_navigation_box: GTKBox,
    overview_button: Button,
    general_button: Button,
    display_button: Button,
    wallpaper_button: Button,
    lockscreen_button: Button,
    appearance_button: Button,
    startup_button: Button,
    keybinds_button: Button,
    info_button: Button,
    save_button: Button,
}

impl Control for Navigation {
    fn init_events(&self) {
    }

    fn get_widget(&self) -> &GTKBox {
        &self.settings_navigation_box
    }
}

impl Navigation {
    pub fn new(settings_switcher_manager: SettingsSwitcherManager) -> Self {
        let settings_navigation_box = BoxBuilder::new("navigation")
            .set_orientation(Orientation::Vertical)
            .set_width(320)
            .build();

        let overview_button =  Self::create_settings_button("👀 overview");
        let general_button = Self::create_settings_button("💾 user programs");
        let display_button = Self::create_settings_button("🖥️ display");
        let wallpaper_button = Self::create_settings_button("🖼️ wallpaper");
        let lockscreen_button = Self::create_settings_button("🔐 lockscreen");
        let appearance_button = Self::create_settings_button("🖌️ appearance");
        let startup_button = Self::create_settings_button("🚀 startup");
        let keybinds_button = Self::create_settings_button("⌨️ keybinds");
        let info_button = Self::create_settings_button("📝 info");
    
        let save_button = Button::with_label("save");
        save_button.set_widget_name("save-settings");
        save_button.set_height_request(48);
        save_button.set_margin_top(10);

        settings_navigation_box.append(&overview_button);
        settings_navigation_box.append(&general_button);
        settings_navigation_box.append(&display_button);
        settings_navigation_box.append(&wallpaper_button);
        settings_navigation_box.append(&lockscreen_button);
        settings_navigation_box.append(&appearance_button);
        settings_navigation_box.append(&startup_button);
        settings_navigation_box.append(&keybinds_button);
        settings_navigation_box.append(&info_button);
        settings_navigation_box.append(&save_button);
        
        Self {
            settings_switcher_manager,
            settings_navigation_box,
            overview_button,
            general_button,
            display_button,
            wallpaper_button,
            lockscreen_button,
            appearance_button,
            startup_button,
            keybinds_button,
            info_button,
            save_button,
        }
    }

    pub fn init_events(&self) {
        let settings_switcher_manager = self.settings_switcher_manager.clone();
        let overview_button_click = Self::create_settings_button_click(
            OVERVIEW_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.overview_button.connect_clicked(overview_button_click);

        let general_button_click = Self::create_settings_button_click(
            GENERAL_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.general_button.connect_clicked(general_button_click);

        let display_button_click = Self::create_settings_button_click(
            DISPLAY_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.display_button.connect_clicked(display_button_click);

        let wallpaper_button_click = Self::create_settings_button_click(
            WALLPAPER_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.wallpaper_button.connect_clicked(wallpaper_button_click);

        let lockscreen_button_click = Self::create_settings_button_click(
            LOCKSCREEN_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.lockscreen_button.connect_clicked(lockscreen_button_click);

        let appearance_button_click = Self::create_settings_button_click(
            APPEARANCE_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.appearance_button.connect_clicked(appearance_button_click);

        let startup_button_click = Self::create_settings_button_click(
            STARTUP_PROGRAMS_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.startup_button.connect_clicked(startup_button_click);

        let keybinds_button_click = Self::create_settings_button_click(
            KEYBINDS_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.keybinds_button.connect_clicked(keybinds_button_click);

        let info_button_click = Self::create_settings_button_click(
            INFO_PAGE.to_string(), settings_switcher_manager.clone()
        );
        self.info_button.connect_clicked(info_button_click);

        let save_button_click = move |_: &Button| {
            settings_switcher_manager.notify_event(SettingsSwitcherEvent::SaveSettings);
        };
        self.save_button.connect_clicked(save_button_click);
    }

    fn create_settings_button(label_text: &str) -> Button {
        let label = Label::new(Some(label_text));
        label.set_xalign(0.0);
        label.set_hexpand(false);
        label.set_margin_start(80);

        let button = Button::new();
        button.set_height_request(48);
        button.set_child(Some(&label));
        button
    }

    fn create_settings_button_click(
        settings_name: String, settings_switcher_manager: SettingsSwitcherManager
    ) -> impl Fn(&Button) + 'static
    {
        move |_: &Button| {
            let new_control_name = SettingsSwitcherEvent::NewControlName(settings_name.clone());
            settings_switcher_manager.notify_event(new_control_name);
        }
    }
}