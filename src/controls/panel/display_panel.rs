use std::cell::{RefCell};
use std::rc::Rc;
use gtk::{glib, Align, Label, Orientation, Separator, SpinButton, Switch};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::controls::named_spin_button_section::NamedSpinButtonSection;
use crate::controls::panel::Panel;
use crate::monitor::monitor_setting::MonitorSetting;
use crate::settings_container::SettingsContainer;

pub struct DisplayPanel {
    widget: gtk::Box
}

impl Panel for DisplayPanel {
    fn get_widget(&self) -> &gtk::Box {
        &self.widget
    }
}

impl DisplayPanel {
    pub fn new(settings: &Rc<RefCell<SettingsContainer>>) -> Self {
        let widget = gtk::Box::new(Orientation::Vertical, 10);
        widget.set_margin_top(10);
        widget.set_margin_bottom(10);
        widget.set_margin_start(10);
        widget.set_margin_end(10);

        let available_displays_label = Label::new(Some("Available displays"));
        widget.append(&available_displays_label);

        let mut monitor_index: usize = 0;
        for monitor_setting in settings.borrow_mut().get_monitor_settings() {
            let settings_clone = settings.clone();
            let status_action_callback = move |_: &Switch, state: bool| -> glib::Propagation {
                settings_clone.borrow_mut().set_monitor_status_by_index(state, monitor_index);
                glib::Propagation::Proceed
            };

            let settings_clone = settings.clone();
            let width_change_callback = move |spin_button: &SpinButton| {
                settings_clone.borrow_mut().set_monitor_width_resolution_by_index(spin_button.value() as u32, monitor_index);
            };

            let settings_clone = settings.clone();
            let height_change_callback = move |spin_button: &SpinButton| {
                settings_clone.borrow_mut().set_monitor_height_resolution_by_index(spin_button.value() as u32, monitor_index);
            };

            let settings_clone = settings.clone();
            let refresh_rate_change_callback = move |spin_button: &SpinButton| {
                settings_clone.borrow_mut().set_monitor_refresh_rate_by_index(spin_button.value() as u32, monitor_index);
            };

            let display_entry = DisplayPanel::create_display_entry(
                monitor_setting, status_action_callback, width_change_callback,
                height_change_callback, refresh_rate_change_callback
            );
            let vertical_separator = Separator::new(Orientation::Vertical);

            widget.append(&vertical_separator);
            widget.append(&display_entry);

            monitor_index += 1;
        }

        Self {
            widget
        }
    }

    fn create_display_entry(
        settings: &MonitorSetting,
        status_action_callback: impl Fn(&Switch, bool) -> glib::Propagation + 'static,
        width_change_callback: impl Fn(&SpinButton) + 'static,
        height_change_callback: impl Fn(&SpinButton) + 'static,
        refresh_rate_change_callback: impl Fn(&SpinButton) + 'static,
    ) -> gtk::Box {
        let monitor_information = settings.get_information();
        let min_video_mode = monitor_information.get_min_video_mode();
        let max_video_mode = monitor_information.get_max_video_mode();

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
        let monitor_port_name = monitor_information.get_port_name().clone() + ": ";
        let monitor_port_label = DisplayPanel::create_label(&monitor_port_name, 80);

        let monitor_display_name = monitor_information.get_model_name().clone() + " - " +
            monitor_information.get_brand_name();
        let monitor_display_label = DisplayPanel::create_label(&monitor_display_name, 180);

        monitor_display_info_box.append(&monitor_toggle_switch);
        monitor_display_info_box.append(&monitor_port_label);
        monitor_display_info_box.append(&monitor_display_label);

        // The toggle button to set the monitor width size
        let width_resolution_spin_button_section = DisplayPanel::create_display_spin_button(
            "Width:", min_video_mode.get_width_resolution(), max_video_mode.get_width_resolution(),
            Some(width_change_callback)
        );

        // The toggle button to set the monitor height size
        let height_resolution_spin_button_section = DisplayPanel::create_display_spin_button(
            "Height:", min_video_mode.get_height_resolution(), max_video_mode.get_height_resolution(),
            Some(height_change_callback)
        );

        // The toggle button to set the monitor refresh rate
        let refresh_rate_spin_button_section = DisplayPanel::create_display_spin_button(
            "Refresh Rate:", min_video_mode.get_refresh_rate(), max_video_mode.get_refresh_rate(),
            Some(refresh_rate_change_callback)
        );

        monitor_video_setting_box.append(width_resolution_spin_button_section.get_widget());
        monitor_video_setting_box.append(height_resolution_spin_button_section.get_widget());
        monitor_video_setting_box.append(refresh_rate_spin_button_section.get_widget());

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
        label_text: &str, min_value: u32, max_value: u32,
        spin_button_change_callback: Option<impl Fn(&SpinButton) + 'static>
    ) -> NamedSpinButtonSection {
        const NORMAL_INCREMENT_VALUE: f64 = 160.0;
        const PAGE_INCREMENT_VALUE: f64 = 320.0;
        const CLIMB_RATE: f64 = 1.0;
        const DISPLAYED_FLOAT_DIGITS: u32 = 0;

        NamedSpinButtonSection::new(
            label_text, min_value as f64, max_value as f64, max_value as f64, NORMAL_INCREMENT_VALUE,
            PAGE_INCREMENT_VALUE, 0.0, CLIMB_RATE, DISPLAYED_FLOAT_DIGITS,
            false, spin_button_change_callback
        )
    }
}