use std::cell::RefCell;
use std::rc::Rc;
use gtk::Orientation;
use gtk::prelude::BoxExt;
use crate::providers::application_provider::ApplicationProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::manager::settings_switcher_manager::SettingsSwitcherManager;
use crate::ui::pages::programs::Programs;
use crate::ui::controls::navigation::Navigation;
use crate::ui::controls::settings_switcher::SettingsSwitcher;
use crate::ui::states::general_settings_state::GeneralSettingsState;
use crate::ui::controls::Control;
use crate::ui::pages::appearance::Appearance;
use crate::ui::pages::displays::Displays;
use crate::ui::pages::infos::Infos;
use crate::ui::pages::keybinds::Keybinds;
use crate::ui::pages::lockscreen::Lockscreen;
use crate::ui::pages::overview::Overview;
use crate::ui::pages::{
    APPEARANCE_PAGE, DISPLAY_PAGE, GENERAL_PAGE, INFO_PAGE, KEYBINDS_PAGE, LOCKSCREEN_PAGE,
    OVERVIEW_PAGE, STARTUP_PROGRAMS_PAGE, WALLPAPER_PAGE
};
use crate::ui::pages::startup_programs::StartupPrograms;
use crate::ui::pages::wallpaper::Wallpaper;
use crate::ui::states::display_settings_state::DisplaySettingsState;
use crate::ui::states::lockscreen_page_state::LockScreenPageState;
use crate::ui::states::settings_switcher_state::SettingsSwitcherState;
use crate::ui::states::wallpaper_page_state::WallpaperPageState;
use crate::ui::updatable_control::UpdatableControl;

pub struct App {
    app_box: GTKBox
}

impl Control for App {
    fn init_events(&self) {

    }

    fn get_widget(&self) -> &GTKBox {
        &self.app_box
    }
}

impl App {
    pub fn new() -> Self {
        let application_provider = ApplicationProvider::new();
        
        let app_box = BoxBuilder::new("app")
            .set_orientation(Orientation::Horizontal)
            .set_margin(DEFAULT_MARGIN)
            .build();

        let overview_settings = Box::new(Overview::new(application_provider.clone()));

        let state = GeneralSettingsState::from(&application_provider);
        let mut program_settings = Box::new(Programs::new(application_provider.clone()));
        program_settings.init_events();
        program_settings.update_ui(state);

        let state = DisplaySettingsState::from(&application_provider);
        let mut display_settings = Box::new(Displays::new(application_provider.clone()));
        display_settings.init_events();
        display_settings.update_ui(state);

        let state = WallpaperPageState::from(&application_provider);
        let mut wallpaper_settings = Box::new(Wallpaper::new(application_provider.clone()));
        wallpaper_settings.update_ui(state.clone());

        let state = LockScreenPageState::from(&application_provider);
        let mut lockscreen_settings = Box::new(Lockscreen::new(application_provider.clone()));
        lockscreen_settings.update_ui(state.clone());

        let appearance_settings = Box::new(Appearance::new(application_provider.clone()));
        appearance_settings.init_events();

        let keybinds_settings = Box::new(Keybinds::new(application_provider.clone()));
        keybinds_settings.init_events();

        let startup_program_settings = Box::new(StartupPrograms::new());
        startup_program_settings.init_events(application_provider.clone());
        startup_program_settings.init_ui(application_provider.clone());
        
        let info_settings = Box::new(Infos::new());

        let settings_switcher = Rc::new(RefCell::new(SettingsSwitcher::new()));
        settings_switcher.borrow_mut()
            .insert_control(OVERVIEW_PAGE.to_string(), overview_settings)
            .insert_control(GENERAL_PAGE.to_string(), program_settings)
            .insert_control(DISPLAY_PAGE.to_string(), display_settings)
            .insert_control(WALLPAPER_PAGE.to_string(), wallpaper_settings)
            .insert_control(LOCKSCREEN_PAGE.to_string(), lockscreen_settings)
            .insert_control(APPEARANCE_PAGE.to_string(), appearance_settings)
            .insert_control(KEYBINDS_PAGE.to_string(), keybinds_settings)
            .insert_control(STARTUP_PROGRAMS_PAGE.to_string(), startup_program_settings)
            .insert_control(INFO_PAGE.to_string(), info_settings);

        let settings_switcher_state = SettingsSwitcherState::new(OVERVIEW_PAGE.to_string());
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