use serde::{Deserialize, Serialize};
use crate::models::settings::appearance_settings::AppearanceSettings;
use crate::models::settings::monitor_settings::MonitorSettings;
use crate::models::settings::input_settings::InputSettings;
use crate::models::settings::keybind_settings::KeyBindSettings;
use crate::models::settings::lockscreen_settings::LockScreenSettings;
use crate::models::settings::program_settings::ProgramSettings;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HyprlandSettings {
    pub program_settings: ProgramSettings,
    pub monitor_settings: MonitorSettings,
    pub appearance_settings: AppearanceSettings,
    pub input_settings: InputSettings,
    pub keybind_settings: KeyBindSettings,
    pub lockscreen_settings: LockScreenSettings,
}

impl HyprlandSettings {
    pub fn new(
        program_settings: ProgramSettings, monitor_settings: MonitorSettings,
        appearance_settings: AppearanceSettings, input_settings: InputSettings, 
        keybind_settings: KeyBindSettings, lockscreen_settings: LockScreenSettings
    ) -> Self {
        Self {
            program_settings,
            monitor_settings,
            appearance_settings,
            input_settings,
            keybind_settings,
            lockscreen_settings,
        }
    }
}