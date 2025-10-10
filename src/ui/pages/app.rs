use std::cell::RefCell;
use std::rc::Rc;
use gtk::Orientation;
use gtk::prelude::BoxExt;
use crate::providers::application_provider::ApplicationProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::manager::settings_switcher_manager::SettingsSwitcherManager;
use crate::ui::pages::user_programs::UserPrograms;
use crate::ui::controls::navigation::Navigation;
use crate::ui::controls::settings_switcher::SettingsSwitcher;
use crate::ui::states::programs_state::ProgramsState;
use crate::ui::controls::Control;
use crate::ui::managed_control::ManagedControl;
use crate::ui::manager::input_manager::InputManager;
use crate::ui::pages::appearance::Appearance;
use crate::ui::pages::monitors::Monitors;
use crate::ui::pages::infos::Infos;
use crate::ui::pages::keybinds::Keybinds;
use crate::ui::pages::lockscreen::Lockscreen;
use crate::ui::pages::overview::Overview;
use crate::ui::pages::{APPEARANCE_PAGE, MONITORS_PAGE, USER_PROGRAMS_PAGE, INFO_PAGE, INPUT_PAGE, KEYBINDS_PAGE, LOCKSCREEN_PAGE, OVERVIEW_PAGE, STARTUP_PROGRAMS_PAGE, WALLPAPER_PAGE};
use crate::ui::pages::input::Input;
use crate::ui::pages::startup_programs::StartupPrograms;
use crate::ui::pages::wallpaper::Wallpaper;
use crate::ui::states::monitors_state::MonitorsState;
use crate::ui::states::input_state::InputState;
use crate::ui::states::lockscreen_page_state::LockScreenPageState;
use crate::ui::states::settings_switcher_state::SettingsSwitcherState;
use crate::ui::states::wallpaper_page_state::WallpaperPageState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::new_rc_mut;

pub struct App {
    app_box: GTKBox
}

impl Control for App {
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

        let overview_settings = new_rc_mut(Overview::new(application_provider.clone()));

        let state = ProgramsState::from(&application_provider);
        let user_programs = new_rc_mut(UserPrograms::new(application_provider.clone()));
        user_programs.borrow_mut().update_state(state);

        let state = MonitorsState::from(&application_provider);
        let monitors = new_rc_mut(Monitors::new(application_provider.clone()));
        monitors.borrow_mut().update_state(state);

        let state = WallpaperPageState::from(&application_provider);
        let wallpaper = new_rc_mut(Wallpaper::new(application_provider.clone()));
        wallpaper.borrow_mut().update_state(state.clone());

        let state = LockScreenPageState::from(&application_provider);
        let lockscreen = new_rc_mut(Lockscreen::new(application_provider.clone()));
        lockscreen.borrow_mut().update_state(state.clone());

        let appearance = new_rc_mut(Appearance::new(application_provider.clone()));

        let state = InputState::from(&application_provider);
        let input = new_rc_mut(Input::new(application_provider.clone()));
        input.borrow_mut().update_state(state.clone());

        let input_manager = InputManager::new(input.clone());
        input.borrow_mut().init_events_by_manager(input_manager);

        let keybinds = new_rc_mut(Keybinds::new(application_provider.clone()));

        let startup_program = new_rc_mut(StartupPrograms::new());
        startup_program.borrow_mut().init_events(application_provider.clone());
        startup_program.borrow_mut().init_ui(application_provider.clone());
        
        let infos = new_rc_mut(Infos::new());
        let settings_switcher = Rc::new(RefCell::new(SettingsSwitcher::new()));
        settings_switcher.borrow_mut()
            .insert_control(OVERVIEW_PAGE.to_string(), overview_settings)
            .insert_control(USER_PROGRAMS_PAGE.to_string(), user_programs)
            .insert_control(MONITORS_PAGE.to_string(), monitors)
            .insert_control(WALLPAPER_PAGE.to_string(), wallpaper)
            .insert_control(LOCKSCREEN_PAGE.to_string(), lockscreen)
            .insert_control(APPEARANCE_PAGE.to_string(), appearance)
            .insert_control(KEYBINDS_PAGE.to_string(), keybinds)
            .insert_control(STARTUP_PROGRAMS_PAGE.to_string(), startup_program)
            .insert_control(INPUT_PAGE.to_string(), input)
            .insert_control(INFO_PAGE.to_string(), infos);

        let settings_switcher_state = SettingsSwitcherState::new(OVERVIEW_PAGE.to_string());
        settings_switcher.borrow_mut().update_state(settings_switcher_state);
        
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