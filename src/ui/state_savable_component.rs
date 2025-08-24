use std::cell::RefCell;
use std::rc::Rc;
use crate::settings::settings_manager::SettingsManager;

pub trait StateSavableComponent {
    fn save_settings(&self, settings_manager: Rc<RefCell<SettingsManager>>);
    fn remove_settings(&self, settings_manager: Rc<RefCell<SettingsManager>>);
}