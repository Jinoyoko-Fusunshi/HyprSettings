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

    fn get_widget(&self) -> &gtk::Box {
        &self.selection_box
    }
}

impl NamedSelectionBox {
    pub fn new(
        label_text: &str, options: Vec<&str>, 
        selection_changed_callback: Option<impl Fn(&ComboBoxText) + 'static>
    ) -> NamedSelectionBox {
        let selection_box = gtk::Box::new(Orientation::Horizontal, 10);

        let selection_box_label = Label::new(Some(label_text));
        selection_box_label.set_halign(Align::Start);
        selection_box_label.set_xalign(0.0);
        
        let combobox = ComboBoxText::new();
        for option in options {
            combobox.append_text(option);
        }
        combobox.set_active(Some(0));

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