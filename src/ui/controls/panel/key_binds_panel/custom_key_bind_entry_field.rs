use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, Entry, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt};
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::settings::key_binds::custom_keybind::CustomKeybind;
use crate::settings::key_binds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::controls::named_section::named_input_section::NamedInputSection;
use crate::ui::controls::panel::{Panel, key_binds_panel::key_bind_entry::KeyBindEntry};
use crate::ui::controls::panel::key_binds_panel::custom_key_bind_entry_field::CustomKeyBindMode::Edit;

#[derive(Clone)]
pub enum CustomKeyBindMode {
    Locked,
    Edit,
}

#[derive(Clone)]
pub struct CustomKeyBindEntryFieldModel {
    previous_shortcut_name: String,
    shortcut_name: String,
    command: String,
    keybind_configuration: KeyBindConfiguration,
    mode: CustomKeyBindMode,
}

impl CustomKeyBindEntryFieldModel {
    pub fn new(
        shortcut_name: String, custom_keybind: Option<CustomKeybind>, mode: CustomKeyBindMode,
    ) -> Self {
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
            keybind_configuration,
            mode
        }
    }
}

#[derive(Clone)]
pub struct CustomKeyBindEntryField {
    settings: Rc<RefCell<HyprlandSettings>>,
    parent_box: gtk::Box,
    key_bind_entry_box: gtk::Box,
    cancel_button: Button,
    named_key_bind_entry_name: NamedInputSection,
    named_key_bind_entry_command: NamedInputSection,
    key_bind_entry: KeyBindEntry,
    toggle_action_button: Button,
    model: Rc<RefCell<CustomKeyBindEntryFieldModel>>,
}

impl Panel for CustomKeyBindEntryField {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.key_bind_entry_box
    }
}

impl CustomKeyBindEntryField {
    pub fn new(
        parent_box: &gtk::Box, settings: &Rc<RefCell<HyprlandSettings>>,
        shortcut_name: Option<String>,
        selected_keybind: Option<CustomKeybind>,
        mode: CustomKeyBindMode,
    ) -> Self {
        let key_bind_entry_box = gtk::Box::new(Orientation::Horizontal, 10);
        key_bind_entry_box.set_height_request(56);

        let cancel_button = Button::with_label("🗑️");
        cancel_button.set_vexpand(false);
        cancel_button.set_valign(Align::Center);

        let model = Rc::new(RefCell::new(CustomKeyBindEntryFieldModel::new(
            shortcut_name.unwrap_or("".to_string()), selected_keybind.clone(), mode.clone()
        )));
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
        let toggle_action_button_label = match mode {
            CustomKeyBindMode::Locked => "✏️",
            Edit => "✅"
        };
        let toggle_action_button = Button::with_label(toggle_action_button_label);
        toggle_action_button.set_vexpand(false);
        toggle_action_button.set_valign(Align::Center);

        key_bind_entry_box.append(&cancel_button);
        key_bind_entry_box.append(named_key_bind_entry_name.get_container_box());
        key_bind_entry_box.append(named_key_bind_entry_command.get_container_box());
        key_bind_entry_box.append(key_bind_entry.get_container_box());
        key_bind_entry_box.append(&toggle_action_button);

        let custom_keybind_entry_field = Self {
            settings: settings.clone(),
            parent_box: parent_box.clone(),
            key_bind_entry_box,
            cancel_button,
            named_key_bind_entry_name,
            named_key_bind_entry_command,
            key_bind_entry,
            toggle_action_button,
            model
        };

        match mode {
            CustomKeyBindMode::Locked => custom_keybind_entry_field.set_fields_active(false),
            Edit => custom_keybind_entry_field.set_fields_active(true)
        }

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

        let this = self.clone();
        let cancel_button_callback = move |_: &Button| {
            this.parent_box.remove(&this.key_bind_entry_box);
            this.settings.borrow_mut().key_bind_settings
                .remove_custom_key_bind(this.model.borrow().shortcut_name.clone());
        };
        this.cancel_button.connect_clicked(cancel_button_callback);

        let this = self.clone();
        let toggle_action_button_callback = move |_: &Button| {
            let mut model = this.model.borrow_mut();
            match model.mode {
                CustomKeyBindMode::Locked => {
                    this.toggle_action_button.set_label("✅");
                    model.mode = Edit;
                    this.set_fields_active(true);
                },
                Edit => {
                    this.toggle_action_button.set_label("✏️");
                    model.mode = CustomKeyBindMode::Locked;
                    this.set_fields_active(false);

                    this.settings.borrow_mut().key_bind_settings
                        .remove_custom_key_bind(model.previous_shortcut_name.clone());

                    this.settings.borrow_mut().key_bind_settings.set_custom_key_bind(
                        model.shortcut_name.clone(),
                        CustomKeybind::new(
                            model.command.clone(),
                            model.keybind_configuration.clone()
                        ),
                    );

                    model.previous_shortcut_name = model.shortcut_name.clone();
                }
            }
        };

        self.toggle_action_button.connect_clicked(toggle_action_button_callback);
    }

    pub fn set_fields_active(&self, state: bool) {
        self.cancel_button.set_sensitive(!state);
        self.named_key_bind_entry_name.set_active(state);
        self.named_key_bind_entry_command.set_active(state);
        self.key_bind_entry.set_active(state);
    }
}