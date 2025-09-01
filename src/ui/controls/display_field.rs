use gtk::{Align, Label, Orientation, Switch};
use gtk::glib::Propagation;
use gtk::SpinButton as GTKSpinButton;
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::controls::Control;
use crate::ui::controls::spin_button::SpinButton;
use crate::ui::states::display_field_state::DisplayFieldState;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_control::UpdatableControl;

const NORMAL_INCREMENT_VALUE: f64 = 160.0;
const PAGE_INCREMENT_VALUE: f64 = 320.0;
const CLIMB_RATE: f64 = 1.0;
const DISPLAYED_FLOAT_DIGITS: u32 = 0;

pub struct DisplayField {
    display_field_box: gtk::Box,
    monitor_active_switch: Switch,
    monitor_port_label: Label,
    monitor_display_label: Label,
    width_spin_button: SpinButton,
    height_spin_button: SpinButton,
    refresh_rate_spin_button: SpinButton,
}

impl Control for DisplayField {
    fn init_events(&self) {

    }

    fn get_widget(&self) -> &gtk::Box {
        &self.display_field_box

    }
}

impl UpdatableControl<DisplayFieldState> for DisplayField {
    fn update_ui(&mut self, state: DisplayFieldState) {
        self.monitor_active_switch.set_active(state.monitor_configuration.enabled);

        let monitor_port_name = state.monitor_port + ": ";
        self.monitor_port_label.set_label(monitor_port_name.as_str());

        let monitor_information = state.monitor_configuration.information.clone();
        let monitor_display_name = monitor_information.model_name.clone() + " - "
            + monitor_information.brand_name.as_str();
        self.monitor_display_label.set_label(monitor_display_name.as_str());

        let min_video_mode = state.monitor_configuration.information.min_video_mode.clone();
        let max_video_mode = state.monitor_configuration.information.max_video_mode.clone();
        let current_video_mode = state.monitor_configuration.video_mode.clone();

        let width_button_state = SpinButtonState {
            label_text: "Width:".to_string(),
            climb_rate: CLIMB_RATE,
            min_value: min_video_mode.width_resolution as f64,
            max_value: max_video_mode.width_resolution as f64,
            digit_count: DISPLAYED_FLOAT_DIGITS,
            use_integral_numbers: false,
            current_value: current_video_mode.width_resolution as f64,
            increment_value: NORMAL_INCREMENT_VALUE,
            page_size: 0.0,
            page_increment_value: PAGE_INCREMENT_VALUE
        };
        self.width_spin_button.update_ui(width_button_state);

        let height_spin_button_state = SpinButtonState {
            label_text: "Height:".to_string(),
            climb_rate: CLIMB_RATE,
            min_value: min_video_mode.height_resolution as f64,
            max_value: max_video_mode.height_resolution as f64,
            digit_count: DISPLAYED_FLOAT_DIGITS,
            use_integral_numbers: false,
            current_value: current_video_mode.height_resolution as f64,
            increment_value: NORMAL_INCREMENT_VALUE,
            page_size: 0.0,
            page_increment_value: PAGE_INCREMENT_VALUE
        };
        self.height_spin_button.update_ui(height_spin_button_state);

        let refresh_rate_spin_button_state = SpinButtonState {
            label_text: "Refresh Rate:".to_string(),
            climb_rate: CLIMB_RATE,
            min_value: min_video_mode.refresh_rate as f64,
            max_value: max_video_mode.refresh_rate as f64,
            digit_count: DISPLAYED_FLOAT_DIGITS,
            use_integral_numbers: false,
            current_value: current_video_mode.refresh_rate as f64,
            increment_value: NORMAL_INCREMENT_VALUE,
            page_size: 0.0,
            page_increment_value: PAGE_INCREMENT_VALUE
        };
        self.refresh_rate_spin_button.update_ui(refresh_rate_spin_button_state);
    }
}

impl DisplayField {
    pub fn new() -> Self {
        let display_field_box = gtk::Box::new(Orientation::Vertical, 0);
        let monitor_info_box = gtk::Box::new(Orientation::Horizontal, 10);
        let video_setting_box = gtk::Box::new(Orientation::Horizontal, 10);

        // Toggle button to en/-disable the monitor
        let monitor_active_switch = Switch::new();
        monitor_active_switch.set_valign(Align::Center);
        monitor_active_switch.set_halign(Align::Center);
        monitor_active_switch.set_active(true);

        let monitor_port_name = "".to_string();
        let monitor_port_label = Self::create_label(&monitor_port_name, 80);

        let monitor_display_name = "".to_string();
        let monitor_display_label = Self::create_label(&monitor_display_name, 180);

        monitor_info_box.append(&monitor_active_switch);
        monitor_info_box.append(&monitor_port_label);
        monitor_info_box.append(&monitor_display_label);

        let width_spin_button = SpinButton::new();
        let height_spin_button = SpinButton::new();
        let refresh_rate_spin_button = SpinButton::new();

        video_setting_box.append(width_spin_button.get_widget());
        video_setting_box.append(height_spin_button.get_widget());
        video_setting_box.append(refresh_rate_spin_button.get_widget());

        display_field_box.append(&monitor_info_box);
        display_field_box.append(&video_setting_box);

        Self {
            display_field_box,
            monitor_active_switch,
            monitor_port_label,
            monitor_display_label,
            width_spin_button,
            height_spin_button,
            refresh_rate_spin_button
        }
    }

    pub fn set_active_change(&self, value_change: impl Fn(&Switch, bool) -> Propagation + 'static) {
        self.monitor_active_switch.connect_state_set(value_change);
    }

    pub fn set_width_change(&self, value_change: impl Fn(&GTKSpinButton) + 'static) {
        self.width_spin_button.set_value_change(value_change)
    }

    pub fn set_height_change(&self, value_change: impl Fn(&GTKSpinButton) + 'static) {
        self.height_spin_button.set_value_change(value_change)
    }

    pub fn set_refresh_rate_change(&self, value_change: impl Fn(&GTKSpinButton) + 'static) {
        self.refresh_rate_spin_button.set_value_change(value_change)
    }

    fn create_label(text: &String, width: i32) -> Label {
        let label = Label::new(Some(text.as_str()));
        label.set_width_request(width);
        label.set_valign(Align::Center);
        label.set_halign(Align::Start);
        label.set_xalign(0.0);
        label
    }
}


