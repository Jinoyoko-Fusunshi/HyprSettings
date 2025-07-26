use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Adjustment, Align, Label, Orientation, SpinButton};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::controls::panel::Panel;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct NamedSpinButtonSection {
    spin_button_box: gtk::Box,
    spin_button_label: Label,
}

impl Panel for NamedSpinButtonSection {
    fn reload_settings(&self, settings: &Rc<RefCell<HyprlandSettings>>) {

    }

    fn get_widget(&self) -> &gtk::Box {
        &self.spin_button_box
    }
}

impl NamedSpinButtonSection {
    pub fn new(
        label_text: &str,
        min_value: f64, max_value: f64, current_value: f64, increment_value: f64,
        page_increment_value: f64, page_size: f64, climb_rate: f64, digit_count: u32,
        use_integral_numbers: bool,
        spin_button_change_callback: Option<impl Fn(&SpinButton) + 'static>
    ) -> NamedSpinButtonSection {
        let spin_button_box = gtk::Box::new(Orientation::Horizontal, 10);

        let adjustment = Adjustment::new(
            current_value,
            min_value,
            max_value,
            increment_value,
            page_increment_value,
            page_size,
        );
        let spin_button = SpinButton::new(
            Some(&adjustment), climb_rate, digit_count,
        );
        spin_button.set_numeric(use_integral_numbers);
        spin_button.set_wrap(true);
        
        if let Some(callback) = spin_button_change_callback {
            spin_button.connect_value_changed(callback);   
        }

        let spin_button_label = Label::new(Some(label_text));
        spin_button_label.set_halign(Align::Start);
        spin_button_label.set_xalign(0.0);
        
        spin_button_box.append(&spin_button_label);
        spin_button_box.append(&spin_button);
        
        Self {
            spin_button_box,
            spin_button_label,
        }
    }
    
    pub fn set_label_width(&mut self, label_width: i32) {
        self.spin_button_label.set_width_request(label_width);   
    }
}