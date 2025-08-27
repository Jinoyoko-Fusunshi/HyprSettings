use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Label, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::component::Component;
use crate::ui::controls::keybinds::keybind_input::KeybindInput;
use crate::ui::manager::keybind_input_manager::KeybindInputManager;
use crate::ui::states::keybind_input_field_state::KeybindInputFieldState;
use crate::ui::states::keybind_input_state::KeybindInputState;
use crate::ui::updatable_component::UpdatableComponent;

pub struct KeybindInputField {
    keybind_input_field_box: gtk::Box,
    keybind_input_label: Label,
    keybind_input: Rc<RefCell<KeybindInput>>
}

impl Component for KeybindInputField {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.keybind_input_field_box
    }
}

impl UpdatableComponent<KeybindInputFieldState> for KeybindInputField {
    fn update_ui(&mut self, state: KeybindInputFieldState) {
        self.keybind_input_label.set_text(state.input_text.as_str());

        let keybind_input_state = KeybindInputState {
            configuration: state.configuration.clone(),
        };

        self.keybind_input.borrow_mut().update_ui(keybind_input_state);
    }
}

impl KeybindInputField {
    pub fn new() -> Self {
        let keybind_input_field_box = gtk::Box::new(Orientation::Horizontal, 10);
        keybind_input_field_box.set_height_request(56);

        let keybind_input_label = Label::new(None);
        keybind_input_label.set_xalign(0.0);
        keybind_input_label.set_width_request(200);

        let keybind_input = Rc::new(RefCell::new(KeybindInput::new()));
        let keybind_input_manager = KeybindInputManager::new(keybind_input.clone());
        keybind_input.borrow().init_events();
        keybind_input.borrow().set_reset_button_click(
            keybind_input_manager.clone(), None::<fn()>
        );

        keybind_input_field_box.append(&keybind_input_label);
        keybind_input_field_box.append(keybind_input.borrow().get_widget());

        Self {
            keybind_input_label,
            keybind_input,
            keybind_input_field_box
        }
    }

    pub fn set_input_callback(&self, callback: impl Fn(KeyBindConfiguration) + 'static) {
        self.keybind_input.borrow().set_input_change(callback);
    }
}