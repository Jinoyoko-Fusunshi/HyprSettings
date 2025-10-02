use gtk::{Align, Button, EventControllerKey, GestureClick, Label, Orientation};
use gtk::gdk::{Key, ModifierType};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::manager::keybind_input_manager::{KeybindInputEvent, KeybindInputManager};
use crate::ui::controls::keybinds::key_symbol::KeySymbol;
use crate::ui::controls::keybinds::keybind_converter::KeybindConverter;
use crate::ui::css_styles::CSSStyles;
use crate::ui::manager::control_manager::ControlManager;
use crate::ui::states::keybind_input_state::KeybindInputState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::{new_rc_mut, RcMut};

pub struct KeybindInput {
    state: RcMut<KeybindInputState>,
    keybind_input_box: GTKBox,
    keybind_symbols_box: GTKBox,
    reset_key_button: Button,
}

impl Control for KeybindInput {
    fn init_events(&self) {
        let keybind_symbols_box = self.keybind_symbols_box.clone();
        let click_input_controller = GestureClick::new();
        let state = self.state.clone();

        click_input_controller.connect_pressed(move |_, _, _, _| {
            keybind_symbols_box.grab_focus();

            if let None = state.borrow().configuration.clone() {
                Self::set_keybind_symbols_text(&keybind_symbols_box, "Record keys individually ...");
            }
        });

        self.keybind_symbols_box.add_controller(click_input_controller);
    }

    fn get_widget(&self) -> &GTKBox {
        &self.keybind_input_box
    }
}

impl UpdatableControl<KeybindInputState> for KeybindInput {
    fn update_state(&mut self, state: KeybindInputState) {
        if let Some(configuration) = state.configuration.clone() {
            self.set_keybind(configuration);
        } else {
            self.reset_input();
        }

        *self.state.borrow_mut() = state;
    }

    fn get_current_state(&self) -> KeybindInputState {
        self.state.borrow().clone()
    }
}

impl ActivableControl for KeybindInput {
    fn enable_control(&self) {
        self.keybind_input_box.set_sensitive(true);
    }

    fn disable_control(&self) {
        self.keybind_input_box.set_sensitive(false);
    }
}

impl KeybindInput {
    pub fn new() -> Self {
        let keybind_input_box = BoxBuilder::new("keybind-input")
            .set_orientation(Orientation::Horizontal)
            .build();

        let keybind_symbols_box = Self::create_keybind_symbols_box();
        let reset_key_button = Button::with_label("➖");
        reset_key_button.set_valign(Align::Center);
        reset_key_button.set_vexpand(false);

        keybind_input_box.append(&keybind_symbols_box);
        keybind_input_box.append(&reset_key_button);

        let state = new_rc_mut(KeybindInputState {
            configuration: None,
        });

        Self {
            state,
            keybind_input_box,
            keybind_symbols_box,
            reset_key_button
        }
    }

    pub fn set_input_change(&self, input_change: impl Fn(KeyBindConfiguration) + 'static) {
        let key_input_controller = EventControllerKey::new();
        let key_pressed_callback = Self::create_keybind_change_callback(
            self.keybind_symbols_box.clone(), self.state.clone(), input_change
        );
        key_input_controller.connect_key_pressed(key_pressed_callback);
        self.keybind_symbols_box.add_controller(key_input_controller);
    }

    pub fn set_reset_button_click(
        &self,
        keybind_input_manager: KeybindInputManager,
        input_clear: Option<impl Fn() + 'static>
    ) {
        if let Some(input_clear) = input_clear {
            self.reset_key_button.connect_clicked(Self::create_custom_reset_button_click(keybind_input_manager, input_clear));
        } else {
            self.reset_key_button.connect_clicked(Self::create_reset_button_click(keybind_input_manager));
        };
    }

    pub fn set_keybind(&self, keybind_configuration: KeyBindConfiguration) {
        Self::create_key_bind_display(self.keybind_symbols_box.clone(), keybind_configuration);
    }

    pub fn reset_input(&self) {
        Self::set_keybind_symbols_text(&self.keybind_symbols_box, "Click to bind key");
    }

    fn clear_input_box(input_box: &GTKBox) {
        let mut current_child = input_box.first_child();
        while let Some(child) = current_child {
            current_child = child.next_sibling();
            input_box.remove(&child);
        }
    }

    fn set_keybind_symbols_text(keybind_symbols_box: &GTKBox, text: &str) {
        Self::clear_input(keybind_symbols_box);

        let label = Label::new(Some(text));
        label.set_hexpand(true);
        label.set_xalign(0.5);

        keybind_symbols_box.append(&label);
    }

    fn clear_input(keybind_symbols_box: &GTKBox) {
        Self::clear_input_box(keybind_symbols_box)
    }

    fn create_keybind_symbols_box() -> GTKBox {
        let key_bind_input_box = BoxBuilder::new("keybind-symbols")
            .set_orientation(Orientation::Horizontal)
            .set_class(CSSStyles::KEYBIND_INPUT)
            .set_width(250)
            .set_focusable(true)
            .set_can_focus(true)
            .build();

        key_bind_input_box
    }

    fn create_keybind_change_callback(
        keybind_symbols_box: GTKBox,
        state: RcMut<KeybindInputState>,
        key_bind_changed_callback: impl Fn(KeyBindConfiguration)
    ) -> impl Fn(&EventControllerKey, Key, u32, ModifierType) -> Propagation {
        let key_pressed_callback = move |_: &EventControllerKey, key: Key, _: u32, _: ModifierType| {
            let key_name = key.name()
                .expect("Cannot get key name")
                .to_string()
                .to_uppercase();
            let real_key_name = KeybindConverter::convert_to_real_name(key_name.clone());

            let old_configuration = state.borrow().configuration.clone();
            let keybind_configuration = if let Some(configuration) = old_configuration.clone() {
                if !configuration.has_key(real_key_name.clone()) {
                    let mut new_configuration = configuration.clone();
                    new_configuration.append_key(real_key_name.clone());
                    new_configuration
                } else {
                    configuration.clone()
                }
            } else {
                let mut new_configuration = KeyBindConfiguration::new();
                new_configuration.append_key(real_key_name.clone());
                new_configuration
            };

            state.borrow_mut().configuration = Some(keybind_configuration.clone());
            key_bind_changed_callback(keybind_configuration.clone());

            Self::create_key_bind_display(keybind_symbols_box.clone(), keybind_configuration);
            Propagation::Proceed
        };
        key_pressed_callback
    }

    fn create_key_bind_display(keybind_symbols_box: GTKBox, configuration: KeyBindConfiguration) {
        Self::clear_input_box(&keybind_symbols_box);
        for key in configuration.get_key_names() {
            let key_symbol = KeySymbol::new(key.clone());
            keybind_symbols_box.append(key_symbol.get_widget());
        }
    }

    fn create_custom_reset_button_click(
        keybind_input_manager: KeybindInputManager,
        reset_action: impl Fn() + 'static
    ) -> impl Fn(&Button) + 'static {
        let reset_button_click = move |_: &Button| {
            keybind_input_manager.send_event(KeybindInputEvent::ConfigurationChanged(None));
            reset_action();
        };
        reset_button_click
    }

    fn create_reset_button_click(
        keybind_input_manager: KeybindInputManager
    ) -> impl Fn(&Button) + 'static {
        let reset_button_click = move |_: &Button| {
            keybind_input_manager.send_event(KeybindInputEvent::ConfigurationChanged(None));
        };
        reset_button_click
    }
}