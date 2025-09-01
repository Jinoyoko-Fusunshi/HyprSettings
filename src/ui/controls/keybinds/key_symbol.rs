use gtk::{Label, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::controls::Control;

pub struct KeySymbol {
    key_symbol_box: gtk::Box
}

impl Control for KeySymbol {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.key_symbol_box
    }
}

impl KeySymbol {
    pub fn new(key_name: String) -> Self {
        let key_symbol_box = gtk::Box::new(Orientation::Vertical, 0);
        key_symbol_box.add_css_class("key-panel");

        let key_label = Label::new(Some(&key_name));
        key_label.add_css_class("key-panel");
        key_symbol_box.append(&key_label);

        Self {
            key_symbol_box
        }
    }
}