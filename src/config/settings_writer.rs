use std::cell::RefCell;
use std::rc::Rc;
use crate::hyprland_settings::HyprlandSettings;

pub struct SettingsWriter {
    settings: Rc<RefCell<HyprlandSettings>>
}

impl SettingsWriter {
    pub fn new(settings: Rc<RefCell<HyprlandSettings>>) -> Self {
        Self {
            settings
        }
    }

    pub fn write(&self) {
        todo!("Implement writing of serialized settings values")
    }
}