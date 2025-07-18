use gtk::gdk::RGBA;
use crate::config::appearance_settings::AppearanceSettings;
use crate::monitor::monitor_setting::MonitorSetting;

#[derive(Debug)]
pub struct SettingsContainer {
    hyprland_config_path: String,
    virtual_terminal_program_path: String,
    file_manager_program_path: String,
    quick_search_program_path: String,
    lock_screen_program_path: String,
    notification_handler_program_path: String,
    monitor_settings: Vec<MonitorSetting>,
    appearance_settings: AppearanceSettings,
}

impl SettingsContainer {
    pub fn new() -> Self {
        let appearance_settings = AppearanceSettings::new(
            String::new(),
            false,
            false,
            0.0,
            0.0,
            0.0,
            RGBA::new(0.0, 0.0, 0.0, 0.0),
            RGBA::new(0.0, 0.0, 0.0, 0.0),
            false,
            false,
            0.0,
            0.0,
            false,
            0.0,
            0.0,
            false,
            0.0,
            0.0,
            RGBA::new(0.0, 0.0, 0.0, 0.0),
            false,
            0.0,
            0,
            0.0,
            String::new(),
            String::new(),
            false,
            false,
        );
        
        Self {
            hyprland_config_path: String::new(),
            virtual_terminal_program_path: String::new(),
            file_manager_program_path: String::new(),
            quick_search_program_path: String::new(),
            lock_screen_program_path: String::new(),
            notification_handler_program_path: String::new(),
            monitor_settings: Vec::new(),
            appearance_settings,
        }
    }

    pub fn set_hyprland_config_path(&mut self, hyprland_config_path: String) {
        self.hyprland_config_path = hyprland_config_path;
    }

    pub fn set_virtual_terminal_program_path(&mut self, virtual_terminal_program_path: String) {
        self.virtual_terminal_program_path = virtual_terminal_program_path;
    }

    pub fn set_file_manager_program_path(&mut self, file_manager_program_path: String) {
        self.file_manager_program_path = file_manager_program_path;
    }

    pub fn set_quick_search_program_path(&mut self, quick_search_program_path: String) {
        self.quick_search_program_path = quick_search_program_path;
    }

    pub fn set_lock_screen_program_path(&mut self, lock_screen_program_path: String) {
        self.lock_screen_program_path = lock_screen_program_path;
    }
    pub fn set_notification_handler_program_path(&mut self, notification_handler_program_path: String) {
        self.notification_handler_program_path = notification_handler_program_path;
    }

    pub fn set_monitor_settings(&mut self, monitor_information: Vec<MonitorSetting>) {
        self.monitor_settings = monitor_information;
    }

    pub fn set_monitor_status_by_index(&mut self, status: bool, index: usize) {
        self.monitor_settings[index].set_enabled(status);
    }

    pub fn set_monitor_width_resolution_by_index(&mut self, width: u32, index: usize) {
        self.monitor_settings[index].set_width_resolution(width);
    }

    pub fn set_monitor_height_resolution_by_index(&mut self, height: u32, index: usize) {
        self.monitor_settings[index].set_height_resolution(height);
    }

    pub fn set_monitor_refresh_rate_by_index(&mut self, rate: u32, index: usize) {
        self.monitor_settings[index].set_refresh_rate(rate);
    }

    pub fn get_hyprland_config_path(&self) -> &String {
        &self.hyprland_config_path
    }

    pub fn get_virtual_terminal_program_path(&self) -> &String {
        &self.virtual_terminal_program_path
    }

    pub fn get_file_manager_program_path(&self) -> &String {
        &self.file_manager_program_path
    }

    pub fn get_quick_search_program_path(&self) -> &String {
        &self.quick_search_program_path
    }

    pub fn get_lock_screen_program_path(&self) -> &String {
        &self.lock_screen_program_path
    }
    
    pub fn get_notification_handler_program_path(&mut self) -> &String {
        &self.notification_handler_program_path
    }

    pub fn get_monitor_settings(&mut self) -> &Vec<MonitorSetting> {
        &self.monitor_settings
    }

    pub fn get_appearance_settings(&mut self) -> &mut AppearanceSettings {
        &mut self.appearance_settings
    }
}