use crate::providers::application_provider::ApplicationProvider;

pub struct GeneralSettingsState {
    pub hyprland_config_path: Option<String>,
    pub terminal_path: Option<String>,
    pub file_manager_path: Option<String>,
    pub quick_search_path: Option<String>,
    pub lock_screen_path: Option<String>,
    pub notification_handler_path: Option<String>,
}

impl From<&ApplicationProvider> for GeneralSettingsState {
    fn from(value: &ApplicationProvider) -> Self {
        let settings_provider = value.get_settings_provider();
        let settings_provider_ref = settings_provider.borrow();

        Self {
            hyprland_config_path: settings_provider_ref.get_hyprland_config_program_path(),
            terminal_path: settings_provider_ref.get_terminal_program_path(),
            file_manager_path: settings_provider_ref.get_files_program_path(),
            quick_search_path: settings_provider_ref.get_quick_search_program_path(),
            lock_screen_path: settings_provider_ref.get_lockscreen_program_path(),
            notification_handler_path: settings_provider_ref.get_notifications_program_path(),
        }
    }
}