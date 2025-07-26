use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, ColorButton, Label};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::controls::panel::Panel;
use crate::hyprland_settings::HyprlandSettings;

pub struct NamedColorButtonSection {
    color_button_box: gtk::Box,
    color_button_label: Label,
}

impl Panel for NamedColorButtonSection {
    fn reload_settings(&self, settings: &Rc<RefCell<HyprlandSettings>>) {

    }

    fn get_widget(&self) -> &gtk::Box {
        &self.color_button_box
    }
}

impl NamedColorButtonSection {
    pub fn new(label_text: &str, color_changed_callback: impl Fn(&ColorButton) + 'static) -> NamedColorButtonSection {
        let color_button_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        
        let color_button_label = Label::new(Some(label_text));
        color_button_label.set_halign(Align::Start);
        color_button_label.set_xalign(0.0);
        
        let color_button = ColorButton::new();
        color_button.connect_color_set(color_changed_callback);
        
        color_button_box.append(&color_button_label);
        color_button_box.append(&color_button);
        
        Self {
            color_button_box,
            color_button_label
        }
    }
    
    pub fn set_label_width(&mut self, label_width: i32) {
        self.color_button_label.set_width_request(label_width);
    }
}