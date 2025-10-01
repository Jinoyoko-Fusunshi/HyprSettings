#[derive(Clone, Default)]
pub struct SettingsSwitcherState {
    pub active_settings_name: String,
}

impl SettingsSwitcherState {
    pub fn new(active_settings_name: String) -> Self {
        Self {
            active_settings_name
        }
    }
}