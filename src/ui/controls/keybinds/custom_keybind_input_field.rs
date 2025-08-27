use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, Entry, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt};
use crate::settings::keybinds::custom_keybind::CustomKeybind;
use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::settings::settings_manager::SettingsManager;
use crate::ui::component::Component;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::input_field::{InputField, InputFieldState};
use crate::ui::manager::keybind_input_manager::KeybindInputManager;
use crate::ui::controls::keybinds::keybind_input::KeybindInput;
use crate::ui::statable_component::StatableComponent;
use crate::ui::state_savable_component::StateSavableComponent;
use crate::ui::states::custom_keybind_input_field_state::CustomKeybindInputFieldState;
use crate::ui::states::keybind_input_state::KeybindInputState;
use crate::ui::updatable_component::UpdatableComponent;

pub struct CustomKeyBindInputField {
    state: Rc<RefCell<CustomKeybindInputFieldState>>,
    key_bind_entry_box: gtk::Box,
    delete_button: Button,
    shortcut_input_field: InputField,
    command_input_field: InputField,
    keybind_input: Rc<RefCell<KeybindInput>>,
}

impl Component for CustomKeyBindInputField {
    fn init_events(&self) {
        let state = self.state.clone();
        let shortcut_name_entry_callback = move |entry: &Entry| {
            state.borrow_mut().shortcut_name = Some(entry.text().to_string());
        };

        let state = self.state.clone();
        let command_entry_callback = move |entry: &Entry| {
            state.borrow_mut().command = Some(entry.text().to_string());
        };

        let state = self.state.clone();
        let keybind_input_change = move |keybind_configuration: KeyBindConfiguration| {
            state.borrow_mut().keybind = Some(keybind_configuration);
        };

        self.shortcut_input_field.set_input_callback(shortcut_name_entry_callback);
        self.command_input_field.set_input_callback(command_entry_callback);
        self.keybind_input.borrow().set_input_change(keybind_input_change);
    }

    fn get_widget(&self) -> &gtk::Box {
        &self.key_bind_entry_box
    }
}

impl UpdatableComponent<CustomKeybindInputFieldState> for CustomKeyBindInputField {
    fn update_ui(&mut self, state: CustomKeybindInputFieldState) {
        let input_field_state = InputFieldState {
            label_text: "Shortcut name:".to_string(),
            entry_text: state.shortcut_name,
            placeholder_text: "Open program".to_string(),
        };
        self.shortcut_input_field.update_ui(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "Command:".to_string(),
            entry_text: state.command,
            placeholder_text: "Command:".to_string(),
        };
        self.command_input_field.update_ui(input_field_state);

        let keybind_input_state = KeybindInputState {
            configuration: state.keybind,
        };
        self.keybind_input.borrow_mut().update_ui(keybind_input_state);
    }
}

impl StatableComponent<CustomKeybindInputFieldState> for CustomKeyBindInputField {
    fn update_state(&mut self, state: CustomKeybindInputFieldState) {
        *self.state.borrow_mut() = state;
    }
}

impl StateSavableComponent for CustomKeyBindInputField {
    fn save_settings(&self, settings_manager: Rc<RefCell<SettingsManager>>) {
        let mut settings_manager_mut = settings_manager.borrow_mut();
        let mut state_mut = self.state.borrow_mut();
        if let Some(name) = state_mut.previous_shortcut_name.clone() {
            settings_manager_mut.remove_custom_keybind(name)
        }

        if let Some(name) = state_mut.shortcut_name.clone() {
            if let Some(command) = state_mut.command.clone() {
                if let Some(keybind) = state_mut.keybind.clone() {
                    let custom_keybind = CustomKeybind::new(command, keybind);
                    settings_manager_mut.set_custom_keybind(state_mut.shortcut_name.clone(), Some(custom_keybind));
                    state_mut.previous_shortcut_name = Some(name);
                }
            }
        }
    }

    fn remove_settings(&self, settings_manager: Rc<RefCell<SettingsManager>>) {
        let mut settings_manager_mut = settings_manager.borrow_mut();
        let state_ref = self.state.borrow();
        if let Some(name) = state_ref.previous_shortcut_name.clone() {
            settings_manager_mut.remove_custom_keybind(name)
        }
    }
}

impl ActivableControl for CustomKeyBindInputField {
    fn enable_control(&self) {
        self.delete_button.set_sensitive(false);
        self.shortcut_input_field.enable_control();
        self.command_input_field.enable_control();
        self.keybind_input.borrow().enable_control();
    }

    fn disable_control(&self) {
        self.delete_button.set_sensitive(true);
        self.shortcut_input_field.disable_control();
        self.command_input_field.disable_control();
        self.keybind_input.borrow().disable_control();
    }
}

impl CustomKeyBindInputField {
    pub fn new() -> Self {
        let state = Rc::new(RefCell::new(CustomKeybindInputFieldState {
            previous_shortcut_name: None,
            shortcut_name: None,
            keybind: None,
            command: None,
        }));

        let key_bind_entry_box = gtk::Box::new(Orientation::Horizontal, 10);
        key_bind_entry_box.set_height_request(56);

        let delete_button = Button::with_label("❌");
        delete_button.set_vexpand(false);
        delete_button.set_valign(Align::Center);

        let shortcut_input_field = InputField::new();
        let command_input_field = InputField::new();

        let keybind_input = Rc::new(RefCell::new(KeybindInput::new()));
        keybind_input.borrow().init_events();

        let keybind_input_manager = KeybindInputManager::new(keybind_input.clone());

        let state_clone = state.clone();
        let reset_button_action = move || {
            state_clone.borrow_mut().keybind = None;
        };
        keybind_input.borrow().set_reset_button_click(
            keybind_input_manager, Some(reset_button_action)
        );

        key_bind_entry_box.append(&delete_button);
        key_bind_entry_box.append(shortcut_input_field.get_widget());
        key_bind_entry_box.append(command_input_field.get_widget());
        key_bind_entry_box.append(keybind_input.borrow().get_widget());

        Self {
            state,
            key_bind_entry_box,
            delete_button,
            shortcut_input_field,
            command_input_field,
            keybind_input,
        }
    }

    pub fn set_delete_button_callback(&self, delete_button_click_callback: impl Fn(&Button) + 'static) {
        self.delete_button.connect_clicked(delete_button_click_callback);
    }
}