use std::cell::RefCell;
use std::rc::Rc;
use gtk::Orientation;
use gtk::prelude::{BoxExt, WidgetExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::manager::settings_switcher_manager::SettingsSwitcherManager;
use crate::ui::pages::settings::general_page::GeneralSettings;
use crate::ui::pages::settings::{APPEARANCE_SETTINGS, DISPLAY_SETTINGS, GENERAL_SETTINGS, INFO_SETTINGS, KEYBINDS_SETTINGS, OVERVIEW_SETTINGS, STARTUP_PROGRAM_SETTINGS};
use crate::ui::controls::navigation::Navigation;
use crate::ui::controls::settings_switcher::SettingsSwitcher;
use crate::ui::states::general_settings_state::GeneralSettingsState;
use crate::ui::controls::Control;
use crate::ui::pages::settings::appearance_page::AppearanceSettings;
use crate::ui::pages::settings::display_page::DisplaySettings;
use crate::ui::pages::settings::info_page::InfoSettings;
use crate::ui::pages::settings::keybinds_page::KeyBindsSettings;
use crate::ui::pages::settings::overview_page::OverviewPage;
use crate::ui::pages::settings::startups_page::StartupProgramsSettings;
use crate::ui::states::display_settings_state::DisplaySettingsState;
use crate::ui::states::settings_switcher_state::SettingsSwitcherState;
use crate::ui::updatable_control::UpdatableControl;

pub struct App {
    app_box: gtk::Box
}

impl Control for App {
    fn init_events(&self) {

    }

    fn get_widget(&self) -> &gtk::Box {
        &self.app_box
    }
}

impl App {
    pub fn new() -> Self {
        let application_provider = ApplicationProvider::new();
        
        let app_box = gtk::Box::new(Orientation::Horizontal, 10);
        app_box.set_margin_start(10);
        app_box.set_margin_end(10);
        app_box.set_margin_top(10);
        app_box.set_margin_bottom(10);

        let overview_settings = Box::new(OverviewPage::new(application_provider.clone()));

        let state = GeneralSettingsState::from(&application_provider);
        let mut general_settings = Box::new(GeneralSettings::new(application_provider.clone()));
        general_settings.init_events();
        general_settings.update_ui(state);

        let state = DisplaySettingsState::from(&application_provider);
        let mut display_settings = Box::new(DisplaySettings::new(application_provider.clone()));
        display_settings.init_events();
        display_settings.update_ui(state);

        let appearance_settings = Box::new(AppearanceSettings::new(application_provider.clone()));
        appearance_settings.init_events();

        let keybinds_settings = Box::new(KeyBindsSettings::new(application_provider.clone()));
        keybinds_settings.init_events();

        let startup_program_settings = Box::new(StartupProgramsSettings::new());
        startup_program_settings.init_events(application_provider.clone());
        startup_program_settings.init_ui(application_provider.clone());
        
        let info_settings = Box::new(InfoSettings::new());

        let settings_switcher = Rc::new(RefCell::new(SettingsSwitcher::new()));
        settings_switcher.borrow_mut()
            .insert_control(OVERVIEW_SETTINGS.to_string(), overview_settings)
            .insert_control(GENERAL_SETTINGS.to_string(), general_settings)
            .insert_control(DISPLAY_SETTINGS.to_string(), display_settings)
            .insert_control(APPEARANCE_SETTINGS.to_string(), appearance_settings)
            .insert_control(KEYBINDS_SETTINGS.to_string(), keybinds_settings)
            .insert_control(STARTUP_PROGRAM_SETTINGS.to_string(), startup_program_settings)
            .insert_control(INFO_SETTINGS.to_string(), info_settings);

        let settings_switcher_state = SettingsSwitcherState::new(GENERAL_SETTINGS.to_string());
        settings_switcher.borrow_mut().update_ui(settings_switcher_state);
        
        let settings_switcher_manager = SettingsSwitcherManager::new(settings_switcher.clone(), application_provider.clone());
        let navigation = Navigation::new(settings_switcher_manager.clone());
        navigation.init_events();

        app_box.append(navigation.get_widget());
        app_box.append(settings_switcher.borrow().get_widget());

        Self {
            app_box
        }
    }
}