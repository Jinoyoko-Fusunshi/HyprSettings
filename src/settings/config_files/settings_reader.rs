use std::cell::RefCell;
use std::rc::Rc;
use crate::settings::hyprland_settings::HyprlandSettings;

pub trait SettingsReader {
    fn read_from_config(&mut self);
    fn apply_settings(&mut self, settings: &Rc<RefCell<HyprlandSettings>>);
}