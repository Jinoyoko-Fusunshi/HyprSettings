use gtk::{Button, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::ui::manager::settings_switcher_manager::{SettingsSwitcherEvent, SettingsSwitcherManager};
use crate::ui::css_styles::CSSStyles;
use crate::ui::pages::settings::{APPEARANCE_SETTINGS, DISPLAY_SETTINGS, GENERAL_SETTINGS, INFO_SETTINGS, KEYBINDS_SETTINGS, OVERVIEW_SETTINGS, STARTUP_PROGRAM_SETTINGS};
use crate::ui::component::Component;

pub struct SettingsNavigation {
    settings_switcher_manager: SettingsSwitcherManager,
    settings_navigation_box: gtk::Box,
    overview_button: Button,
    general_button: Button,
    display_button: Button,
    appearance_button: Button,
    startup_button: Button,
    keybinds_button: Button,
    info_button: Button,
    save_button: Button,
}

impl Component for SettingsNavigation {
    fn init_events(&self) {
    }

    fn get_widget(&self) -> &gtk::Box {
        &self.settings_navigation_box
    }
}

impl SettingsNavigation {
    pub fn new(settings_switcher_manager: SettingsSwitcherManager) -> Self {
        let settings_navigation_box = gtk::Box::new(Orientation::Vertical, 10);
        settings_navigation_box.set_width_request(320);
        settings_navigation_box.add_css_class(CSSStyles::NAVIGATION_PANEL);

        let overview_button = Button::with_label("overview");
        overview_button.set_height_request(48);

        let general_button = Button::with_label("general");
        general_button.set_height_request(48);

        let display_button = Button::with_label("display");
        display_button.set_height_request(48);

        let appearance_button = Button::with_label("appearance");
        appearance_button.set_height_request(48);

        let startup_button = Button::with_label("startup");
        startup_button.set_height_request(48);

        let keybinds_button = Button::with_label("keybinds");
        keybinds_button.set_height_request(48);
    
        let info_button = Button::with_label("info");
        info_button.set_height_request(48);
    
        let save_button = Button::with_label("Save");
        save_button.set_height_request(48);
        save_button.set_margin_top(10);
        save_button.add_css_class(CSSStyles::SAVE_BUTTON);
        
        settings_navigation_box.append(&overview_button);
        settings_navigation_box.append(&general_button);
        settings_navigation_box.append(&display_button);
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
            appearance_button,
            startup_button,
            keybinds_button,
            info_button,
            save_button,
        }
    }

    pub fn init_events(&self) {
        let settings_switcher_manager_clone = self.settings_switcher_manager.clone();
        let overview_button_click = move |_: &Button| {
            settings_switcher_manager_clone.notify_event(
                SettingsSwitcherEvent::NewComponentName(OVERVIEW_SETTINGS.to_string())
            );
        };
        self.overview_button.connect_clicked(overview_button_click);

        let settings_switcher_manager_clone = self.settings_switcher_manager.clone();
        let general_button_click = move |_: &Button| {
            settings_switcher_manager_clone.notify_event(
                SettingsSwitcherEvent::NewComponentName(GENERAL_SETTINGS.to_string())
            );
        };
        self.general_button.connect_clicked(general_button_click);

        let settings_switcher_manager_clone = self.settings_switcher_manager.clone();
        let display_button_click = move |_: &Button| {
            settings_switcher_manager_clone.notify_event(
                SettingsSwitcherEvent::NewComponentName(DISPLAY_SETTINGS.to_string())
            );
        };
        self.display_button.connect_clicked(display_button_click);

        let settings_switcher_manager_clone = self.settings_switcher_manager.clone();
        let appearance_button_click = move |_: &Button| {
            settings_switcher_manager_clone.notify_event(
                SettingsSwitcherEvent::NewComponentName(APPEARANCE_SETTINGS.to_string())
            );
        };
        self.appearance_button.connect_clicked(appearance_button_click);

        let settings_switcher_manager_clone = self.settings_switcher_manager.clone();
        let startup_button_click = move |_: &Button| {
            settings_switcher_manager_clone.notify_event(
                SettingsSwitcherEvent::NewComponentName(STARTUP_PROGRAM_SETTINGS.to_string())
            );
        };
        self.startup_button.connect_clicked(startup_button_click);

        let settings_switcher_manager_clone = self.settings_switcher_manager.clone();
        let keybinds_button_click = move |_: &Button| {
            settings_switcher_manager_clone.notify_event(
                SettingsSwitcherEvent::NewComponentName(KEYBINDS_SETTINGS.to_string())
            );
        };
        self.keybinds_button.connect_clicked(keybinds_button_click);

        let settings_switcher_manager_clone = self.settings_switcher_manager.clone();
        let info_button_click = move |_: &Button| {
            let new_component_name = SettingsSwitcherEvent::NewComponentName(INFO_SETTINGS.to_string());
            settings_switcher_manager_clone.notify_event(new_component_name);
        };
        self.info_button.connect_clicked(info_button_click);

        let settings_switcher_manager_clone = self.settings_switcher_manager.clone();
        let save_button_click = move |_: &Button| {
            settings_switcher_manager_clone.notify_event(SettingsSwitcherEvent::SaveSettings);
        };
        self.save_button.connect_clicked(save_button_click);
    }
}