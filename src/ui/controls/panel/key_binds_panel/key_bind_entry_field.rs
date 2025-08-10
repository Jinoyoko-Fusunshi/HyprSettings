use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Label, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::settings::key_binds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::controls::panel::key_binds_panel::key_bind_entry::KeyBindEntry;
use crate::ui::controls::panel::Panel;

pub struct KeyBindEntryField {
    key_binds_entry_field_box: gtk::Box,
    key_bind_entry: KeyBindEntry
}

impl Clone for KeyBindEntryField {
    fn clone(&self) -> Self {
        Self {
            key_binds_entry_field_box: self.key_binds_entry_field_box.clone(),
            key_bind_entry: self.key_bind_entry.clone()
        }
    }
}

impl Panel for KeyBindEntryField {
    fn reload_settings(&self, settings: &Rc<RefCell<HyprlandSettings>>) {
        self.key_bind_entry.reload_settings(settings);
    }

    fn get_container_box(&self) -> &gtk::Box {
        &self.key_binds_entry_field_box
    }
}

impl KeyBindEntryField {
    pub fn new(
        entry_name: String, selected_key_bind: Option<KeyBindConfiguration>
    ) -> Self {
        let key_binds_entry_field_box = gtk::Box::new(Orientation::Horizontal, 10);
        key_binds_entry_field_box.set_height_request(56);

        let entry_label = Label::new(Some(&entry_name));
        entry_label.set_xalign(0.0);
        entry_label.set_width_request(200);

        let key_bind_entry: KeyBindEntry = KeyBindEntry::new(selected_key_bind);

        key_binds_entry_field_box.append(&entry_label);
        key_binds_entry_field_box.append(key_bind_entry.get_container_box());

        Self {
            key_bind_entry,
            key_binds_entry_field_box
        }
    }

    pub fn set_input_callback(&self, callback: impl Fn(KeyBindConfiguration) + 'static) {
        self.key_bind_entry.set_input_callback(callback);
    }
}