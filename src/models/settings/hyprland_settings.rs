use serde::{Deserialize, Serialize};
use crate::models::settings::appearance_settings::AppearanceSettings;
use crate::models::settings::display_settings::DisplaySettings;
use crate::models::settings::keybind_settings::KeyBindSettings;
use crate::models::settings::lockscreen_settings::LockScreenSettings;
use crate::models::settings::program_settings::ProgramSettings;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HyprlandSettings {
    pub program_settings: ProgramSettings,
    pub display_settings: DisplaySettings,
    pub appearance_settings: AppearanceSettings,
    pub keybind_settings: KeyBindSettings,
    pub lockscreen_settings: LockScreenSettings,
}

impl HyprlandSettings {
    pub fn new(
        program_settings: ProgramSettings, display_settings: DisplaySettings, 
        appearance_settings: AppearanceSettings, keybind_settings: KeyBindSettings, 
        lockscreen_settings: LockScreenSettings
    ) -> Self {
        Self {
            program_settings,
            display_settings,
            appearance_settings,
            keybind_settings,
            lockscreen_settings,
        }
    }
}