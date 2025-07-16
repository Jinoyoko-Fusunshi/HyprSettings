use std::cell::RefCell;
use std::rc::Rc;
use crate::settings_container::SettingsContainer;

pub struct SettingsWriter {
    settings: Rc<RefCell<SettingsContainer>>
}

impl SettingsWriter {
    pub fn new(settings: Rc<RefCell<SettingsContainer>>) -> Self {
        Self {
            settings
        }
    }

    pub fn write(&self) {
        todo!("Implement writing of serialized settings values")
    }
}