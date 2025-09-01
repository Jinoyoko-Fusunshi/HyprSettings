use std::cell::RefCell;
use std::rc::Rc;
use crate::providers::hyprland_settings_provider::HyprlandSettingsProvider;
use crate::providers::module_provider::ModuleProvider;

#[derive(Clone)]
pub struct ApplicationProvider {
    settings_provider: Rc<RefCell<HyprlandSettingsProvider>>,
    module_provider: Rc<RefCell<ModuleProvider>>,
}

impl ApplicationProvider {
    pub fn new() -> Self {
        let module_provider = Rc::new(RefCell::new(ModuleProvider::new()));
        module_provider.borrow_mut().init();

        Self {
            settings_provider: Rc::new(RefCell::new(HyprlandSettingsProvider::new())),
            module_provider
        }
    }
    
    pub fn get_settings_provider(&self) -> Rc<RefCell<HyprlandSettingsProvider>> {
        self.settings_provider.clone()
    }

    pub fn get_module_provider(&self) -> Rc<RefCell<ModuleProvider>> {
        self.module_provider.clone()
    }
}