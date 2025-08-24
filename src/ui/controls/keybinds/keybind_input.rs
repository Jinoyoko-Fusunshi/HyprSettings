use gtk::{Align, Button, EventControllerKey, GestureClick, Label, Orientation};
use gtk::gdk::{Key, ModifierType};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::component::Component;
use crate::ui::manager::keybind_input_manager::{KeybindInputEvent, KeybindInputManager};
use crate::ui::controls::keybinds::key_symbol::KeySymbol;
use crate::ui::states::keybind_input_state::KeybindInputState;
use crate::ui::updatable_component::UpdatableComponent;

pub struct KeybindInput {
    keybind_input_box: gtk::Box,
    keybind_symbols_box: gtk::Box,
    reset_key_button: Button,
}

impl Component for KeybindInput {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.keybind_input_box
    }
}

impl UpdatableComponent<KeybindInputState> for KeybindInput {
    fn update_ui(&mut self, state: KeybindInputState) {
        if let Some(configuration) = state.configuration {
            self.set_keybind(configuration);
        } else {
            self.reset_input();
        }
    }
}

impl KeybindInput {
    pub fn new() -> Self {
        let keybind_input_box = gtk::Box::new(Orientation::Horizontal, 10);
        let keybind_symbols_box = Self::create_keybind_symbols_box();

        let reset_key_button = Button::with_label("➖");
        reset_key_button.set_valign(Align::Center);
        reset_key_button.set_vexpand(false);

        keybind_input_box.append(&keybind_symbols_box);
        keybind_input_box.append(&reset_key_button);

        Self {
            keybind_input_box,
            keybind_symbols_box,
            reset_key_button
        }
    }

    pub fn set_input_change(&self, input_change: impl Fn(KeyBindConfiguration) + 'static) {
        let key_input_controller = EventControllerKey::new();
        let key_pressed_callback = Self::create_key_bind_change_callback(self.keybind_symbols_box.clone(), input_change);
        key_input_controller.connect_key_pressed(key_pressed_callback);
        self.keybind_symbols_box.add_controller(key_input_controller);
    }

    pub fn set_reset_button_click(
        &self, keybind_input_manager: KeybindInputManager, input_cleared: impl Fn() + 'static
    ) {
        let reset_button_click = Self::create_reset_button_click(keybind_input_manager, input_cleared);
        self.reset_key_button.connect_clicked(reset_button_click);
    }

    pub fn set_keybind(&self, keybind_configuration: KeyBindConfiguration) {
        Self::create_key_bind_display(
            self.keybind_symbols_box.clone(), keybind_configuration.get_key_names().clone()
        );
    }

    pub fn clear_input(&self) {
        Self::clear_input_box(&self.keybind_symbols_box)
    }

    pub fn reset_input(&self) {
        const INITIAL_TEXT: &str = "Click to bind key";

        self.clear_input();
        let label = Label::new(Some(INITIAL_TEXT));
        label.set_hexpand(true);
        label.set_xalign(0.5);

        self.keybind_symbols_box.append(&label);
    }

    pub fn set_active(&self, active: bool) {
        self.keybind_input_box.set_sensitive(active);
    }

    fn clear_input_box(input_box: &gtk::Box) {
        let mut current_child = input_box.first_child();
        while let Some(child) = current_child {
            current_child = child.next_sibling();
            input_box.remove(&child);
        }
    }

    fn create_keybind_symbols_box() -> gtk::Box {
        let key_bind_input_box = gtk::Box::new(Orientation::Horizontal, 10);
        key_bind_input_box.add_css_class("key-binds-entry");
        key_bind_input_box.set_width_request(250);
        key_bind_input_box.set_hexpand(false);
        key_bind_input_box.set_focusable(true);
        key_bind_input_box.set_can_focus(true);

        let key_bind_input_box_clone = key_bind_input_box.clone();
        let click_input_controller = GestureClick::new();
        click_input_controller.connect_pressed(move |_, _, _, _| {
            key_bind_input_box_clone.grab_focus();
        });

        key_bind_input_box.add_controller(click_input_controller);
        key_bind_input_box
    }

    fn create_key_bind_change_callback(
        key_bind_input_box: gtk::Box,
        key_bind_changed_callback: impl Fn(KeyBindConfiguration)
    ) -> impl Fn(&EventControllerKey, Key, u32, ModifierType) -> Propagation {
        let key_pressed_callback = move |_: &EventControllerKey, key: Key, _: u32, modifier_state: ModifierType| {
            let control_name = "CTRL".to_string();
            let shift_name = "SHIFT".to_string();
            let alt_name = "ALT".to_string();

            let key_name = key.name()
                .expect("Cannot get key name")
                .to_string()
                .to_uppercase();

            let mut captured_keys: Vec<String> = vec![];
            if modifier_state.contains(ModifierType::SHIFT_MASK) || key_name.contains(&shift_name) {
                captured_keys.push(shift_name.clone());
            }

            if modifier_state.contains(ModifierType::CONTROL_MASK) || key_name.contains("CONTROL") {
                captured_keys.push(control_name.clone());
            }

            if modifier_state.contains(ModifierType::ALT_MASK) || key_name.contains(&alt_name) {
                captured_keys.push(alt_name.clone());
            }

            if !key_name.contains(&shift_name) && !key_name.contains(&alt_name) && !key_name.contains("CONTROL") {
                captured_keys.push(key_name);
            }

            let key_bind_configuration = KeyBindConfiguration::new(captured_keys.clone());
            key_bind_changed_callback(key_bind_configuration);

            Self::create_key_bind_display(key_bind_input_box.clone(), captured_keys.clone());
            Propagation::Proceed
        };
        key_pressed_callback
    }

    fn create_key_bind_display(key_bind_input_box: gtk::Box, keys: Vec<String>) {
        Self::clear_input_box(&key_bind_input_box);
        for key in keys {
            let key_symbol = KeySymbol::new(key.clone());
            key_bind_input_box.append(key_symbol.get_widget());
        }
    }

    fn create_reset_button_click(keybind_input_manager: KeybindInputManager, reset_action: impl Fn() + 'static) -> impl Fn(&Button) + 'static {
        let reset_button_click = move |_: &Button| {
            keybind_input_manager.send_event(KeybindInputEvent::ConfigurationChanged(None));
            reset_action();
        };
        reset_button_click
    }
}