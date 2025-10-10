use gtk::{DropDown, Entry, Orientation, ScrolledWindow};
use gtk::prelude::{BoxExt, EditableExt, WidgetExt};
use crate::models::monitor::monitor_configuration::MonitorOrientation;
use crate::providers::application_provider::ApplicationProvider;
use crate::providers::monitor_provider::MonitorProvider;
use crate::types::{GTKBox, GTKSpinButton};
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::Control;
use crate::ui::controls::input_field::InputField;
use crate::ui::controls::selection_box::SelectionBox;
use crate::ui::controls::spin_button::SpinButton;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::managed_control::ManagedControl;
use crate::ui::manager::input_manager::{InputManager, InputManagerEvent};
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::states::input_state::{InputState, CURRENT_MONITOR};
use crate::ui::states::selection_box_state::SelectionBoxState;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::RcMut;

const INPUT_LABEL_WIDTH: u32 = 180;
const RESOLUTION_INCREMENT: f64 = 1.0;
const RESOLUTION_PAGE_INCREMENT: f64 = 120.0;
const RESOLUTION_CLIMB_RATE: f64 = 10.0;
const RESOLUTION_DIGITS: u32 = 0;

pub struct Input {
    state: InputState,
    application_provider: ApplicationProvider,
    input_scroll_box: GTKBox,
    layout_input_field: InputField,
    numlock_enabled_selection_box: SelectionBox,
    repeat_rate_spin_button: SpinButton,
    repeat_delay_spin_button: SpinButton,
    sensitivity_spin_button: SpinButton,
    left_handed_selection_box: SelectionBox,
    scroll_factor_spin_button: SpinButton,
    natural_scroll_enabled_selection_box: SelectionBox,
    tablet_orientation_selection_box: SelectionBox,
    tablet_monitor_selection_box: SelectionBox,
    tablet_region_xspinbutton: SpinButton,
    tablet_region_yspinbutton: SpinButton,
    tablet_region_width_spinbutton: SpinButton,
    tablet_region_height_spinbutton: SpinButton,
    tablet_relative_input_selection_box: SelectionBox,
    tablet_left_handed_selection_box: SelectionBox,
    tablet_active_area_width_spinbutton: SpinButton,
    tablet_active_area_height_spinbutton: SpinButton,
    tablet_active_area_xspinbutton: SpinButton,
    tablet_active_area_yspinbutton: SpinButton
}

impl Control for Input {
    fn get_widget(&self) -> &GTKBox {
        &self.input_scroll_box
    }
}

impl UpdatableControl<InputState> for Input {
    fn update_state(&mut self, input_state: InputState) {
        let input_provider = self.application_provider.get_input_provider();
        let monitor_provider = self.application_provider.get_monitor_provider();

        let state = InputFieldState {
            label_text: "Layout".to_string(),
            entry_text: Some(input_provider.borrow().get_keyboard_layout()),
            placeholder_text: "e.g us".to_string(),
        };
        self.layout_input_field.update_state(state);

        let state = SelectionBoxState {
            label_text: "Numlock enabled:".to_string(),
            selected_option: Some(input_provider.borrow().get_numlock_enabled().to_string()),
            options: SelectionBoxState::get_false_true_options()
        };
        self.numlock_enabled_selection_box.update_state(state.clone());

        let state = SpinButtonState {
            label_text: "Repeat rate:".to_string(),
            min_value: 1.0,
            max_value: 10000.0,
            current_value: input_provider.borrow().get_keyboard_repeat_rate() as f64,
            increment_value: 1.0,
            page_increment_value: 10.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        self.repeat_rate_spin_button.update_state(state);

        let state = SpinButtonState {
            label_text: "Repeat delay:".to_string(),
            min_value: 1.0,
            max_value: 10000.0,
            current_value: input_provider.borrow().get_keyboard_repeat_delay() as f64,
            increment_value: 1.0,
            page_increment_value: 10.0,
            page_size: 0.0,
            climb_rate: 20.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        self.repeat_delay_spin_button.update_state(state);

        let state = SpinButtonState {
            label_text: "Sensitivity:".to_string(),
            min_value: 1.0,
            max_value: 200.0,
            current_value: input_provider.borrow().get_mouse_sensitivity() as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 2,
            use_integral_numbers: false,
        };
        self.sensitivity_spin_button.update_state(state);

        let state = SelectionBoxState {
            label_text: "Left handed:".to_string(),
            selected_option: Some(input_provider.borrow().get_mouse_left_handed().to_string()),
            options: SelectionBoxState::get_false_true_options()
        };
        self.left_handed_selection_box.update_state(state.clone());

        let state = SpinButtonState {
            label_text: "Scroll factor:".to_string(),
            min_value: 1.0,
            max_value: 100.0,
            current_value: input_provider.borrow().get_mouse_scroll_factor() as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        self.scroll_factor_spin_button.update_state(state);

        let state = SelectionBoxState {
            label_text: "Natural Scroll:".to_string(),
            selected_option: Some(input_provider.borrow().get_mouse_natural_scroll().to_string()),
            options: SelectionBoxState::get_false_true_options(),
        };
        self.natural_scroll_enabled_selection_box.update_state(state.clone());

        let orientation_options = MonitorOrientation::get_orientation_option_names();
        let state = SelectionBoxState {
            label_text: "Orientation:".to_string(),
            selected_option: Some(input_state.tablet_orientation.to_string()),
            options: orientation_options,
        };
        self.tablet_orientation_selection_box.update_state(state);

        let state = SelectionBoxState {
            label_text: "Monitor:".to_string(),
            selected_option: Some(input_state.tablet_monitor.to_string()),
            options: Self::get_monitor_option_names(&monitor_provider),
        };
        self.tablet_monitor_selection_box.update_state(state);

        let state = SelectionBoxState {
            label_text: "Relative input:".to_string(),
            selected_option: Some(input_state.tablet_relative_input.to_string()),
            options: SelectionBoxState::get_false_true_options(),
        };
        self.tablet_relative_input_selection_box.update_state(state);

        let state = SelectionBoxState {
            label_text: "Left handed:".to_string(),
            selected_option: Some(input_state.tablet_left_handed.to_string()),
            options: SelectionBoxState::get_false_true_options(),
        };
        self.tablet_left_handed_selection_box.update_state(state);

        self.update_tablet_region(input_state.clone());
        self.state = input_state;
    }

    fn get_current_state(&self) -> InputState {
        self.state.clone()
    }
}

impl ManagedControl<InputManager> for Input {
    fn init_events_by_manager(&self, input_manager: InputManager) {
        let input_provider = self.application_provider.get_input_provider();
        let monitor_provider = self.application_provider.get_monitor_provider();

        let input_provider_clone = input_provider.clone();
        self.layout_input_field.set_input_callback(move |entry: &Entry| {
            input_provider_clone.borrow_mut().set_keyboard_layout(entry.text().to_string());
        });

        let input_provider_clone = input_provider.clone();
        self.numlock_enabled_selection_box.set_selection_change(move |dropdown: &DropDown| {
            let bool_value = SelectionBox::get_selected_option_as_bool(dropdown);
            input_provider_clone.borrow_mut().set_numlock_enabled(bool_value);
        });

        let input_provider_clone = input_provider.clone();
        self.repeat_rate_spin_button.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_keyboard_repeat_rate(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.repeat_delay_spin_button.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_keyboard_repeat_delay(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.sensitivity_spin_button.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_mouse_sensitivity(spin_button.value() as f32);
        });

        let input_provider_clone = input_provider.clone();
        self.left_handed_selection_box.set_selection_change(move |dropdown: &DropDown| {
            let bool_value = SelectionBox::get_selected_option_as_bool(dropdown);
            input_provider_clone.borrow_mut().set_mouse_left_handed(bool_value);
        });

        let input_provider_clone = input_provider.clone();
        self.scroll_factor_spin_button.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_mouse_scroll_factor(spin_button.value() as f32);
        });

        let input_provider_clone = input_provider.clone();
        self.natural_scroll_enabled_selection_box.set_selection_change(move |dropdown: &DropDown| {
            let bool_value = SelectionBox::get_selected_option_as_bool(dropdown);
            input_provider_clone.borrow_mut().set_mouse_natural_scroll(bool_value);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_orientation_selection_box.set_selection_change(move |dropdown: &DropDown| {
            let selected_text = SelectionBox::get_selected_option(dropdown);
            input_provider_clone.borrow_mut().set_tablet_orientation(MonitorOrientation::from(selected_text));
        });

        let input_provider_clone = input_provider.clone();
        let monitor_provider_clone = monitor_provider.clone();
        self.tablet_monitor_selection_box.set_selection_change(move |dropdown: &DropDown| {
            let selected_monitor = SelectionBox::get_selected_option(dropdown);
            input_provider_clone.borrow_mut().set_tablet_monitor(selected_monitor.clone());

            let selected_configuration = monitor_provider_clone.borrow()
                .get_monitor_configuration(selected_monitor.clone());
            input_manager.send_event(
                InputManagerEvent::MonitorChanged(selected_monitor, selected_configuration)
            );
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_region_xspinbutton.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_tablet_region_x(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_region_yspinbutton.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_tablet_region_y(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_region_width_spinbutton.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_tablet_region_width(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_region_height_spinbutton.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_tablet_region_height(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_relative_input_selection_box.set_selection_change(move |dropdown: &DropDown| {
            let selected_option = SelectionBox::get_selected_option_as_bool(dropdown);
            input_provider_clone.borrow_mut().set_tablet_relative_input(selected_option);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_left_handed_selection_box.set_selection_change(move |dropdown: &DropDown| {
            let selected_option = SelectionBox::get_selected_option_as_bool(dropdown);
            input_provider_clone.borrow_mut().set_tablet_left_handed(selected_option);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_active_area_width_spinbutton.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_tablet_active_width(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_active_area_height_spinbutton.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_tablet_active_height(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_active_area_xspinbutton.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_tablet_active_x(spin_button.value() as u32);
        });

        let input_provider_clone = input_provider.clone();
        self.tablet_active_area_yspinbutton.set_value_change(move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_tablet_active_y(spin_button.value() as u32);
        });
    }
}

impl Input {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let input_box = BoxBuilder::new("input_box")
            .set_orientation(Orientation::Vertical)
            .set_margin(DEFAULT_MARGIN)
            .build();

        let input_scroll_window = ScrolledWindow::new();
        input_scroll_window.set_widget_name("input_scroll_window");
        input_scroll_window.set_vexpand(true);
        input_scroll_window.set_child(Some(&input_box));

        let input_scroll_box = BoxBuilder::new("input_scroll_box")
            .set_orientation(Orientation::Vertical)
            .build();
        input_scroll_box.append(&input_scroll_window);

        let mut layout_input_field = InputField::new();
        let mut numlock_enabled_selection_box = SelectionBox::new();
        let mut repeat_rate_spin_button = SpinButton::new();
        let mut repeat_delay_spin_button = SpinButton::new();
        let keyboard_section = Self::create_keyboard_section(
            &mut layout_input_field, &mut numlock_enabled_selection_box,
            &mut repeat_rate_spin_button, &mut repeat_delay_spin_button
        );

        let mut sensitivity_spin_button = SpinButton::new();
        let mut left_handed_selection_box = SelectionBox::new();
        let mut scroll_factor_spin_button = SpinButton::new();
        let mut natural_scroll_enabled_selection_box = SelectionBox::new();
        let mouse_section = Self::create_mouse_section(
            &mut sensitivity_spin_button, &mut left_handed_selection_box,
            &mut scroll_factor_spin_button, &mut natural_scroll_enabled_selection_box
        );

        let mut tablet_orientation_selection_box = SelectionBox::new();
        let mut tablet_monitor_selection_box = SelectionBox::new();
        let mut tablet_region_xspinbutton = SpinButton::new();
        let mut tablet_region_yspinbutton = SpinButton::new();
        let mut tablet_region_width_spinbutton = SpinButton::new();
        let mut tablet_region_height_spinbutton = SpinButton::new();
        let mut tablet_relative_input_selection_box = SelectionBox::new();
        let mut tablet_left_handed_selection_box = SelectionBox::new();
        let mut tablet_active_area_width_spinbutton = SpinButton::new();
        let mut tablet_active_area_height_spinbutton = SpinButton::new();
        let mut tablet_active_area_xspinbutton = SpinButton::new();
        let mut tablet_active_area_yspinbutton = SpinButton::new();
        let tablet_section = Self::create_tablet_section(
            &mut tablet_orientation_selection_box, &mut tablet_monitor_selection_box,
            &mut tablet_region_xspinbutton, &mut tablet_region_yspinbutton,
            &mut tablet_region_width_spinbutton, &mut tablet_region_height_spinbutton,
            &mut tablet_relative_input_selection_box, &mut tablet_left_handed_selection_box,
            &mut tablet_active_area_width_spinbutton, &mut tablet_active_area_height_spinbutton,
            &mut tablet_active_area_xspinbutton, &mut tablet_active_area_yspinbutton
        );

        input_box.append(&keyboard_section);
        input_box.append(&mouse_section);
        input_box.append(&tablet_section);

        let state = Default::default();
        Self {
            state,
            application_provider,
            input_scroll_box,
            layout_input_field,
            numlock_enabled_selection_box,
            repeat_rate_spin_button,
            repeat_delay_spin_button,
            sensitivity_spin_button,
            left_handed_selection_box,
            scroll_factor_spin_button,
            natural_scroll_enabled_selection_box,
            tablet_orientation_selection_box,
            tablet_monitor_selection_box,
            tablet_region_xspinbutton,
            tablet_region_yspinbutton,
            tablet_region_width_spinbutton,
            tablet_region_height_spinbutton,
            tablet_relative_input_selection_box,
            tablet_left_handed_selection_box,
            tablet_active_area_width_spinbutton,
            tablet_active_area_height_spinbutton,
            tablet_active_area_xspinbutton,
            tablet_active_area_yspinbutton
        }
    }

    fn create_keyboard_section(
        layout_input_field: &mut InputField, numlock_enabled_selection_box: &mut SelectionBox,
        repeat_rate_spin_button: &mut SpinButton, repeat_delay_spin_button: &mut SpinButton,
    ) -> GTKBox {
        let keyboard_section_box = SectionBoxBuilder::new("keyboard-section", 0)
            .create_header_elements("Keyboard")
            .build().expect("Failed to create keyboard section");

        numlock_enabled_selection_box.set_text_width(INPUT_LABEL_WIDTH);
        repeat_rate_spin_button.set_text_width(INPUT_LABEL_WIDTH);
        repeat_delay_spin_button.set_text_width(INPUT_LABEL_WIDTH);

        keyboard_section_box.append(layout_input_field.get_widget());
        keyboard_section_box.append(numlock_enabled_selection_box.get_widget());
        keyboard_section_box.append(repeat_rate_spin_button.get_widget());
        keyboard_section_box.append(repeat_delay_spin_button.get_widget());
        keyboard_section_box
    }

    fn create_mouse_section(
        sensitivity_spin_button: &mut SpinButton, left_handed_selection_box: &mut SelectionBox,
        scroll_factor_spin_button: &mut SpinButton, natural_scroll_enabled_selection_box: &mut SelectionBox,
    ) -> GTKBox {
        let mouse_section_box = SectionBoxBuilder::new("mouse-section", 0)
            .create_header_elements("Mouse")
            .build().expect("Failed to create mouse section");

        sensitivity_spin_button.set_text_width(INPUT_LABEL_WIDTH);
        left_handed_selection_box.set_text_width(INPUT_LABEL_WIDTH);
        scroll_factor_spin_button.set_text_width(INPUT_LABEL_WIDTH);
        natural_scroll_enabled_selection_box.set_text_width(INPUT_LABEL_WIDTH);

        mouse_section_box.append(sensitivity_spin_button.get_widget());
        mouse_section_box.append(left_handed_selection_box.get_widget());
        mouse_section_box.append(scroll_factor_spin_button.get_widget());
        mouse_section_box.append(natural_scroll_enabled_selection_box.get_widget());
        mouse_section_box
    }

    fn create_tablet_section(
        orientation_selection_box: &mut SelectionBox, monitor_selection_box: &mut SelectionBox,
        region_xspinbutton: &mut SpinButton, region_yspinbutton: &mut SpinButton,
        region_width_spinbutton: &mut SpinButton, region_height_spinbutton: &mut SpinButton,
        relative_input_selection_box: &mut SelectionBox, left_handed_selection_box: &mut SelectionBox,
        active_area_width_spinbutton: &mut SpinButton, active_area_height_spinbutton: &mut SpinButton,
        active_area_xspinbutton: &mut SpinButton, active_area_yspinbutton: &mut SpinButton
    ) -> GTKBox {
        let tablet_section_box = SectionBoxBuilder::new("tablet-section", 0)
            .create_header_elements("Tablet")
            .build().expect("Could not create tablet section");

        orientation_selection_box.set_text_width(INPUT_LABEL_WIDTH);
        monitor_selection_box.set_text_width(INPUT_LABEL_WIDTH);
        region_xspinbutton.set_text_width(INPUT_LABEL_WIDTH);
        region_yspinbutton.set_text_width(INPUT_LABEL_WIDTH);
        region_width_spinbutton.set_text_width(INPUT_LABEL_WIDTH);
        region_height_spinbutton.set_text_width(INPUT_LABEL_WIDTH);
        relative_input_selection_box.set_text_width(INPUT_LABEL_WIDTH);
        left_handed_selection_box.set_text_width(INPUT_LABEL_WIDTH);
        active_area_width_spinbutton.set_text_width(INPUT_LABEL_WIDTH);
        active_area_height_spinbutton.set_text_width(INPUT_LABEL_WIDTH);
        active_area_xspinbutton.set_text_width(INPUT_LABEL_WIDTH);
        active_area_yspinbutton.set_text_width(INPUT_LABEL_WIDTH);

        tablet_section_box.append(orientation_selection_box.get_widget());
        tablet_section_box.append(monitor_selection_box.get_widget());
        tablet_section_box.append(region_xspinbutton.get_widget());
        tablet_section_box.append(region_yspinbutton.get_widget());
        tablet_section_box.append(region_width_spinbutton.get_widget());
        tablet_section_box.append(region_height_spinbutton.get_widget());
        tablet_section_box.append(relative_input_selection_box.get_widget());
        tablet_section_box.append(left_handed_selection_box.get_widget());
        tablet_section_box.append(active_area_width_spinbutton.get_widget());
        tablet_section_box.append(active_area_height_spinbutton.get_widget());
        tablet_section_box.append(active_area_xspinbutton.get_widget());
        tablet_section_box.append(active_area_yspinbutton.get_widget());
        tablet_section_box
    }

    pub fn update_tablet_region(&mut self, input_state: InputState) {
        let monitor_provider = self.application_provider.get_monitor_provider();

        let tablet_max_region_size = input_state.get_tablet_max_region_size(monitor_provider);
        let state = SpinButtonState {
            label_text: "Region x:".to_string(),
            min_value: 0.0,
            max_value: tablet_max_region_size.get_x(),
            current_value: input_state.tablet_region_position.get_x(),
            increment_value: RESOLUTION_INCREMENT,
            page_increment_value: RESOLUTION_PAGE_INCREMENT,
            page_size: 0.0,
            climb_rate: RESOLUTION_CLIMB_RATE,
            digit_count: RESOLUTION_DIGITS,
            use_integral_numbers: false,
        };
        self.tablet_region_xspinbutton.update_state(state);

        let state = SpinButtonState {
            label_text: "Region y:".to_string(),
            min_value: 0.0,
            max_value: tablet_max_region_size.get_y(),
            current_value: input_state.tablet_region_position.get_y(),
            increment_value: RESOLUTION_INCREMENT,
            page_increment_value: RESOLUTION_PAGE_INCREMENT,
            page_size: 0.0,
            climb_rate: RESOLUTION_CLIMB_RATE,
            digit_count: RESOLUTION_DIGITS,
            use_integral_numbers: false,
        };
        self.tablet_region_yspinbutton.update_state(state);

        let state = SpinButtonState {
            label_text: "Region width:".to_string(),
            min_value: 0.0,
            max_value: tablet_max_region_size.get_x(),
            current_value: input_state.tablet_region_size.get_x(),
            increment_value: RESOLUTION_INCREMENT,
            page_increment_value: RESOLUTION_PAGE_INCREMENT,
            page_size: 0.0,
            climb_rate: RESOLUTION_CLIMB_RATE,
            digit_count: RESOLUTION_DIGITS,
            use_integral_numbers: false,
        };
        self.tablet_region_width_spinbutton.update_state(state);

        let state = SpinButtonState {
            label_text: "Region height:".to_string(),
            min_value: 0.0,
            max_value: tablet_max_region_size.get_x(),
            current_value: input_state.tablet_region_size.get_y(),
            increment_value: RESOLUTION_INCREMENT,
            page_increment_value: RESOLUTION_PAGE_INCREMENT,
            page_size: 0.0,
            climb_rate: RESOLUTION_CLIMB_RATE,
            digit_count: RESOLUTION_DIGITS,
            use_integral_numbers: false,
        };
        self.tablet_region_height_spinbutton.update_state(state);

        let state = SpinButtonState {
            label_text: "Active width:".to_string(),
            min_value: 0.0,
            max_value: 100000.0, // TODO: fetch tablet information for max screen size
            current_value: input_state.tablet_active_size.get_x(),
            increment_value: RESOLUTION_INCREMENT,
            page_increment_value: RESOLUTION_PAGE_INCREMENT,
            page_size: 0.0,
            climb_rate: RESOLUTION_CLIMB_RATE,
            digit_count: RESOLUTION_DIGITS,
            use_integral_numbers: false,
        };
        self.tablet_active_area_width_spinbutton.update_state(state);

        let state = SpinButtonState {
            label_text: "Active height:".to_string(),
            min_value: 0.0,
            max_value: 100000.0, // TODO: fetch tablet information for max screen size
            current_value: input_state.tablet_active_size.get_y(),
            increment_value: RESOLUTION_INCREMENT,
            page_increment_value: RESOLUTION_PAGE_INCREMENT,
            page_size: 0.0,
            climb_rate: RESOLUTION_CLIMB_RATE,
            digit_count: RESOLUTION_DIGITS,
            use_integral_numbers: false,
        };
        self.tablet_active_area_height_spinbutton.update_state(state);

        let state = SpinButtonState {
            label_text: "Active x:".to_string(),
            min_value: 0.0,
            max_value: 100000.0, // TODO: fetch tablet information for max screen size
            current_value: input_state.tablet_active_position.get_x(),
            increment_value: RESOLUTION_INCREMENT,
            page_increment_value: RESOLUTION_PAGE_INCREMENT,
            page_size: 0.0,
            climb_rate: RESOLUTION_CLIMB_RATE,
            digit_count: RESOLUTION_DIGITS,
            use_integral_numbers: false,
        };
        self.tablet_active_area_xspinbutton.update_state(state);

        let state = SpinButtonState {
            label_text: "Active y:".to_string(),
            min_value: 0.0,
            max_value: 100000.0, // TODO: fetch tablet information for max screen size
            current_value: input_state.tablet_active_position.get_y(),
            increment_value: RESOLUTION_INCREMENT,
            page_increment_value: RESOLUTION_PAGE_INCREMENT,
            page_size: 0.0,
            climb_rate: RESOLUTION_CLIMB_RATE,
            digit_count: RESOLUTION_DIGITS,
            use_integral_numbers: false,
        };
        self.tablet_active_area_yspinbutton.update_state(state);
    }

    pub fn toggle_tablet_region(&self, state: bool) {
        if state {
            self.tablet_region_xspinbutton.enable_control();
            self.tablet_region_yspinbutton.enable_control();
            self.tablet_region_width_spinbutton.enable_control();
            self.tablet_region_height_spinbutton.enable_control();
            self.tablet_active_area_width_spinbutton.enable_control();
            self.tablet_active_area_height_spinbutton.enable_control();
            self.tablet_active_area_xspinbutton.enable_control();
            self.tablet_active_area_yspinbutton.enable_control();
        } else {
            self.tablet_region_xspinbutton.disable_control();
            self.tablet_region_yspinbutton.disable_control();
            self.tablet_region_width_spinbutton.disable_control();
            self.tablet_region_height_spinbutton.disable_control();
            self.tablet_active_area_width_spinbutton.disable_control();
            self.tablet_active_area_height_spinbutton.disable_control();
            self.tablet_active_area_xspinbutton.disable_control();
            self.tablet_active_area_yspinbutton.disable_control();
        }
    }

    fn get_monitor_option_names(monitor_provider: &RcMut<MonitorProvider>) -> Vec<String> {
        let mut monitors = monitor_provider.borrow()
            .get_monitor_configurations()
            .iter()
            .map(|(port_name, _)| {
                port_name.clone()
            })
            .collect::<Vec<String>>();
        monitors.push(CURRENT_MONITOR.to_string());

        monitors
    }
}