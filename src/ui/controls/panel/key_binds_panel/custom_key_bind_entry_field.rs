use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, Entry, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::ui::controls::named_section::named_input_section::NamedInputSection;
use crate::ui::controls::panel::{Panel, key_binds_panel::key_bind_entry::KeyBindEntry};

pub struct CustomKeyBindEntryField {
    key_binds_entry_box: gtk::Box
}

impl Panel for CustomKeyBindEntryField {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.key_binds_entry_box
    }
}

impl CustomKeyBindEntryField {
    pub fn new(parent_box: &gtk::Box) -> Self {
        let custom_key_bind_entry_box = gtk::Box::new(Orientation::Horizontal, 10);
        custom_key_bind_entry_box.set_height_request(56);

        let parent_box_clone = parent_box.clone();
        let custom_key_binds_entry_box_clone = custom_key_bind_entry_box.clone();
        let cancel_button_callback = move |_: &Button| {
            parent_box_clone.remove(&custom_key_binds_entry_box_clone);
        };

        let named_key_bind_entry_name = NamedInputSection::new(
            "Shortcut name:", "Open program", None::<fn(&Entry)>);
        let named_key_bind_entry_command = NamedInputSection::new(
            "Command:", "firefox", None::<fn(&Entry)>);

        let cancel_button = Button::with_label("❌");
        cancel_button.set_vexpand(false);
        cancel_button.set_valign(Align::Center);
        cancel_button.connect_clicked(cancel_button_callback);

        let key_bind_entry = KeyBindEntry::new();
        custom_key_bind_entry_box.append(&cancel_button);
        custom_key_bind_entry_box.append(named_key_bind_entry_name.get_widget());
        custom_key_bind_entry_box.append(named_key_bind_entry_command.get_widget());
        custom_key_bind_entry_box.append(key_bind_entry.get_widget());

        Self {
            key_binds_entry_box: custom_key_bind_entry_box
        }
    }
}