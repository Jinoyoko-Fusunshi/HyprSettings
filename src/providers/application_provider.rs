use std::cell::RefCell;
use std::rc::Rc;
use crate::providers::hyprland_settings_provider::HyprlandSettingsProvider;

#[derive(Clone)]
pub struct ApplicationProvider {
    settings_provider: Rc<RefCell<HyprlandSettingsProvider>>
}

impl ApplicationProvider {
    pub fn new() -> Self {
        Self {
            settings_provider: Rc::new(RefCell::new(HyprlandSettingsProvider::new()))
        }
    }
    
    pub fn get_settings_provider(&self) -> Rc<RefCell<HyprlandSettingsProvider>> {
        self.settings_provider.clone()
    }
}