use gtk::{ComboBoxText, Entry, Orientation};
use gtk::prelude::{BoxExt, EditableExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::providers::input_provider::InputProvider;
use crate::types::{GTKBox, GTKSpinButton};
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::controls::Control;
use crate::ui::controls::input_field::InputField;
use crate::ui::controls::selection_box::SelectionBox;
use crate::ui::controls::spin_button::SpinButton;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::states::selection_box_state::SelectionBoxState;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::RcMut;

const INPUT_LABEL_WIDTH: u32 = 180;

pub struct Input {
    input_box: GTKBox,
}

impl Control for Input {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.input_box
    }
}

impl Input {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let input_provider = application_provider.get_input_provider();

        let input_box = BoxBuilder::new("input_box")
            .set_orientation(Orientation::Vertical)
            .set_margin(DEFAULT_MARGIN)
            .build();

        let keyboard_section = Self::create_keyboard_section(&input_provider);
        let mouse_section = Self::create_mouse_section(&input_provider);

        input_box.append(&keyboard_section);
        input_box.append(&mouse_section);

        Self {
            input_box
        }
    }

    fn create_keyboard_section(input_provider: &RcMut<InputProvider>) -> GTKBox {
        let keyboard_section = SectionBoxBuilder::new("keyboard-section-box", 0)
            .create_header_elements("Keyboard")
            .build().expect("Failed to create keyboard section");

        let input_provider_clone = input_provider.clone();
        let layout_input_field_change = move |entry: &Entry| {
            input_provider_clone.borrow_mut().set_keyboard_layout(entry.text().to_string());
        };
        let state = InputFieldState {
            label_text: "Layout".to_string(),
            entry_text: Some(input_provider.borrow().get_keyboard_layout()),
            placeholder_text: "e.g us".to_string(),
        };
        let mut layout_input_field = InputField::new();
        layout_input_field.update_state(state);
        layout_input_field.set_input_callback(layout_input_field_change);


        let input_provider_clone = input_provider.clone();
        let numlock_enabled_selection_box_change = move |combobox: &ComboBoxText| {
            let bool_value = SelectionBox::parse_selection_as_bool(combobox.active_text());
            input_provider_clone.borrow_mut().set_numlock_enabled(bool_value);
        };
        let state = SelectionBoxState {
            label_text: "Numlock enabled:".to_string(),
            selected_option: Some(input_provider.borrow().get_numlock_enabled().to_string()),
            options: SelectionBoxState::get_false_true_options(),
        };
        let mut numlock_enabled = SelectionBox::new();
        numlock_enabled.set_text_width(INPUT_LABEL_WIDTH);
        numlock_enabled.update_state(state.clone());
        numlock_enabled.update_state(state.clone());
        numlock_enabled.set_selection_change(numlock_enabled_selection_box_change);


        let input_provider_clone = input_provider.clone();
        let repeat_rate_spin_button_change = move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_keyboard_repeat_rate(spin_button.value() as u32);
        };
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
        let mut repeat_rate_spin_button = SpinButton::new();
        repeat_rate_spin_button.set_text_width(INPUT_LABEL_WIDTH);
        repeat_rate_spin_button.update_state(state);
        repeat_rate_spin_button.set_value_change(repeat_rate_spin_button_change);


        let input_provider_clone = input_provider.clone();
        let repeat_delay_spin_button_change = move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_keyboard_repeat_delay(spin_button.value() as u32);
        };
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
        let mut repeat_delay_spin_button = SpinButton::new();
        repeat_delay_spin_button.set_text_width(INPUT_LABEL_WIDTH);
        repeat_delay_spin_button.update_state(state);
        repeat_delay_spin_button.set_value_change(repeat_delay_spin_button_change);

        keyboard_section.append(layout_input_field.get_widget());
        keyboard_section.append(numlock_enabled.get_widget());
        keyboard_section.append(repeat_rate_spin_button.get_widget());
        keyboard_section.append(repeat_delay_spin_button.get_widget());
        keyboard_section
    }

    fn create_mouse_section(input_provider: &RcMut<InputProvider>) -> GTKBox {
        let mouse_section = SectionBoxBuilder::new("mouse-section-box", 0)
            .create_header_elements("Mouse")
            .build().expect("Failed to create mouse section");

        let input_provider_clone = input_provider.clone();
        let sensitivity_spin_button_change = move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_mouse_sensitivity(spin_button.value() as f32);
        };
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
        let mut sensitivity_spin_button = SpinButton::new();
        sensitivity_spin_button.set_text_width(INPUT_LABEL_WIDTH);
        sensitivity_spin_button.update_state(state);
        sensitivity_spin_button.set_value_change(sensitivity_spin_button_change);

        let input_provider_clone = input_provider.clone();
        let left_handed_selection_box_change = move |combobox: &ComboBoxText| {
            let bool_value = SelectionBox::parse_selection_as_bool(combobox.active_text());
            input_provider_clone.borrow_mut().set_mouse_left_handed(bool_value);
        };
        let state = SelectionBoxState {
            label_text: "Left handed:".to_string(),
            selected_option: Some(input_provider.borrow().get_mouse_left_handed().to_string()),
            options: SelectionBoxState::get_false_true_options(),
        };
        let mut left_handed_selection_box = SelectionBox::new();
        left_handed_selection_box.set_text_width(INPUT_LABEL_WIDTH);
        left_handed_selection_box.update_state(state.clone());
        left_handed_selection_box.update_state(state.clone());
        left_handed_selection_box.set_selection_change(left_handed_selection_box_change);

        let input_provider_clone = input_provider.clone();
        let scroll_factor_spin_button_change = move |spin_button: &GTKSpinButton| {
            input_provider_clone.borrow_mut().set_mouse_scroll_factor(spin_button.value() as f32);
        };
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
        let mut scroll_factor_spin_button = SpinButton::new();
        scroll_factor_spin_button.set_text_width(INPUT_LABEL_WIDTH);
        scroll_factor_spin_button.update_state(state);
        scroll_factor_spin_button.set_value_change(scroll_factor_spin_button_change);

        let input_provider_clone = input_provider.clone();
        let natural_scroll_enabled_selection_box_change = move |combobox: &ComboBoxText| {
            let bool_value = SelectionBox::parse_selection_as_bool(combobox.active_text());
            input_provider_clone.borrow_mut().set_mouse_natural_scroll(bool_value);
        };
        let state = SelectionBoxState {
            label_text: "Natural Scroll:".to_string(),
            selected_option: Some(input_provider.borrow().get_mouse_natural_scroll().to_string()),
            options: SelectionBoxState::get_false_true_options(),
        };
        let mut natural_scroll_enabled_selection_box = SelectionBox::new();
        natural_scroll_enabled_selection_box.set_text_width(INPUT_LABEL_WIDTH);
        natural_scroll_enabled_selection_box.update_state(state.clone());
        natural_scroll_enabled_selection_box.update_state(state.clone());
        natural_scroll_enabled_selection_box.set_selection_change(natural_scroll_enabled_selection_box_change);

        mouse_section.append(sensitivity_spin_button.get_widget());
        mouse_section.append(left_handed_selection_box.get_widget());
        mouse_section.append(scroll_factor_spin_button.get_widget());
        mouse_section.append(natural_scroll_enabled_selection_box.get_widget());
        mouse_section
    }
}