use gtk::{ComboBoxText, Orientation, Separator, Switch};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, WidgetExt};
use crate::models::monitor::monitor_configuration::MonitorOrientation;
use crate::providers::application_provider::ApplicationProvider;
use crate::types::{GTKBox, GTKSpinButton};
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::{Boxes, DEFAULT_MARGIN};
use crate::ui::controls::Control;
use crate::ui::controls::display_configurator::DisplayConfigurator;
use crate::ui::controls::display_field::DisplayField;
use crate::ui::manager::display_configurator_manager::DisplayConfiguratorManager;
use crate::ui::manager::display_field_manager::{DisplayFieldEvent, DisplayFieldManager};
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::statable_control::StatableControl;
use crate::ui::states::display_configurator_state::DisplayConfiguratorState;
use crate::ui::states::display_field_state::DisplayFieldState;
use crate::ui::states::display_settings_state::DisplaySettingsState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::new_rc_mut;

pub struct Displays {
    application_provider: ApplicationProvider,
    display_box: GTKBox,
    display_fields_box: GTKBox
}

impl Control for Displays {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.display_box
    }
}

impl UpdatableControl<DisplaySettingsState> for Displays {
    fn update_ui(&mut self, state: DisplaySettingsState) {
        self.clear_display_fields();

        if state.enabled {
            Self::create_display_fields(self, state);
        } else {
            Self::create_display_warning(self);
        }
    }
}

impl Displays {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let display_box = SectionBoxBuilder::new("displays", DEFAULT_MARGIN)
            .create_header_elements("Available displays")
            .build().expect("Failed to create display settings section box");

        display_box.set_margin_top(10);
        display_box.set_margin_bottom(10);
        display_box.set_margin_start(10);
        display_box.set_margin_end(10);

        let display_fields_box = BoxBuilder::new("display-fields")
            .set_orientation(Orientation::Vertical)
            .build();

        let display_provider = application_provider.get_display_provider();
        let display_configurator_state = DisplayConfiguratorState::from(display_provider);
        let display_configurator = new_rc_mut(
            DisplayConfigurator::new(application_provider.get_display_provider())
        );

        for (monitor_port, _) in &display_configurator_state.display_element_states {
            display_configurator.borrow_mut().insert_display_element(monitor_port.clone());
        }

        display_configurator.borrow_mut().update_state(display_configurator_state.clone());
        display_configurator.borrow_mut().update_ui(display_configurator_state.clone());

        let display_configurator_manager = DisplayConfiguratorManager::new(display_configurator.clone());
        display_configurator.borrow_mut().init_events_by_manager(display_configurator_manager);

        display_box.append(&display_fields_box);
        display_box.append(display_configurator.borrow().get_widget());

        Self {
            application_provider,
            display_box,
            display_fields_box
        }
    }

    fn clear_display_fields(&mut self) {
        Boxes::clear_box_content(&self.display_fields_box);
    }

    fn create_display_fields(&mut self, state: DisplaySettingsState) {
        for (port, configuration) in state.monitor_configurations {
            let separator = Separator::new(Orientation::Horizontal);
            let display_field = new_rc_mut(DisplayField::new());

            let display_field_state = DisplayFieldState {
                monitor_port: port.clone(),
                monitor_configuration: configuration.clone(),
            };
            display_field.borrow_mut().update_ui(display_field_state);

            let display_field_manager = DisplayFieldManager::new(display_field.clone());
            let display_provider = self.application_provider.get_display_provider();
            let port_clone = port.clone();
            let active_spin_button_change = move |_: &Switch, state: bool| -> Propagation {
                display_field_manager.send_event(DisplayFieldEvent::VisibilityChanged(state));
                display_provider.borrow_mut().set_monitor_state(port_clone.clone(), state);

                Propagation::Proceed
            };
            display_field.borrow_mut().set_active_change(active_spin_button_change);

            let display_provider = self.application_provider.get_display_provider();
            let port_clone = port.clone();
            let width_spin_button_change = move |spin_button: &GTKSpinButton| {
                display_provider.borrow_mut().set_monitor_width(port_clone.clone(), spin_button.value() as u32);
            };
            display_field.borrow_mut().set_width_change(width_spin_button_change);

            let display_provider = self.application_provider.get_display_provider();
            let port_clone = port.clone();
            let height_spin_button_change = move |spin_button: &GTKSpinButton| {
                display_provider.borrow_mut().set_monitor_height(port_clone.clone(), spin_button.value() as u32);
            };
            display_field.borrow_mut().set_height_change(height_spin_button_change);

            let display_provider = self.application_provider.get_display_provider();
            let port_clone = port.clone();
            let refresh_rate_spin_button_change = move |spin_button: &GTKSpinButton| {
                display_provider.borrow_mut().set_monitor_refresh_rate(port_clone.clone(), spin_button.value() as u32);
            };
            display_field.borrow_mut().set_refresh_rate_change(refresh_rate_spin_button_change);

            let display_provider = self.application_provider.get_display_provider();
            let port_clone = port.clone();
            let resolution_scale_spin_button_change = move |spin_button: &GTKSpinButton| {
                display_provider.borrow_mut().set_resolution_scale(port_clone.clone(), spin_button.value() as f32);
            };
            display_field.borrow_mut().set_resolution_scale_change(resolution_scale_spin_button_change);

            let display_provider = self.application_provider.get_display_provider();
            let port_clone = port.clone();
            let monitor_mode_selection_box_change = move |combobox: &ComboBoxText| {
                let monitor_orientation = MonitorOrientation::from(combobox.active_text().unwrap().to_string());
                display_provider.borrow_mut().set_monitor_orientation(port_clone.clone(), monitor_orientation);
            };
            display_field.borrow_mut().set_orientation_change(monitor_mode_selection_box_change);

            self.display_fields_box.append(display_field.borrow().get_widget());
            self.display_fields_box.append(&separator);
        }
    }

    fn create_display_warning(&mut self) {
        let display_warning_box = Boxes::create_warning_box(
            "⚠️ Wayland RandR program module not found. This is required to configure the monitor displays."
        );

        self.display_fields_box.append(&display_warning_box);
    }
}