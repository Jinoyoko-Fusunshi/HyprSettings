pub struct SettingsContainer {
    hyprland_config_path: String,
    virtual_terminal_program_path: String,
    file_manager_program_path: String,
    quick_search_program_path: String,
    lock_screen_program_path: String,
    notification_handler_program_path: String,
}

impl SettingsContainer {
    pub fn new() -> Self {
        Self {
            hyprland_config_path: String::new(),
            virtual_terminal_program_path: String::new(),
            file_manager_program_path: String::new(),
            quick_search_program_path: String::new(),
            lock_screen_program_path: String::new(),
            notification_handler_program_path: String::new(),
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
    
    pub fn get_notification_handler_program_path(&self) -> &String {
        &self.notification_handler_program_path
    }
}