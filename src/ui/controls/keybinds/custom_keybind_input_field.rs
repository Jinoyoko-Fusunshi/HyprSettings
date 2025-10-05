use gtk::{Align, Button, Entry, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt};
use crate::models::keybinds::custom_keybind::CustomKeybind;
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::providers::application_provider::ApplicationProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::input_field::InputField;
use crate::ui::manager::keybind_input_manager::KeybindInputManager;
use crate::ui::controls::keybinds::keybind_input::KeybindInput;
use crate::ui::state_savable_control::StateSavableControl;
use crate::ui::states::custom_keybind_input_field_state::CustomKeybindInputFieldState;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::states::keybind_input_state::KeybindInputState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::{new_rc_mut, RcMut};

pub struct CustomKeyBindInputField {
    state: RcMut<CustomKeybindInputFieldState>,
    key_bind_entry_box: GTKBox,
    delete_button: Button,
    shortcut_input_field: InputField,
    command_input_field: InputField,
    keybind_input: RcMut<KeybindInput>,
}

impl Control for CustomKeyBindInputField {
    fn get_widget(&self) -> &GTKBox {
        &self.key_bind_entry_box
    }
}

impl UpdatableControl<CustomKeybindInputFieldState> for CustomKeyBindInputField {
    fn update_state(&mut self, state: CustomKeybindInputFieldState) {
        let input_field_state = InputFieldState {
            label_text: "Shortcut name:".to_string(),
            entry_text: state.shortcut_name.clone(),
            placeholder_text: "Open program".to_string(),
        };
        self.shortcut_input_field.update_state(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "Command:".to_string(),
            entry_text: state.command.clone(),
            placeholder_text: "Command:".to_string(),
        };
        self.command_input_field.update_state(input_field_state);

        let keybind_input_state = KeybindInputState {
            configuration: state.keybind.clone(),
        };
        
        self.keybind_input.borrow_mut().update_state(keybind_input_state);
        *self.state.borrow_mut() = state.clone();
    }

    fn get_current_state(&self) -> CustomKeybindInputFieldState {
        self.state.borrow().clone()
    }
}

impl StateSavableControl for CustomKeyBindInputField {
    fn save_settings(&self, application_provider: ApplicationProvider) {
        let keybinds_provider = application_provider.get_keybinds_provider();
        let mut keybinds_provider_mut = keybinds_provider.borrow_mut();
        let mut state_mut = self.state.borrow_mut();
        if let Some(name) = state_mut.previous_shortcut_name.clone() {
            keybinds_provider_mut.remove_custom_keybind(name)
        }

        if let Some(name) = state_mut.shortcut_name.clone() {
            if let Some(command) = state_mut.command.clone() {
                if let Some(keybind) = state_mut.keybind.clone() {
                    let custom_keybind = CustomKeybind::new(command, keybind);
                    keybinds_provider_mut.set_custom_keybind(state_mut.shortcut_name.clone(), Some(custom_keybind));
                    state_mut.previous_shortcut_name = Some(name);
                }
            }
        }
    }

    fn remove_settings(&self, application_provider: ApplicationProvider) {
        let keybinds_provider = application_provider.get_keybinds_provider();
        let mut keybinds_provider_mut = keybinds_provider.borrow_mut();
        let state_ref = self.state.borrow();
        if let Some(name) = state_ref.previous_shortcut_name.clone() {
            keybinds_provider_mut.remove_custom_keybind(name)
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
        let state = new_rc_mut(CustomKeybindInputFieldState {
            previous_shortcut_name: None,
            shortcut_name: None,
            keybind: None,
            command: None,
        });

        let key_bind_entry_box = BoxBuilder::new("custom-keybind-input-field")
            .set_orientation(Orientation::Horizontal)
            .build();

        let delete_button = Button::with_label("❌");
        delete_button.set_vexpand(false);
        delete_button.set_valign(Align::Center);

        let shortcut_input_field = InputField::new();
        shortcut_input_field.get_widget().set_valign(Align::End);

        let state_clone = state.clone();
        shortcut_input_field.set_input_callback(move |entry: &Entry| {
            state_clone.borrow_mut().shortcut_name = Some(entry.text().to_string());
        });

        let command_input_field = InputField::new();
        command_input_field.get_widget().set_valign(Align::End);

        let state_clone = state.clone();
        command_input_field.set_input_callback(move |entry: &Entry| {
            state_clone.borrow_mut().command = Some(entry.text().to_string());
        });

        let keybind_input = new_rc_mut(KeybindInput::new());

        let state_clone = state.clone();
        keybind_input.borrow().set_input_change(move |keybind_configuration: KeyBindConfiguration| {
            state_clone.borrow_mut().keybind = Some(keybind_configuration);
        });

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