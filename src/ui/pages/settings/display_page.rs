use std::collections::HashMap;
use gtk::{Label, Orientation, Separator, SpinButton, Switch};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, WidgetExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::boxes::Boxes;
use crate::ui::controls::Control;
use crate::ui::controls::display_field::DisplayField;
use crate::ui::states::display_field_state::DisplayFieldState;
use crate::ui::states::display_settings_state::DisplaySettingsState;
use crate::ui::updatable_control::UpdatableControl;

pub struct DisplaySettings {
    application_provider: ApplicationProvider,
    display_box: gtk::Box,
    display_fields_box: gtk::Box,
    display_fields: HashMap<String, DisplayField>,
}

impl Control for DisplaySettings {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.display_box
    }
}

impl UpdatableControl<DisplaySettingsState> for DisplaySettings {
    fn update_ui(&mut self, state: DisplaySettingsState) {
        self.clear_display_fields();

        if state.enabled {
            Self::create_display_fields(self, state);
        } else {
            Self::create_display_warning(self);
        }
    }
}

impl DisplaySettings {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let display_box = gtk::Box::new(Orientation::Vertical, 10);
        display_box.set_margin_top(10);
        display_box.set_margin_bottom(10);
        display_box.set_margin_start(10);
        display_box.set_margin_end(10);

        let available_displays_label = Label::new(Some("Available displays"));
        let display_fields_box = gtk::Box::new(Orientation::Vertical, 10);

        display_box.append(&available_displays_label);
        display_box.append(&Separator::new(Orientation::Horizontal));
        display_box.append(&display_fields_box);

        Self {
            application_provider,
            display_box,
            display_fields_box,
            display_fields: HashMap::new(),
        }
    }

    fn clear_display_fields(&mut self) {
        self.display_fields.clear();

        Boxes::clear_box_content(&self.display_fields_box);
    }

    fn create_display_fields(&mut self, state: DisplaySettingsState) {
        for (port, configuration) in state.monitor_configurations {
            let separator = Separator::new(Orientation::Horizontal);
            let mut display_field = DisplayField::new();
            let display_field_state = DisplayFieldState {
                monitor_port: port.clone(),
                monitor_configuration: configuration.clone(),
            };
            display_field.update_ui(display_field_state);

            let settings_provider = self.application_provider.get_settings_provider();
            let port_clone = port.clone();
            let spin_button_active_change = move |_: &Switch, state: bool| -> Propagation {
                settings_provider.borrow_mut().set_monitor_state(port_clone.clone(), state);

                Propagation::Proceed
            };
            display_field.set_active_change(spin_button_active_change);

            let settings_provider = self.application_provider.get_settings_provider();
            let port_clone = port.clone();
            let spin_button_width_change = move |spin_button: &SpinButton| {
                settings_provider.borrow_mut().set_monitor_width(port_clone.clone(), spin_button.value() as u32);
            };
            display_field.set_width_change(spin_button_width_change);

            let settings_provider = self.application_provider.get_settings_provider();
            let port_clone = port.clone();
            let spin_button_height_change = move |spin_button: &SpinButton| {
                settings_provider.borrow_mut().set_monitor_height(port_clone.clone(), spin_button.value() as u32);
            };
            display_field.set_height_change(spin_button_height_change);

            let settings_provider = self.application_provider.get_settings_provider();
            let port_clone = port.clone();
            let spin_button_refresh_rate_change = move |spin_button: &SpinButton| {
                settings_provider.borrow_mut().set_monitor_refresh_rate(port_clone.clone(), spin_button.value() as u32);
            };
            display_field.set_refresh_rate_change(spin_button_refresh_rate_change);

            self.display_fields_box.append(display_field.get_widget());
            self.display_fields_box.append(&separator);
            self.display_fields.insert(port, display_field);
        }
    }

    fn create_display_warning(&mut self) {
        let display_warning_box = Boxes::create_warning_box(
            "⚠️ Wayland RandR program module not found. This is required to configure the monitor displays."
        );

        self.display_fields_box.append(&display_warning_box);
    }
}