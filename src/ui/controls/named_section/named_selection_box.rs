use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, ComboBoxText, Label, Orientation};
use gtk::prelude::{BoxExt, ComboBoxExt, ComboBoxExtManual, WidgetExt};
use crate::ui::controls::panel::Panel;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct NamedSelectionBox {
    selection_box: gtk::Box,
    selection_box_label: Label,
}

impl Panel for NamedSelectionBox {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.selection_box
    }
}

impl NamedSelectionBox {
    pub fn new(
        label_text: &str, available_options: Vec<&str>, selected_option: Option<&str>,
        selection_changed_callback: Option<impl Fn(&ComboBoxText) + 'static>
    ) -> NamedSelectionBox {
        let selection_box = gtk::Box::new(Orientation::Horizontal, 10);

        let selection_box_label = Label::new(Some(label_text));
        selection_box_label.set_halign(Align::Start);
        selection_box_label.set_xalign(0.0);
        
        let combobox = ComboBoxText::new();
        for option in available_options.clone() {
            combobox.append_text(option);
        }

        let mut selected_option_index = 0;
        if let Some(option_text) = selected_option {
            let selected_index = available_options.iter().position(|&option| option == option_text);
            if let Some(index) = selected_index {
                selected_option_index = index as u32;
            }
        }

        combobox.set_active(Some(selected_option_index));
        if let Some(callback) = selection_changed_callback {
            combobox.connect_changed(callback);
        }

        selection_box.append(&selection_box_label);
        selection_box.append(&combobox);

        Self {
            selection_box,
            selection_box_label,
        }
    }

    pub fn set_label_width(&mut self, label_width: i32) {
        self.selection_box_label.set_width_request(label_width);
    }
}