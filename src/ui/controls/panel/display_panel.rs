use std::cell::{RefCell};
use std::rc::Rc;
use gtk::{Align, Label, Orientation, Separator, SpinButton, Switch};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::controls::{named_section::named_spin_button_section::NamedSpinButtonSection, panel::Panel};
use crate::settings::monitor::monitor_configuration::MonitorConfiguration;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct DisplayPanel {
    widget: gtk::Box
}

impl Panel for DisplayPanel {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.widget
    }
}

impl Clone for DisplayPanel {
    fn clone(&self) -> Self {
        Self {
            widget: self.widget.clone()
        }
    }
}

impl DisplayPanel {
    pub fn new(settings: &Rc<RefCell<HyprlandSettings>>) -> Self {
        let widget = gtk::Box::new(Orientation::Vertical, 10);
        widget.set_margin_top(10);
        widget.set_margin_bottom(10);
        widget.set_margin_start(10);
        widget.set_margin_end(10);

        let available_displays_label = Label::new(Some("Available displays"));
        widget.append(&available_displays_label);

        let monitor_ports = settings.clone().borrow().get_monitor_ports();
        for monitor_port in monitor_ports {
            let settings_clone = settings.clone();
            let mut hyprland_settings = settings_clone.borrow_mut();
            let monitor_configuration = hyprland_settings.monitor_configurations
                .get_mut(&monitor_port.clone())
                .unwrap();

            let display_entry = DisplayPanel::create_display_entry(
                monitor_configuration.clone(),
                DisplayPanel::create_status_action_callback(settings.clone(), monitor_port.clone()),
                DisplayPanel::create_width_change_callback(settings.clone(), monitor_port.clone()),
                DisplayPanel::create_height_change_callback(settings.clone(), monitor_port.clone()),
                DisplayPanel::create_refresh_rate_change_callback(settings.clone(), monitor_port.clone())
            );
            let vertical_separator = Separator::new(Orientation::Vertical);

            widget.append(&vertical_separator);
            widget.append(&display_entry);
        }

        Self {
            widget
        }
    }

    //TODO: Extract the part in its own sub component and set each callback individually"
    fn create_display_entry(
        monitor_configuration: MonitorConfiguration,
        status_action_callback: impl Fn(&Switch, bool) -> Propagation + 'static,
        width_change_callback: impl Fn(&SpinButton) + 'static,
        height_change_callback: impl Fn(&SpinButton) + 'static,
        refresh_rate_change_callback: impl Fn(&SpinButton) + 'static,
    ) -> gtk::Box {
        let monitor_information = &monitor_configuration.information;
        let min_video_mode = &monitor_information.min_video_mode;
        let max_video_mode = &monitor_information.max_video_mode;

        let entry_box = gtk::Box::new(Orientation::Vertical, 10);
        let monitor_display_info_box = gtk::Box::new(Orientation::Horizontal, 10);
        let monitor_video_setting_box = gtk::Box::new(Orientation::Horizontal, 10);

        // Toggle button to en/-disable the monitor
        let monitor_toggle_switch = Switch::new();
        monitor_toggle_switch.set_valign(Align::Center);
        monitor_toggle_switch.set_halign(Align::Center);
        monitor_toggle_switch.set_active(true);
        monitor_toggle_switch.connect_state_set(status_action_callback);

        // The monitor display text information
        let monitor_port_name = monitor_information.port_name.clone() + ": ";
        let monitor_port_label = DisplayPanel::create_label(&monitor_port_name, 80);

        let monitor_display_name = monitor_information.model_name.clone() + " - " +
            monitor_information.brand_name.as_str();
        let monitor_display_label = DisplayPanel::create_label(&monitor_display_name, 180);

        monitor_display_info_box.append(&monitor_toggle_switch);
        monitor_display_info_box.append(&monitor_port_label);
        monitor_display_info_box.append(&monitor_display_label);

        // The toggle button to set the monitor width size
        let width_resolution_spin_button_section = DisplayPanel::create_display_spin_button(
            "Width:", min_video_mode.width_resolution, max_video_mode.width_resolution,
            Some(monitor_configuration.video_mode.width_resolution)
        );
        width_resolution_spin_button_section.set_change_callback(width_change_callback);

        // The toggle button to set the monitor height size
        let height_resolution_spin_button_section = DisplayPanel::create_display_spin_button(
            "Height:", min_video_mode.height_resolution, max_video_mode.height_resolution,
            Some(monitor_configuration.video_mode.height_resolution)
        );
        height_resolution_spin_button_section.set_change_callback(height_change_callback);

        // The toggle button to set the monitor refresh rate
        let refresh_rate_spin_button_section = DisplayPanel::create_display_spin_button(
            "Refresh Rate:", min_video_mode.refresh_rate, max_video_mode.refresh_rate,
            Some(monitor_configuration.video_mode.refresh_rate)
        );
        refresh_rate_spin_button_section.set_change_callback(refresh_rate_change_callback);

        monitor_video_setting_box.append(width_resolution_spin_button_section.get_container_box());
        monitor_video_setting_box.append(height_resolution_spin_button_section.get_container_box());
        monitor_video_setting_box.append(refresh_rate_spin_button_section.get_container_box());

        entry_box.append(&monitor_display_info_box);
        entry_box.append(&monitor_video_setting_box);
        entry_box
    }

    fn create_label(text: &String, width: i32) -> Label {
        let label = Label::new(Some(text.as_str()));
        label.set_width_request(width);
        label.set_valign(Align::Center);
        label.set_halign(Align::Start);
        label.set_xalign(0.0);
        label
    }

    fn create_display_spin_button(
        label_text: &str, min_value: u32, max_value: u32, current_value: Option<u32>,
    ) -> NamedSpinButtonSection {
        const NORMAL_INCREMENT_VALUE: f64 = 160.0;
        const PAGE_INCREMENT_VALUE: f64 = 320.0;
        const CLIMB_RATE: f64 = 1.0;
        const DISPLAYED_FLOAT_DIGITS: u32 = 0;

        let selected_value = current_value.unwrap_or_else(|| max_value);
        NamedSpinButtonSection::new(
            label_text, min_value as f64, max_value as f64, selected_value as f64, NORMAL_INCREMENT_VALUE,
            PAGE_INCREMENT_VALUE, 0.0, CLIMB_RATE, DISPLAYED_FLOAT_DIGITS,
            false
        )
    }

    fn create_status_action_callback(
        settings: Rc<RefCell<HyprlandSettings>>, monitor_port: String
    ) -> impl Fn(&Switch, bool) -> Propagation {
        move |_: &Switch, state: bool| -> Propagation {
            let mut hyprland_settings = settings.borrow_mut();
            let monitor_configuration = hyprland_settings.monitor_configurations
                .get_mut(&monitor_port)
                .unwrap();

            monitor_configuration.enabled = state;
            Propagation::Proceed
        }
    }

    fn create_width_change_callback(
        settings: Rc<RefCell<HyprlandSettings>>, monitor_port: String
    ) -> impl Fn(&SpinButton) {
        move |spin_button: &SpinButton| {
            let mut hyprland_settings = settings.borrow_mut();
            let monitor_configuration = hyprland_settings.monitor_configurations
                .get_mut(&monitor_port)
                .unwrap();
            monitor_configuration.video_mode.width_resolution = spin_button.value() as u32;
        }
    }

    fn create_height_change_callback(
        settings: Rc<RefCell<HyprlandSettings>>, monitor_port: String
    ) -> impl Fn(&SpinButton) {
        move |spin_button: &SpinButton| {
            let mut hyprland_settings = settings.borrow_mut();
            let monitor_configuration = hyprland_settings.monitor_configurations
                .get_mut(&monitor_port)
                .unwrap();

            monitor_configuration.video_mode.height_resolution = spin_button.value() as u32;
        }
    }

    fn create_refresh_rate_change_callback(
        settings: Rc<RefCell<HyprlandSettings>>, monitor_port: String
    ) -> impl Fn(&SpinButton) {
        move |spin_button: &SpinButton| {
            let mut hyprland_settings = settings.borrow_mut();
            let monitor_configuration = hyprland_settings.monitor_configurations
                .get_mut(&monitor_port)
                .unwrap();

            monitor_configuration.video_mode.refresh_rate = spin_button.value() as u32;
        }
    }
}