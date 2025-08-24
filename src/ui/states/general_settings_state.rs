use std::cell::RefCell;
use std::rc::Rc;
use crate::settings::settings_manager::SettingsManager;

pub struct GeneralSettingsState {
    pub hyprland_config_path: Option<String>,
    pub terminal_path: Option<String>,
    pub file_manager_path: Option<String>,
    pub quick_search_path: Option<String>,
    pub lock_screen_path: Option<String>,
    pub notification_handler_path: Option<String>,
}

impl From<&Rc<RefCell<SettingsManager>>> for GeneralSettingsState {
    fn from(value: &Rc<RefCell<SettingsManager>>) -> Self {
        Self {
            hyprland_config_path: value.borrow().get_hyprland_config_program_path(),
            terminal_path: value.borrow().get_terminal_program_path(),
            file_manager_path: value.borrow().get_files_program_path(),
            quick_search_path: value.borrow().get_quick_search_program_path(),
            lock_screen_path: value.borrow().get_lockscreen_program_path(),
            notification_handler_path: value.borrow().get_notifications_program_path(),
        }
    }
}