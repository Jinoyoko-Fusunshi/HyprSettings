use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, Entry, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt};
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::settings::key_binds::custom_keybind::CustomKeybind;
use crate::settings::key_binds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::controls::editable_control::EditableControl;
use crate::ui::controls::named_section::named_input_section::NamedInputSection;
use crate::ui::controls::panel::{Panel, key_binds_panel::key_bind_entry::KeyBindEntry};

#[derive(Clone)]
pub struct CustomKeyBindEntryFieldModel {
    previous_shortcut_name: String,
    shortcut_name: String,
    command: String,
    keybind_configuration: KeyBindConfiguration
}

impl CustomKeyBindEntryFieldModel {
    pub fn new(shortcut_name: String, custom_keybind: Option<CustomKeybind>) -> Self {
        let command = match custom_keybind.clone() {
            Some(keybind) => keybind.get_command().clone(),
            None => "".to_string()
        };

        let keybind_configuration = match custom_keybind.clone() {
            Some(keybind) => keybind.get_configuration().clone(),
            None => KeyBindConfiguration::new(Vec::new())
        };

        Self {
            previous_shortcut_name: shortcut_name.clone(),
            shortcut_name,
            command,
            keybind_configuration
        }
    }
}

#[derive(Clone)]
pub struct CustomKeyBindEntryField {
    key_bind_entry_box: gtk::Box,
    delete_button: Button,
    named_key_bind_entry_name: NamedInputSection,
    named_key_bind_entry_command: NamedInputSection,
    key_bind_entry: KeyBindEntry,
    model: Rc<RefCell<CustomKeyBindEntryFieldModel>>,
}

impl Panel for CustomKeyBindEntryField {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.key_bind_entry_box
    }
}

impl EditableControl for CustomKeyBindEntryField {
    fn enable_control(&self) {
        self.delete_button.set_sensitive(false);
        self.named_key_bind_entry_name.set_active(true);
        self.named_key_bind_entry_command.set_active(true);
        self.key_bind_entry.set_active(true);
    }

    fn disable_control(&self) {
        self.delete_button.set_sensitive(true);
        self.named_key_bind_entry_name.set_active(false);
        self.named_key_bind_entry_command.set_active(false);
        self.key_bind_entry.set_active(false);
    }
}

impl CustomKeyBindEntryField {
    pub fn new(
        shortcut_name: Option<String>,
        selected_keybind: Option<CustomKeybind>
    ) -> Self {
        let key_bind_entry_box = gtk::Box::new(Orientation::Horizontal, 10);
        key_bind_entry_box.set_height_request(56);

        let delete_button = Button::with_label("❌");
        delete_button.set_vexpand(false);
        delete_button.set_valign(Align::Center);

        let model = Rc::new(
            RefCell::new(
                CustomKeyBindEntryFieldModel::new(shortcut_name.unwrap_or("".to_string()), selected_keybind.clone())
            )
        );
        let named_key_bind_entry_name = NamedInputSection::new(
            "Shortcut name:", "Open program", Some(model.borrow().shortcut_name.clone())
        );

        let named_key_bind_entry_command = NamedInputSection::new(
            "Command:", "firefox", Some(model.borrow().command.clone()),
        );

        let configuration = match selected_keybind {
            Some(keybind) => Some(keybind.get_configuration().clone()),
            None => None
        };
        let key_bind_entry = KeyBindEntry::new(configuration);

        key_bind_entry_box.append(&delete_button);
        key_bind_entry_box.append(named_key_bind_entry_name.get_container_box());
        key_bind_entry_box.append(named_key_bind_entry_command.get_container_box());
        key_bind_entry_box.append(key_bind_entry.get_container_box());

        let custom_keybind_entry_field = Self {
            key_bind_entry_box,
            delete_button,
            named_key_bind_entry_name,
            named_key_bind_entry_command,
            key_bind_entry,
            model
        };

        custom_keybind_entry_field.init_events();
        custom_keybind_entry_field
    }

    pub fn init_events(&self) {
        let this = self.clone();
        let shortcut_name_entry_callback = move |entry: &Entry| {
            this.model.borrow_mut().shortcut_name = entry.text().to_string();
        };

        let this = self.clone();
        let command_entry_callback = move |entry: &Entry| {
            this.model.borrow_mut().command = entry.text().to_string();
        };

        let this = self.clone();
        let key_bind_entry_callback = move |keybind_configuration: KeyBindConfiguration| {
            this.model.borrow_mut().keybind_configuration = keybind_configuration.clone();
        };

        self.named_key_bind_entry_name.set_input_callback(shortcut_name_entry_callback);
        self.named_key_bind_entry_command.set_input_callback(command_entry_callback);
        self.key_bind_entry.set_input_callback(key_bind_entry_callback);
    }

    pub fn set_delete_button_callback(&self, delete_button_click_callback: impl Fn(&Button) + 'static) {
        self.delete_button.connect_clicked(delete_button_click_callback);
    }

    pub fn save_setting(&self, settings: Rc<RefCell<HyprlandSettings>>) {
        let mut model = self.model.borrow_mut();
        settings.borrow_mut().key_bind_settings
            .remove_custom_key_bind(model.previous_shortcut_name.clone());

        settings.borrow_mut().key_bind_settings.set_custom_key_bind(
            model.shortcut_name.clone(),
            CustomKeybind::new(
                model.command.clone(),
                model.keybind_configuration.clone()
            ),
        );

        model.previous_shortcut_name = model.shortcut_name.clone();
    }

    pub fn remove_setting(&self, settings: Rc<RefCell<HyprlandSettings>>) {
        let model_ref = self.model.borrow();
        let shortcut_name = model_ref.shortcut_name.clone();
        settings.borrow_mut().key_bind_settings.remove_custom_key_bind(shortcut_name);
    }
}