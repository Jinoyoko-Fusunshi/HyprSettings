use gtk::{ColorButton, ComboBoxText, Entry, Orientation, ScrolledWindow};
use gtk::prelude::{BoxExt, ColorChooserExt, EditableExt, WidgetExt};
use crate::models::rgba_color::RGBAColor;
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::boxes::Boxes;
use crate::ui::controls::color_selector::ColorSelector;
use crate::ui::controls::Control;
use crate::ui::controls::input_field::InputField;
use crate::ui::controls::selection_box::SelectionBox;
use crate::ui::controls::spin_button::SpinButton;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::statable_control::StatableControl;
use crate::ui::states::color_selector_state::ColorSelectorState;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::states::lockscreen_page_state::LockScreenPageState;
use crate::ui::states::selection_box_state::SelectionBoxState;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_control::UpdatableControl;

const LOCKSCREEN_LABEL_WIDTH: u32 = 180;

pub struct Lockscreen {
    application_provider: ApplicationProvider,
    state: LockScreenPageState,
    lockscreen_scroll_box: gtk::Box,
    lockscreen_sections_box: gtk::Box,
}

impl Control for Lockscreen {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.lockscreen_scroll_box
    }
}

impl UpdatableControl<LockScreenPageState> for Lockscreen {
    fn update_ui(&mut self, state: LockScreenPageState) {
        Boxes::clear_box_content(&self.lockscreen_sections_box);

        if state.enabled {
            self.create_lockscreen_sections(&state);
        } else {
            self.create_lockscreen_warning();
        }
    }
}

impl StatableControl<LockScreenPageState> for Lockscreen {
    fn update_state(&mut self, state: LockScreenPageState) {
        self.state = state;
    }
}

impl Lockscreen {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let lockscreen_sections_box = gtk::Box::new(Orientation::Vertical, 10);
        Boxes::set_margin(&lockscreen_sections_box, 10);

        let scroll_window = ScrolledWindow::new();
        scroll_window.set_vexpand(true);
        scroll_window.set_child(Some(&lockscreen_sections_box));

        let lockscreen_scroll_box = gtk::Box::new(Orientation::Vertical, 10);
        lockscreen_scroll_box.set_vexpand(true);
        lockscreen_scroll_box.append(&scroll_window);

        let state = LockScreenPageState {
            enabled: true,
            hide_cursor: false,
            grace: 0.0,
            fall_timeout: 0,
            lockscreen_wallpaper: None,
            blur_size: 0,
            blur_passes: 0,
            noise: 0.0,
            contrast: 0.0,
            brightness: 0.0,
            vibrancy: 0.0,
            input_width: 0,
            input_height: 0,
            input_outline_thickness: 0,
            input_dots_size: 0,
            input_dots_spacing: 0,
            input_dots_center: false,
            input_outer_color: Default::default(),
            input_inner_color: Default::default(),
            input_font_color: Default::default(),
            input_placeholder_text: None,
            hide_input: false,
            input_x_position: 0,
            input_y_position: 0,
            input_vertical_alignment: None,
            input_horizontal_alignment: None,
            display_text: None,
            display_text_color: Default::default(),
            display_text_font_size: 0,
            display_text_font: None,
            display_text_x_position: 0,
            display_text_y_position: 0,
            display_text_vertical_alignment: None,
            display_text_horizontal_alignment: None,
        };

        Self {
            application_provider,
            state,
            lockscreen_scroll_box,
            lockscreen_sections_box,
        }
    }

    fn create_lockscreen_sections(&self, lockscreen_state: &LockScreenPageState) {
        self.lockscreen_sections_box.append(&self.create_general_section(lockscreen_state));
        self.lockscreen_sections_box.append(&self.create_background_section(lockscreen_state));
        self.lockscreen_sections_box.append(&self.create_password_input_field_section(lockscreen_state));
        self.lockscreen_sections_box.append(&self.create_text_display_section_box(lockscreen_state));
    }

    fn create_general_section(&self, lockscreen_state: &LockScreenPageState) -> gtk::Box {
        const GENERAL_TITLE: &str = "General";
        let general_section_box = SectionBoxBuilder::new()
            .create_header_elements(GENERAL_TITLE)
            .build().expect("Failed to create general section box");

        // Password hide cursor selection box
        let mut hide_cursor_selection_box = SelectionBox::new();
        hide_cursor_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SelectionBoxState {
            label_text: "Hide cursor".to_string(),
            selected_option: Some(lockscreen_state.hide_cursor.to_string()),
            options: vec!["false".to_string(), "true".to_string()],
        };
        hide_cursor_selection_box.update_state(state.clone());
        hide_cursor_selection_box.update_ui(state.clone());

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let hide_cursor_selection_change = move |combobox: &ComboBoxText| {
            let bool_value = SelectionBox::parse_selection_as_bool(combobox.active_text());
            lockscreen_provider.borrow_mut().set_hide_cursor(bool_value);
        };
        hide_cursor_selection_box.set_selection_change(hide_cursor_selection_change);

        // Grace spin button
        let mut grace_spin_button = SpinButton::new();
        grace_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Grace".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.grace as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: true,
        };
        grace_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let fall_timeout_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_grace(spin_button.value() as f32);
        };
        grace_spin_button.set_value_change(fall_timeout_change);

        // fall timeout spin button
        let mut fall_timeout = SpinButton::new();
        fall_timeout.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Fall timeout".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.fall_timeout as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: true,
        };
        fall_timeout.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let fall_timeout_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_fall_timeout(spin_button.value() as u32);
        };
        fall_timeout.set_value_change(fall_timeout_change);

        general_section_box.append(hide_cursor_selection_box.get_widget());
        general_section_box.append(grace_spin_button.get_widget());
        general_section_box.append(fall_timeout.get_widget());
        general_section_box
    }

    fn create_background_section(&self, lockscreen_state: &LockScreenPageState) -> gtk::Box {
        const BACKGROUND_TITLE: &str = "Background";
        let general_section_box = SectionBoxBuilder::new()
            .create_header_elements(BACKGROUND_TITLE)
            .build().expect("Failed to create background section box");

        // lockscreen wallpaper input field
        let mut lockscreen_wallpaper_input_field = InputField::new();

        let state = InputFieldState {
            label_text: "Lockscreen wallpaper path".to_string(),
            entry_text: lockscreen_state.lockscreen_wallpaper.clone(),
            placeholder_text: "e.g. ~/Pictures/lockscreen.png".to_string(),
        };
        lockscreen_wallpaper_input_field.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let lockscreen_wallpaper_input_field_change = move |entry: &Entry| {
            lockscreen_provider.borrow_mut().set_lockscreen_wallpaper(entry.text().to_string());
        };
        lockscreen_wallpaper_input_field.set_input_callback(lockscreen_wallpaper_input_field_change);

        // blur size spin button
        let mut blur_size_spin_button = SpinButton::new();
        blur_size_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Blur size".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.blur_size as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        blur_size_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let blur_size_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_lockscreen_blur_size(spin_button.value() as u32);
        };
        blur_size_spin_button.set_value_change(blur_size_spin_button_change);

        // blur passes spin button
        let mut blur_passes_spin_button = SpinButton::new();
        blur_passes_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Blur passes".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.blur_passes as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        blur_passes_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let blur_passes_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_lockscreen_blur_passes(spin_button.value() as u32);
        };
        blur_passes_spin_button.set_value_change(blur_passes_spin_button_change);

        // noise spin button
        let mut noise_spin_button = SpinButton::new();
        noise_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Noise".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.noise as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        noise_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let noise_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_noise(spin_button.value() as f32);
        };
        noise_spin_button.set_value_change(noise_spin_button_change);

        // contrast spin button
        let mut contrast_spin_button = SpinButton::new();
        contrast_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Contrast".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.contrast as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        contrast_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let contrast_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_contrast(spin_button.value() as f32);
        };
        contrast_spin_button.set_value_change(contrast_spin_button_change);

        // brightness spin button
        let mut brightness_spin_button = SpinButton::new();
        brightness_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Brightness".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.brightness as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        brightness_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let brightness_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_brightness(spin_button.value() as f32);
        };
        brightness_spin_button.set_value_change(brightness_spin_button_change);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let brightness_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_brightness(spin_button.value() as f32);
        };
        brightness_spin_button.set_value_change(brightness_spin_button_change);

        // vibrancy spin button
        let mut vibrancy_spin_button = SpinButton::new();
        vibrancy_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Vibrancy".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.vibrancy as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        vibrancy_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let vibrancy_spin_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_vibrancy(spin_button.value() as f32);
        };
        vibrancy_spin_button.set_value_change(vibrancy_spin_change);

        general_section_box.append(lockscreen_wallpaper_input_field.get_widget());
        general_section_box.append(blur_size_spin_button.get_widget());
        general_section_box.append(blur_passes_spin_button.get_widget());
        general_section_box.append(noise_spin_button.get_widget());
        general_section_box.append(contrast_spin_button.get_widget());
        general_section_box.append(brightness_spin_button.get_widget());
        general_section_box.append(vibrancy_spin_button.get_widget());
        general_section_box
    }

    fn create_password_input_field_section(&self, lockscreen_state: &LockScreenPageState) -> gtk::Box {
        const PASSWORD_INPUT_FIELD_TITLE: &str = "Password field";
        let password_input_field_section_box = SectionBoxBuilder::new()
            .create_header_elements(PASSWORD_INPUT_FIELD_TITLE)
            .build().expect("Failed to create text display section box");

        // Password input width spin button
        let mut input_width_spin_button = SpinButton::new();
        input_width_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Input width".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.input_width as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        input_width_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_width_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_input_width(spin_button.value() as u32);
        };
        input_width_spin_button.set_value_change(input_width_spin_button_change);

        // Password input height spin button
        let mut input_height_spin_button = SpinButton::new();
        input_height_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Input height".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.input_height as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        input_height_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_height_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_input_height(spin_button.value() as u32);
        };
        input_height_spin_button.set_value_change(input_height_spin_button_change);

        // Password input outline thickness spin button
        let mut input_outline_thickness_spin_button = SpinButton::new();
        input_outline_thickness_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Outline thickness".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.input_outline_thickness as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        input_outline_thickness_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_outline_thickness_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_input_outline_thickness(spin_button.value() as u32);
        };
        input_outline_thickness_spin_button.set_value_change(input_outline_thickness_spin_button_change);

        // Password input dots size spin button
        let mut input_dots_size_spin_button = SpinButton::new();
        input_dots_size_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Dots size".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.input_dots_size as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        input_dots_size_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_dots_size_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_input_dots_size(spin_button.value() as u32);
        };
        input_dots_size_spin_button.set_value_change(input_dots_size_spin_button_change);

        // Password input dots spacing spin button
        let mut input_dots_spacing_spin_button = SpinButton::new();
        input_dots_spacing_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Dots spacing".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.input_dots_spacing as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        input_dots_spacing_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_dots_spacing_spin_button_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_input_dots_spacing(spin_button.value() as u32);
        };
        input_dots_spacing_spin_button.set_value_change(input_dots_spacing_spin_button_change);

        // Password input dots center selection box
        let mut input_dots_center_selection_box = SelectionBox::new();
        input_dots_center_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SelectionBoxState {
            label_text: "Dots center".to_string(),
            selected_option: Some(lockscreen_state.input_dots_center.to_string()),
            options: vec!["false".to_string(), "true".to_string()],
        };
        input_dots_center_selection_box.update_state(state.clone());
        input_dots_center_selection_box.update_ui(state.clone());

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_dots_center_selection_box_change = move |combo_box_text: &ComboBoxText| {
            let bool_value = SelectionBox::parse_selection_as_bool(combo_box_text.active_text());
            lockscreen_provider.borrow_mut().set_input_dots_center(bool_value);
        };
        input_dots_center_selection_box.set_selection_change(input_dots_center_selection_box_change);

        // Password outer color selector
        let mut outer_color_selector = ColorSelector::new();
        outer_color_selector.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = ColorSelectorState {
            label_text: "Outer color".to_string(),
            selected_color: Some(lockscreen_state.input_outer_color.clone()),
        };
        outer_color_selector.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let outer_color_selector_change = move |color_button: &ColorButton| {
            lockscreen_provider.borrow_mut().set_input_outer_color(RGBAColor::new(color_button.rgba()))
        };
        outer_color_selector.set_color_change(outer_color_selector_change);

        // Password inner color selector
        let mut inner_color_selector = ColorSelector::new();
        inner_color_selector.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = ColorSelectorState {
            label_text: "Inner color".to_string(),
            selected_color: Some(lockscreen_state.input_inner_color.clone()),
        };
        inner_color_selector.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let inner_color_selector_change = move |color_button: &ColorButton| {
            lockscreen_provider.borrow_mut().set_input_inner_color(RGBAColor::new(color_button.rgba()))
        };
        inner_color_selector.set_color_change(inner_color_selector_change);

        // Password font color selector
        let mut font_color_selector = ColorSelector::new();
        font_color_selector.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = ColorSelectorState {
            label_text: "Font color".to_string(),
            selected_color: Some(lockscreen_state.input_font_color.clone()),
        };
        font_color_selector.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let font_color_selector_change = move |color_button: &ColorButton| {
            lockscreen_provider.borrow_mut().set_input_font_color(RGBAColor::new(color_button.rgba()))
        };
        font_color_selector.set_color_change(font_color_selector_change);

        // Password input placeholder text input field
        let mut input_placeholder_text_input_field = InputField::new();

        let state = InputFieldState {
            label_text: "Placeholder text".to_string(),
            entry_text: lockscreen_state.input_placeholder_text.clone(),
            placeholder_text: "e.g. insert password here...".to_string(),
        };
        input_placeholder_text_input_field.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_placeholder_text_change = move |entry: &Entry| {
            lockscreen_provider.borrow_mut().set_input_placeholder_text(entry.text().to_string());
        };
        input_placeholder_text_input_field.set_input_callback(input_placeholder_text_change);

        // Password input hide selection box
        let mut hide_input_selection_box = SelectionBox::new();
        hide_input_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SelectionBoxState {
            label_text: "Hide input".to_string(),
            selected_option: Some(lockscreen_state.hide_input.to_string()),
            options: vec!["false".to_string(), "true".to_string()],
        };
        hide_input_selection_box.update_state(state.clone());
        hide_input_selection_box.update_ui(state.clone());

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let hide_input_selection_box_change = move |combo_box_text: &ComboBoxText| {
            let bool_value = SelectionBox::parse_selection_as_bool(combo_box_text.active_text());
            lockscreen_provider.borrow_mut().set_hide_input(bool_value);
        };
        hide_input_selection_box.set_selection_change(hide_input_selection_box_change);

        // Password input x position
        let mut input_x_position = SpinButton::new();
        input_x_position.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "X-Position".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.input_x_position as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 3.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        input_x_position.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_x_position_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_input_x_position(spin_button.value() as u32);
        };
        input_x_position.set_value_change(input_x_position_change);

        // Password input y position
        let mut input_y_position = SpinButton::new();
        input_y_position.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Y-Position".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.input_y_position as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 3.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        input_y_position.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let input_y_position_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_input_y_position(spin_button.value() as u32);
        };
        input_y_position.set_value_change(input_y_position_change);

        // Password input vertical align selection box
        let mut vertical_align_selection_box = SelectionBox::new();
        vertical_align_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SelectionBoxState {
            label_text: "Vertical alignment".to_string(),
            selected_option: lockscreen_state.input_vertical_alignment.clone(),
            options: vec!["top".to_string(), "center".to_string(), "bottom".to_string()],
        };
        vertical_align_selection_box.update_state(state.clone());
        vertical_align_selection_box.update_ui(state.clone());

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let vertical_align_selection_box_change = move |combo_box_text: &ComboBoxText| {
            lockscreen_provider.borrow_mut().set_input_vertical_alignment(combo_box_text.active_text().unwrap().to_string());
        };
        vertical_align_selection_box.set_selection_change(vertical_align_selection_box_change);

        // Password input horizontal align selection box
        let mut horizontal_align_selection_box = SelectionBox::new();
        horizontal_align_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SelectionBoxState {
            label_text: "Horizontal alignment".to_string(),
            selected_option: lockscreen_state.input_horizontal_alignment.clone(),
            options: vec!["start".to_string(), "center".to_string(), "end".to_string()],
        };
        horizontal_align_selection_box.update_state(state.clone());
        horizontal_align_selection_box.update_ui(state.clone());

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let horizontal_align_selection_box_change = move |combo_box_text: &ComboBoxText| {
            lockscreen_provider.borrow_mut().set_input_horizontal_alignment(combo_box_text.active_text().unwrap().to_string());
        };
        horizontal_align_selection_box.set_selection_change(horizontal_align_selection_box_change);

        password_input_field_section_box.append(input_width_spin_button.get_widget());
        password_input_field_section_box.append(input_height_spin_button.get_widget());
        password_input_field_section_box.append(input_outline_thickness_spin_button.get_widget());
        password_input_field_section_box.append(input_dots_size_spin_button.get_widget());
        password_input_field_section_box.append(input_dots_spacing_spin_button.get_widget());
        password_input_field_section_box.append(input_dots_center_selection_box.get_widget());
        password_input_field_section_box.append(outer_color_selector.get_widget());
        password_input_field_section_box.append(inner_color_selector.get_widget());
        password_input_field_section_box.append(font_color_selector.get_widget());
        password_input_field_section_box.append(input_placeholder_text_input_field.get_widget());
        password_input_field_section_box.append(hide_input_selection_box.get_widget());
        password_input_field_section_box.append(input_x_position.get_widget());
        password_input_field_section_box.append(input_y_position.get_widget());
        password_input_field_section_box.append(vertical_align_selection_box.get_widget());
        password_input_field_section_box.append(horizontal_align_selection_box.get_widget());
        password_input_field_section_box
    }

    fn create_text_display_section_box(&self, lockscreen_state: &LockScreenPageState) -> gtk::Box {
        const TEXT_DISPLAY_TITLE: &str = "Display text";
        let text_display_section_box = SectionBoxBuilder::new()
            .create_header_elements(TEXT_DISPLAY_TITLE)
            .build().expect("Failed to create text display section box");

        // Text input field
        let mut display_text_input_field = InputField::new();

        let state = InputFieldState {
            label_text: "Text".to_string(),
            entry_text: lockscreen_state.display_text.clone(),
            placeholder_text: "$time (current formatted datetime)".to_string(),
        };
        display_text_input_field.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let display_text_input_field_change = move |entry: &Entry| {
            lockscreen_provider.borrow_mut().set_display_text(entry.text().to_string());
        };
        display_text_input_field.set_input_callback(display_text_input_field_change);

        // Text color selector
        let mut text_color_picker = ColorSelector::new();
        text_color_picker.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = ColorSelectorState {
            label_text: "Text Color".to_string(),
            selected_color: Some(lockscreen_state.display_text_color.clone()),
        };
        text_color_picker.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let text_color_picker_change = move |color_button: &ColorButton| {
            lockscreen_provider.borrow_mut().set_display_text_color(RGBAColor::new(color_button.rgba()))
        };
        text_color_picker.set_color_change(text_color_picker_change);

        // Font size spin button
        let mut text_font_size_spin_button = SpinButton::new();
        text_font_size_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Font size".to_string(),
            min_value: 1.0,
            max_value: 100.0,
            current_value: lockscreen_state.display_text_font_size as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        text_font_size_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let text_font_size_spin_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_display_text_font_size(spin_button.value() as u32);
        };
        text_font_size_spin_button.set_value_change(text_font_size_spin_change);

        // Font input field
        let mut text_font = InputField::new();

        let state = InputFieldState {
            label_text: "Font".to_string(),
            entry_text: lockscreen_state.display_text_font.clone(),
            placeholder_text: "e.g. Calibri".to_string(),
        };
        text_font.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let text_font_change = move |entry: &Entry| {
            lockscreen_provider.borrow_mut().set_display_text_font(entry.text().to_string());
        } ;
        text_font.set_input_callback(text_font_change);

        // X-Position spin button
        let mut text_x_position_spin_button = SpinButton::new();
        text_x_position_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "X-Position".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.display_text_x_position as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        text_x_position_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let text_x_position_spin_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_display_text_x_position(spin_button.value() as u32);
        };
        text_x_position_spin_button.set_value_change(text_x_position_spin_change);

        // Y-Position spin button
        let mut text_y_position_spin_button = SpinButton::new();
        text_y_position_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SpinButtonState {
            label_text: "Y-Position".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: lockscreen_state.display_text_y_position as f64,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        text_y_position_spin_button.update_ui(state);

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let text_y_position_spin_change = move |spin_button: &gtk::SpinButton| {
            lockscreen_provider.borrow_mut().set_display_text_y_position(spin_button.value() as u32);
        };
        text_y_position_spin_button.set_value_change(text_y_position_spin_change);

        // Vertical alignment selection box
        let mut text_vertical_align_selection_box = SelectionBox::new();
        text_vertical_align_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SelectionBoxState {
            label_text: "Vertical alignment".to_string(),
            selected_option: lockscreen_state.display_text_vertical_alignment.clone(),
            options: vec!["top".to_string(), "center".to_string(), "bottom".to_string()],
        };
        text_vertical_align_selection_box.update_state(state.clone());
        text_vertical_align_selection_box.update_ui(state.clone());

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let text_vertical_align_selection_box_change = move |combobox: &ComboBoxText| {
            lockscreen_provider.borrow_mut().set_display_text_vertical_alignment(combobox.active_text().unwrap().to_string())
        };
        text_vertical_align_selection_box.set_selection_change(text_vertical_align_selection_box_change);

        // Horizontal alignment selection box
        let mut text_horizontal_align_selection_box = SelectionBox::new();
        text_horizontal_align_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);

        let state = SelectionBoxState {
            label_text: "Horizontal alignment".to_string(),
            selected_option: lockscreen_state.display_text_horizontal_alignment.clone(),
            options: vec!["start".to_string(), "center".to_string(), "end".to_string()],
        };
        text_horizontal_align_selection_box.update_state(state.clone());
        text_horizontal_align_selection_box.update_ui(state.clone());

        let lockscreen_provider = self.application_provider.get_lockscreen_provider();
        let text_horizontal_align_selection_box_change = move |combobox: &ComboBoxText| {
            lockscreen_provider.borrow_mut().set_display_text_horizontal_alignment(combobox.active_text().unwrap().to_string())
        };
        text_horizontal_align_selection_box.set_selection_change(text_horizontal_align_selection_box_change);

        text_display_section_box.append(display_text_input_field.get_widget());
        text_display_section_box.append(text_color_picker.get_widget());
        text_display_section_box.append(text_font_size_spin_button.get_widget());
        text_display_section_box.append(text_font.get_widget());
        text_display_section_box.append(text_x_position_spin_button.get_widget());
        text_display_section_box.append(text_y_position_spin_button.get_widget());
        text_display_section_box.append(text_vertical_align_selection_box.get_widget());
        text_display_section_box.append(text_horizontal_align_selection_box.get_widget());
        text_display_section_box
    }

    fn create_lockscreen_warning(&self) {
        let lockscreen_warning = Boxes::create_warning_box(
            "⚠️ Hyprlock program module was not found. This is required to configure the lockscreen settings."
        );
        self.lockscreen_sections_box.append(&lockscreen_warning);
    }
}