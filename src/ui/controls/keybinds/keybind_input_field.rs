use gtk::{Label, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::component::Component;
use crate::ui::controls::keybinds::keybind_input::KeybindInput;

pub struct KeybindInputField {
    keybind_input_field_box: gtk::Box,
    keybind_input: KeybindInput
}

impl Component for KeybindInputField {
    fn init_events(&self) {

    }

    fn get_widget(&self) -> &gtk::Box {
        &self.keybind_input_field_box
    }
}

impl KeybindInputField {
    pub fn new(
        entry_name: String, selected_keybind: Option<KeyBindConfiguration>
    ) -> Self {
        let keybind_input_field_box = gtk::Box::new(Orientation::Horizontal, 10);
        keybind_input_field_box.set_height_request(56);

        let entry_label = Label::new(Some(&entry_name));
        entry_label.set_xalign(0.0);
        entry_label.set_width_request(200);

        let keybind_input = KeybindInput::new();
        if let Some(configuration) = selected_keybind {
            keybind_input.set_keybind(configuration);
        }

        keybind_input_field_box.append(&entry_label);
        keybind_input_field_box.append(keybind_input.get_widget());

        Self {
            keybind_input,
            keybind_input_field_box
        }
    }

    pub fn set_input_callback(&self, callback: impl Fn(KeyBindConfiguration) + 'static) {
        self.keybind_input.set_input_change(callback);
    }
}