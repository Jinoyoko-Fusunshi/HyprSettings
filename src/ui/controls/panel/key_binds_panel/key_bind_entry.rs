use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, EventControllerKey, GestureClick, Label, Orientation};
use gtk::gdk::ModifierType;
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::ui::controls::panel::key_binds_panel::key_symbol::KeySymbol;
use crate::ui::controls::panel::Panel;

pub struct KeyBindEntry {
    key_bind_entry_box: gtk::Box,
    key_bind_input_box: gtk::Box,
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

    fn get_widget(&self) -> &gtk::Box {
        &self.key_bind_entry_box
    }
}

impl KeyBindEntry {
    pub fn new() -> Self {
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

        key_bind_entry.reset_input();
        key_bind_entry
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

        let key_input_controller = EventControllerKey::new();
        key_input_controller.connect_key_pressed(move |_, key, _, state| {
            let control_name = "CTRL".to_string();
            let shift_name = "SHIFT".to_string();
            let alt_name = "ALT".to_string();

            let key_name = key.name()
                .expect("Cannot get key name")
                .to_string()
                .to_uppercase();

            let mut captured_keys: Vec<String> = vec![];
            if state.contains(ModifierType::SHIFT_MASK) || key_name.contains(&shift_name) {
                captured_keys.push(shift_name.clone());
            }

            if state.contains(ModifierType::CONTROL_MASK) || key_name.contains("CONTROL") {
                captured_keys.push(control_name.clone());
            }

            if state.contains(ModifierType::ALT_MASK) || key_name.contains(&alt_name) {
                captured_keys.push(alt_name.clone());
            }

            if !key_name.contains(&shift_name) && !key_name.contains(&alt_name) && !key_name.contains("CONTROL") {
                captured_keys.push(key_name);
            }

            Self::clear_input_box(&key_bind_input_box_clone);
            for key in captured_keys {
                let key_symbol = KeySymbol::new(key);
                key_bind_input_box_clone.append(key_symbol.get_widget());
            }

            Propagation::Proceed
        });

        let key_bind_input_box_clone = key_bind_input_box.clone();
        let click_input_controller = GestureClick::new();
        click_input_controller.connect_pressed(move |_, _, _, _| {
            key_bind_input_box_clone.grab_focus();
        });

        key_bind_input_box.add_controller(key_input_controller);
        key_bind_input_box.add_controller(click_input_controller);
        key_bind_input_box
    }
}