use crate::settings::hyprland_settings::HyprlandSettings;

pub struct SettingsReader<'a> {
    settings: &'a mut HyprlandSettings
}

impl<'a> SettingsReader<'a> {
    pub fn new(&mut self, settings: &'a mut HyprlandSettings) -> Self {
        Self {
            settings
        }
    }
    
    pub fn read(&mut self) {
        todo!("Implement reading of deserialized settings values")
    }
}