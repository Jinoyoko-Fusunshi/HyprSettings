use gtk::{Align, ComboBoxText, Label, Orientation, Switch};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::{GTKBox, GTKSpinButton};
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::Control;
use crate::ui::controls::selection_box::SelectionBox;
use crate::ui::controls::spin_button::SpinButton;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::statable_control::StatableControl;
use crate::ui::states::display_field_state::DisplayFieldState;
use crate::ui::states::selection_box_state::SelectionBoxState;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_control::UpdatableControl;

const RESOLUTION_INCREMENT: f64 = 1.0;
const RESOLUTION_PAGE_INCREMENT: f64 = 120.0;
const RESOLUTION_CLIMB_RATE: f64 = 10.0;
const REFRESH_INCREMENT: f64 = 1.0;
const REFRESH_PAGE_INCREMENT: f64 = 10.0;
const REFRESH_CLIMB_RATE: f64 = 2.0;
const FLOAT_DIGITS: u32 = 0;

pub struct DisplayField {
    display_field_box: GTKBox,
    monitor_active_switch: Switch,
    monitor_port_label: Label,
    monitor_display_label: Label,
    width_spin_button: SpinButton,
    height_spin_button: SpinButton,
    refresh_rate_spin_button: SpinButton,
    resolution_scale_spin_button: SpinButton,
    display_mode_selection_box: SelectionBox,
}

impl Control for DisplayField {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.display_field_box
    }
}

impl UpdatableControl<DisplayFieldState> for DisplayField {
    fn update_ui(&mut self, state: DisplayFieldState) {
        self.monitor_active_switch.set_active(state.monitor_configuration.enabled);

        if state.monitor_configuration.enabled {
            self.enable_control()
        } else {
            self.disable_control()
        }

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
            climb_rate: RESOLUTION_CLIMB_RATE,
            min_value: min_video_mode.width_resolution as f64,
            max_value: max_video_mode.width_resolution as f64,
            digit_count: FLOAT_DIGITS,
            use_integral_numbers: false,
            current_value: current_video_mode.width_resolution as f64,
            increment_value: RESOLUTION_INCREMENT,
            page_size: 0.0,
            page_increment_value: RESOLUTION_PAGE_INCREMENT
        };
        self.width_spin_button.update_ui(width_button_state);

        let height_spin_button_state = SpinButtonState {
            label_text: "Height:".to_string(),
            climb_rate: RESOLUTION_CLIMB_RATE,
            min_value: min_video_mode.height_resolution as f64,
            max_value: max_video_mode.height_resolution as f64,
            digit_count: FLOAT_DIGITS,
            use_integral_numbers: false,
            current_value: current_video_mode.height_resolution as f64,
            increment_value: RESOLUTION_INCREMENT,
            page_size: 0.0,
            page_increment_value: RESOLUTION_PAGE_INCREMENT
        };
        self.height_spin_button.update_ui(height_spin_button_state);

        let refresh_rate_spin_button_state = SpinButtonState {
            label_text: "Refresh Rate:".to_string(),
            climb_rate: REFRESH_CLIMB_RATE,
            min_value: min_video_mode.refresh_rate as f64,
            max_value: max_video_mode.refresh_rate as f64,
            digit_count: FLOAT_DIGITS,
            use_integral_numbers: false,
            current_value: current_video_mode.refresh_rate as f64,
            increment_value: REFRESH_INCREMENT,
            page_size: 0.0,
            page_increment_value: REFRESH_PAGE_INCREMENT
        };
        self.refresh_rate_spin_button.update_ui(refresh_rate_spin_button_state);

        let resolution_scale_spin_button_state = SpinButtonState {
            label_text: "Scale:".to_string(),
            climb_rate: 0.05,
            min_value: 1.0,
            max_value: 5.0,
            digit_count: 3,
            use_integral_numbers: false,
            current_value: state.monitor_configuration.resolution_scale as f64,
            increment_value: 0.10,
            page_size: 0.0,
            page_increment_value: 0.5
        };
        self.resolution_scale_spin_button.update_ui(resolution_scale_spin_button_state);

        let video_mode_selection_box_state = SelectionBoxState {
            label_text: "Rotation:".to_string(),
            selected_option: Some(state.monitor_configuration.orientation.to_string()),
            options: vec![
                "None".to_string(), "90°".to_string(), "180°".to_string(),
                "270°".to_string(), "Flipped".to_string(), "90° Flipped".to_string(),
                "180° Flipped".to_string(), "270° Flipped".to_string(),
            ],
        };
        self.display_mode_selection_box.update_state(video_mode_selection_box_state.clone());
        self.display_mode_selection_box.update_ui(video_mode_selection_box_state.clone());
    }
}

impl ActivableControl for DisplayField {
    fn enable_control(&self) {
        self.width_spin_button.enable_control();
        self.height_spin_button.enable_control();
        self.refresh_rate_spin_button.enable_control();
        self.display_mode_selection_box.enable_control();
    }

    fn disable_control(&self) {
        self.width_spin_button.disable_control();
        self.height_spin_button.disable_control();
        self.refresh_rate_spin_button.disable_control();
        self.display_mode_selection_box.disable_control();
    }
}

impl DisplayField {
    pub fn new() -> Self {
        const SIZE_BOX_LABEL_WIDTH: u32 = 50;
        const REFRESH_BOX_LABEL_WIDTH: u32 = 100;

        let display_field_box = BoxBuilder::new("display-field")
            .set_orientation(Orientation::Vertical)
            .build();

        let monitor_info_box = BoxBuilder::new("monitor-infos")
            .set_orientation(Orientation::Horizontal)
            .build();

        let video_setting_box = BoxBuilder::new("video-settings")
            .set_orientation(Orientation::Horizontal)
            .build();

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

        let size_field_box = BoxBuilder::new("size-field")
            .set_orientation(Orientation::Vertical)
            .build();

        let width_spin_button = SpinButton::new();
        width_spin_button.set_text_width(SIZE_BOX_LABEL_WIDTH);

        let height_spin_button = SpinButton::new();
        height_spin_button.set_text_width(SIZE_BOX_LABEL_WIDTH);

        size_field_box.append(width_spin_button.get_widget());
        size_field_box.append(height_spin_button.get_widget());

        let refresh_rate_box = BoxBuilder::new("refresh-rate-box")
            .set_orientation(Orientation::Vertical)
            .build();

        let refresh_rate_spin_button = SpinButton::new();
        refresh_rate_spin_button.set_text_width(REFRESH_BOX_LABEL_WIDTH);

        let resolution_scale_spin_button = SpinButton::new();
        resolution_scale_spin_button.set_text_width(REFRESH_BOX_LABEL_WIDTH);

        refresh_rate_box.append(refresh_rate_spin_button.get_widget());
        refresh_rate_box.append(resolution_scale_spin_button.get_widget());

        let rotation_mode_box = BoxBuilder::new("rotation-box")
            .set_orientation(Orientation::Vertical)
            .build();

        let rotation_mode_selection_box = SelectionBox::new();
        rotation_mode_box.append(rotation_mode_selection_box.get_widget());

        video_setting_box.append(&size_field_box);
        video_setting_box.append(&refresh_rate_box);
        video_setting_box.append(&rotation_mode_box);

        display_field_box.append(&monitor_info_box);
        display_field_box.append(&video_setting_box);

        Self {
            display_field_box,
            monitor_active_switch,
            monitor_port_label,
            monitor_display_label,
            width_spin_button,
            height_spin_button,
            refresh_rate_spin_button,
            resolution_scale_spin_button,
            display_mode_selection_box: rotation_mode_selection_box
        }
    }

    pub fn set_active_change(&self, value_change: impl Fn(&Switch, bool) -> Propagation + 'static) {
        self.monitor_active_switch.connect_state_set(value_change);
    }

    pub fn set_width_change(&self, value_change: impl Fn(&GTKSpinButton) + 'static) {
        self.width_spin_button.set_value_change(value_change);
    }

    pub fn set_height_change(&self, value_change: impl Fn(&GTKSpinButton) + 'static) {
        self.height_spin_button.set_value_change(value_change);
    }
    
    pub fn set_refresh_rate_change(&self, value_change: impl Fn(&GTKSpinButton) + 'static) {
        self.refresh_rate_spin_button.set_value_change(value_change);
    }

    pub fn set_resolution_scale_change(&self, value_change: impl Fn(&GTKSpinButton) + 'static) {
        self.resolution_scale_spin_button.set_value_change(value_change);
    }

    pub fn set_orientation_change(&self, value_change: impl Fn(&ComboBoxText) + 'static) {
        self.display_mode_selection_box.set_selection_change(value_change);
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


