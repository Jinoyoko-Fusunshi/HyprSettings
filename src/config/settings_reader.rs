use crate::settings_container::SettingsContainer;

pub struct SettingsReader<'a> {
    settings: &'a mut SettingsContainer
}

impl<'a> SettingsReader<'a> {
    pub fn new(&mut self, settings: &'a mut SettingsContainer) -> Self {
        Self {
            settings
        }
    }
    
    pub fn read(&mut self) {
        todo!("Implement reading of deserialized settings values")
    }
}