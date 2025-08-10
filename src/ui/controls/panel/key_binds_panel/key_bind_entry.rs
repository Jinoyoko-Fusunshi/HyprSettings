use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, EventControllerKey, GestureClick, Label, Orientation};
use gtk::gdk::{Key, ModifierType};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::settings::key_binds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::controls::panel::key_binds_panel::key_symbol::KeySymbol;
use crate::ui::controls::panel::Panel;

pub struct KeyBindEntry {
    key_bind_entry_box: gtk::Box,
    key_bind_input_box: gtk::Box
}

impl Clone for KeyBindEntry {
    fn clone(&self) -> Self {
        Self {
            key_bind_entry_box: self.key_bind_entry_box.clone(),
            key_bind_input_box: self.key_bind_input_box.clone()
        }
    }
}

impl Panel for KeyBindEntry {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.key_bind_entry_box
    }
}

impl KeyBindEntry {
    pub fn new(selected_key_bind: Option<KeyBindConfiguration>) -> Self {
        let key_bind_entry_box = gtk::Box::new(Orientation::Horizontal, 10);
        let key_bind_input_box = Self::create_key_bind_input_box();
        let key_bind_entry = Self {
            key_bind_entry_box: key_bind_entry_box.clone(),
            key_bind_input_box: key_bind_input_box.clone()
        };

        let key_bind_entry_clone = key_bind_entry.clone();
        let reset_key_button_callback = move |_: &Button| {
            key_bind_entry_clone.reset_input();
        };
        let reset_key_button = Button::with_label("➖");
        reset_key_button.set_valign(Align::Center);
        reset_key_button.set_vexpand(false);
        reset_key_button.connect_clicked(reset_key_button_callback);

        key_bind_entry_box.append(&key_bind_input_box);
        key_bind_entry_box.append(&reset_key_button);

        if let Some(key_bind) = selected_key_bind {
            key_bind_entry.set_keybind(key_bind);
        } else {
            key_bind_entry.reset_input();
        }

        key_bind_entry
    }

    pub fn set_input_callback(&self, callback: impl Fn(KeyBindConfiguration) + 'static) {
        let key_input_controller = EventControllerKey::new();
        let key_pressed_callback = Self::create_key_bind_change_callback(self.key_bind_input_box.clone(), callback);
        key_input_controller.connect_key_pressed(key_pressed_callback);
        self.key_bind_input_box.add_controller(key_input_controller);
    }

    pub fn set_keybind(&self, key_bind_configuration: KeyBindConfiguration) {
        Self::create_key_bind_display(
            self.key_bind_input_box.clone(), key_bind_configuration.get_key_names().clone()
        );
    }

    pub fn clear_input(&self) {
        Self::clear_input_box(&self.key_bind_input_box)
    }

    pub fn reset_input(&self) {
        const INITIAL_TEXT: &str = "Click to bind key";

        self.clear_input();
        let label = Label::new(Some(INITIAL_TEXT));
        label.set_hexpand(true);
        label.set_xalign(0.5);

        self.key_bind_input_box.append(&label);
    }

    pub fn set_active(&self, active: bool) {
        self.key_bind_entry_box.set_sensitive(active);
    }

    fn clear_input_box(input_box: &gtk::Box) {
        let mut current_child = input_box.first_child();
        while let Some(child) = current_child {
            current_child = child.next_sibling();
            input_box.remove(&child);
        }
    }

    fn create_key_bind_input_box() -> gtk::Box {
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
            key_bind_input_box.append(key_symbol.get_container_box());
        }
    }
}