use gtk::{DropDown, Orientation, ScrolledWindow, Separator, Switch};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, WidgetExt};
use crate::models::monitor::monitor_configuration::MonitorOrientation;
use crate::providers::application_provider::ApplicationProvider;
use crate::types::{GTKBox, GTKSpinButton};
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::{Boxes, DEFAULT_MARGIN};
use crate::ui::manager::control_manager::ControlManager;
use crate::ui::controls::Control;
use crate::ui::controls::monitor_configurator::MonitorConfigurator;
use crate::ui::controls::monitor_field::MonitorField;
use crate::ui::controls::selection_box::SelectionBox;
use crate::ui::managed_control::ManagedControl;
use crate::ui::manager::monitor_configurator_manager::MonitorConfiguratorManager;
use crate::ui::manager::monitor_field_manager::{MonitorFieldEvent, MonitorFieldManager};
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::states::monitor_configurator_state::MonitorConfiguratorState;
use crate::ui::states::monitor_field_state::MonitorFieldState;
use crate::ui::states::monitors_state::MonitorsState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::new_rc_mut;

pub struct Monitors {
    state: MonitorsState,
    application_provider: ApplicationProvider,
    monitor_scroll_box: GTKBox,
    monitor_box: GTKBox,
}

impl Control for Monitors {
    fn get_widget(&self) -> &GTKBox {
        &self.monitor_scroll_box
    }
}

impl UpdatableControl<MonitorsState> for Monitors {
    fn update_state(&mut self, state: MonitorsState) {
        if state.enabled {
            self.create_monitor_fields(state.clone());
            self.create_monitor_configurator();
        } else {
            self.create_monitors_warning();
        }
        
        self.state = state;
    }

    fn get_current_state(&self) -> MonitorsState {
        self.state.clone()
    }
}

impl Monitors {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let monitor_box = SectionBoxBuilder::new("monitors", DEFAULT_MARGIN)
            .create_header_elements("Available monitors")
            .build().expect("Failed to create monitor settings section box");

        let monitor_scroll_window = ScrolledWindow::new();
        monitor_scroll_window.set_widget_name("monitor-scroll-window");
        monitor_scroll_window.set_vexpand(true);
        monitor_scroll_window.set_child(Some(&monitor_box));

        let monitor_scroll_box = BoxBuilder::new("monitor-scroll-box")
            .set_orientation(Orientation::Vertical)
            .build();
        monitor_scroll_box.append(&monitor_scroll_window);

        let state = Default::default();
        
        Self {
            state,
            application_provider,
            monitor_scroll_box,
            monitor_box
        }
    }

    fn create_monitor_fields(&mut self, state: MonitorsState) {
        let monitor_fields_box = BoxBuilder::new("monitor-fields")
            .set_orientation(Orientation::Vertical)
            .build();

        for (port, configuration) in state.monitor_configurations {
            let separator = Separator::new(Orientation::Horizontal);
            let monitor_field = new_rc_mut(MonitorField::new());

            let monitor_field_state = MonitorFieldState {
                monitor_port: port.clone(),
                monitor_configuration: configuration.clone(),
            };
            monitor_field.borrow_mut().update_state(monitor_field_state);

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
            let monitor_mode_selection_box_change = move |dropdown: &DropDown| {
                let selected_option = SelectionBox::get_selected_option(dropdown);
                let monitor_orientation = MonitorOrientation::from(selected_option);
                monitor_provider.borrow_mut().set_monitor_orientation(port_clone.clone(), monitor_orientation);
            };
            monitor_field.borrow_mut().set_orientation_change(monitor_mode_selection_box_change);

            monitor_fields_box.append(monitor_field.borrow().get_widget());
            monitor_fields_box.append(&separator);
        }

        self.monitor_box.append(&monitor_fields_box);
    }

    fn create_monitor_configurator(&mut self) {
        let monitor_provider = self.application_provider.get_monitor_provider();
        let monitor_configurator_state = MonitorConfiguratorState::from(monitor_provider);

        let monitor_configurator = new_rc_mut(
            MonitorConfigurator::new(self.application_provider.get_monitor_provider())
        );

        for (port, _) in &monitor_configurator_state.monitor_states {
            monitor_configurator.borrow_mut().insert_monitor(port);
        }

        monitor_configurator.borrow_mut().update_state(monitor_configurator_state.clone());

        let monitor_configurator_manager = MonitorConfiguratorManager::new(
            monitor_configurator.clone()
        );
        monitor_configurator.borrow_mut().init_events_by_manager(monitor_configurator_manager);
        
        self.monitor_box.append(monitor_configurator.borrow().get_widget());
    }

    fn create_monitors_warning(&mut self) {
        let monitors_warning_box = Boxes::create_warning_box(
            "⚠️ wlr-randr dependency module not found. This is required to configure the monitors."
        );

        self.monitor_box.append(&monitors_warning_box);
    }
}