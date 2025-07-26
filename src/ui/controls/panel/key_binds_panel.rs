use std::cell::RefCell;
use std::rc::Rc;
use crate::ui::controls::panel::Panel;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct KeyBindsPanel {
    key_binds_panel: gtk::Box
}

impl KeyBindsPanel {
    pub fn new() -> Self {
        Self {
            key_binds_panel: gtk::Box::new(gtk::Orientation::Vertical, 0)
        }
    }
}

impl Panel for KeyBindsPanel {
    fn reload_settings(&self, settings: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.key_binds_panel
    }
}

impl Clone for KeyBindsPanel {
    fn clone(&self) -> Self {
        Self {
            key_binds_panel: self.key_binds_panel.clone()
        }
    }   
}