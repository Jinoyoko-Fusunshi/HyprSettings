use gtk::{ComboBoxText, Orientation, Separator, Switch};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, WidgetExt};
use crate::models::monitor::monitor_configuration::MonitorOrientation;
use crate::providers::application_provider::ApplicationProvider;
use crate::types::{GTKBox, GTKSpinButton};
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::{Boxes, DEFAULT_MARGIN};
use crate::ui::controls::Control;
use crate::ui::controls::monitor_configurator::MonitorConfigurator;
use crate::ui::controls::monitor_field::MonitorField;
use crate::ui::manager::monitor_configurator_manager::MonitorConfiguratorManager;
use crate::ui::manager::monitor_field_manager::{MonitorFieldEvent, MonitorFieldManager};
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::statable_control::StatableControl;
use crate::ui::states::monitor_configurator_state::MonitorConfiguratorState;
use crate::ui::states::monitor_field_state::MonitorFieldState;
use crate::ui::states::monitor_settings_state::MonitorSettingsState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::new_rc_mut;

pub struct Monitors {
    application_provider: ApplicationProvider,
    monitor_box: GTKBox,
    monitor_fields_box: GTKBox
}

impl Control for Monitors {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.monitor_box
    }
}

impl UpdatableControl<MonitorSettingsState> for Monitors {
    fn update_ui(&mut self, state: MonitorSettingsState) {
        self.clear_monitor_fields();

        if state.enabled {
            Self::create_monitor_fields(self, state);
        } else {
            Self::create_monitors_warning(self);
        }
    }
}

impl Monitors {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let monitor_box = SectionBoxBuilder::new("monitors", DEFAULT_MARGIN)
            .create_header_elements("Available monitors")
            .build().expect("Failed to create monitor settings section box");

        monitor_box.set_margin_top(10);
        monitor_box.set_margin_bottom(10);
        monitor_box.set_margin_start(10);
        monitor_box.set_margin_end(10);

        let monitor_fields_box = BoxBuilder::new("monitor-fields")
            .set_orientation(Orientation::Vertical)
            .build();

        let monitor_provider = application_provider.get_monitor_provider();
        let monitor_configurator_state = MonitorConfiguratorState::from(monitor_provider);
        let monitor_configurator = new_rc_mut(
            MonitorConfigurator::new(application_provider.get_monitor_provider())
        );

        for (port, _) in &monitor_configurator_state.monitor_states {
            monitor_configurator.borrow_mut().insert_monitor(port);
        }

        monitor_configurator.borrow_mut().update_state(monitor_configurator_state.clone());
        monitor_configurator.borrow_mut().update_ui(monitor_configurator_state.clone());

        let monitor_configurator_manager = MonitorConfiguratorManager::new(monitor_configurator.clone());
        monitor_configurator.borrow_mut().init_events_by_manager(monitor_configurator_manager);

        monitor_box.append(&monitor_fields_box);
        monitor_box.append(monitor_configurator.borrow().get_widget());

        Self {
            application_provider,
            monitor_box,
            monitor_fields_box
        }
    }

    fn clear_monitor_fields(&mut self) {
        Boxes::clear_box_content(&self.monitor_fields_box);
    }

    fn create_monitor_fields(&mut self, state: MonitorSettingsState) {
        for (port, configuration) in state.monitor_configurations {
            let separator = Separator::new(Orientation::Horizontal);
            let monitor_field = new_rc_mut(MonitorField::new());

            let monitor_field_state = MonitorFieldState {
                monitor_port: port.clone(),
                monitor_configuration: configuration.clone(),
            };
            monitor_field.borrow_mut().update_ui(monitor_field_state);

            let monitor_field_manager = MonitorFieldManager::new(monitor_field.clone());
            let monitor_provider = self.application_provider.get_monitor_provider();
            let port_clone = port.clone();
            let active_spin_button_change = move |_: &Switch, state: bool| -> Propagation {
                monitor_field_manager.send_event(MonitorFieldEvent::VisibilityChanged(state));
                monitor_provider.borrow_mut().set_monitor_state(port_clone.clone(), state);

                Propagation::Proceed
            };
            monitor_field.borrow_mut().set_active_change(active_spin_button_change);

            let monitor_provider = self.application_provider.get_monitor_provider();
            let port_clone = port.clone();
            let width_spin_button_change = move |spin_button: &GTKSpinButton| {
                monitor_provider.borrow_mut().set_monitor_width(port_clone.clone(), spin_button.value() as u32);
            };
            monitor_field.borrow_mut().set_width_change(width_spin_button_change);

            let monitor_provider = self.application_provider.get_monitor_provider();
            let port_clone = port.clone();
            let height_spin_button_change = move |spin_button: &GTKSpinButton| {
                monitor_provider.borrow_mut().set_monitor_height(port_clone.clone(), spin_button.value() as u32);
            };
            monitor_field.borrow_mut().set_height_change(height_spin_button_change);

            let monitor_provider = self.application_provider.get_monitor_provider();
            let port_clone = port.clone();
            let refresh_rate_spin_button_change = move |spin_button: &GTKSpinButton| {
                monitor_provider.borrow_mut().set_monitor_refresh_rate(port_clone.clone(), spin_button.value() as u32);
            };
            monitor_field.borrow_mut().set_refresh_rate_change(refresh_rate_spin_button_change);

            let monitor_provider = self.application_provider.get_monitor_provider();
            let port_clone = port.clone();
            let resolution_scale_spin_button_change = move |spin_button: &GTKSpinButton| {
                monitor_provider.borrow_mut().set_monitor_scale(port_clone.clone(), spin_button.value() as f32);
            };
            monitor_field.borrow_mut().set_resolution_scale_change(resolution_scale_spin_button_change);

            let monitor_provider = self.application_provider.get_monitor_provider();
            let port_clone = port.clone();
            let monitor_mode_selection_box_change = move |combobox: &ComboBoxText| {
                let monitor_orientation = MonitorOrientation::from(combobox.active_text().unwrap().to_string());
                monitor_provider.borrow_mut().set_monitor_orientation(port_clone.clone(), monitor_orientation);
            };
            monitor_field.borrow_mut().set_orientation_change(monitor_mode_selection_box_change);

            self.monitor_fields_box.append(monitor_field.borrow().get_widget());
            self.monitor_fields_box.append(&separator);
        }
    }

    fn create_monitors_warning(&mut self) {
        let monitors_warning_box = Boxes::create_warning_box(
            "⚠️ Wayland RandR program module not found. This is required to configure the monitors."
        );

        self.monitor_fields_box.append(&monitors_warning_box);
    }
}