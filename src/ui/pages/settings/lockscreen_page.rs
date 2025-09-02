use gtk::{Orientation, ScrolledWindow};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::boxes::Boxes;
use crate::ui::controls::color_selector::{ColorSelector, ColorSelectorState};
use crate::ui::controls::Control;
use crate::ui::controls::input_field::{InputField, InputFieldState};
use crate::ui::controls::selection_box::{SelectionBox, SelectionBoxState};
use crate::ui::controls::spin_button::SpinButton;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::statable_control::StatableControl;
use crate::ui::states::lockscreen_page_state::LockScreenPageState;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_control::UpdatableControl;

const LOCKSCREEN_LABEL_WIDTH: u32 = 180;

pub struct LockScreenPage {
    application_provider: ApplicationProvider,
    state: LockScreenPageState,
    lockscreen_scroll_box: gtk::Box,
    lockscreen_sections_box: gtk::Box,
}

impl Control for LockScreenPage {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.lockscreen_scroll_box
    }
}

impl UpdatableControl<LockScreenPageState> for LockScreenPage {
    fn update_ui(&mut self, state: LockScreenPageState) {
        Boxes::clear_box_content(&self.lockscreen_sections_box);

        if state.enabled {
            self.create_lockscreen_sections();
        } else {
            self.create_lockscreen_warning();
        }
    }
}

impl StatableControl<LockScreenPageState> for LockScreenPage {
    fn update_state(&mut self, state: LockScreenPageState) {
        self.state = state;
    }
}

impl LockScreenPage {
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
            enabled: true
        };

        Self {
            application_provider,
            state,
            lockscreen_scroll_box,
            lockscreen_sections_box,
        }
    }

    fn create_lockscreen_sections(&self) {
        self.lockscreen_sections_box.append(&self.create_general_section());
        self.lockscreen_sections_box.append(&self.create_background_section());
        self.lockscreen_sections_box.append(&self.create_password_input_field_section());
        self.lockscreen_sections_box.append(&self.create_text_display_section_box());
    }

    fn create_general_section(&self) -> gtk::Box {
        const GENERAL_TITLE: &str = "General";
        let general_section_box = SectionBoxBuilder::new()
            .create_header_elements(GENERAL_TITLE)
            .build().expect("Failed to create general section box");

        let state = SelectionBoxState {
            label_text: "Hide cursor".to_string(),
            selected_option: None,
            options: vec!["false".to_string(), "true".to_string()],
        };
        let mut hide_cursor_selection_box = SelectionBox::new();
        hide_cursor_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        hide_cursor_selection_box.update_state(state.clone());
        hide_cursor_selection_box.update_ui(state.clone());

        let state = SpinButtonState {
            label_text: "Grace".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: true,
        };
        let mut grace_spin_button = SpinButton::new();
        grace_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        grace_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Grace".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: true,
        };
        let mut fall_timeout = SpinButton::new();
        fall_timeout.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        fall_timeout.update_ui(state);

        general_section_box.append(hide_cursor_selection_box.get_widget());
        general_section_box.append(grace_spin_button.get_widget());
        general_section_box.append(fall_timeout.get_widget());
        general_section_box
    }

    fn create_background_section(&self) -> gtk::Box {
        const BACKGROUND_TITLE: &str = "Background";
        let general_section_box = SectionBoxBuilder::new()
            .create_header_elements(BACKGROUND_TITLE)
            .build().expect("Failed to create background section box");

        let state = InputFieldState {
            label_text: "Lockscreen wallpaper path".to_string(),
            entry_text: None,
            placeholder_text: "e.g. ~/Pictures/lockscreen.png".to_string(),
        };
        let mut lockscreen_wallpaper_input_field = InputField::new();
        lockscreen_wallpaper_input_field.update_ui(state);

        let state = SpinButtonState {
            label_text: "Blur size".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut blur_size_spin_button = SpinButton::new();
        blur_size_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        blur_size_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Blur passes".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut blur_passes_spin_button = SpinButton::new();
        blur_passes_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        blur_passes_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Noise".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        let mut noise_spin_button = SpinButton::new();
        noise_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        noise_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Contrast".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        let mut contrast_spin_button = SpinButton::new();
        contrast_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        contrast_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Brightness".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        let mut brightness_spin_button = SpinButton::new();
        brightness_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        brightness_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Vibrancy".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        let mut vibrancy_spin_button = SpinButton::new();
        vibrancy_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        vibrancy_spin_button.update_ui(state);

        general_section_box.append(lockscreen_wallpaper_input_field.get_widget());
        general_section_box.append(blur_size_spin_button.get_widget());
        general_section_box.append(blur_passes_spin_button.get_widget());
        general_section_box.append(noise_spin_button.get_widget());
        general_section_box.append(contrast_spin_button.get_widget());
        general_section_box.append(brightness_spin_button.get_widget());
        general_section_box.append(vibrancy_spin_button.get_widget());
        general_section_box
    }

    fn create_password_input_field_section(&self) -> gtk::Box {
        const PASSWORD_INPUT_FIELD_TITLE: &str = "Password field";
        let password_input_field_section_box = SectionBoxBuilder::new()
            .create_header_elements(PASSWORD_INPUT_FIELD_TITLE)
            .build().expect("Failed to create text display section box");

        let state = SpinButtonState {
            label_text: "Input width".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut input_width_spin_button = SpinButton::new();
        input_width_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        input_width_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Input height".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut input_height_spin_button = SpinButton::new();
        input_height_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        input_height_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Outline thickness".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut input_outline_thickness_spin_button = SpinButton::new();
        input_outline_thickness_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        input_outline_thickness_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Dots size".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut input_dots_size_spin_button = SpinButton::new();
        input_dots_size_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        input_dots_size_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Dots spacing".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut input_dots_spacing_spin_button = SpinButton::new();
        input_dots_spacing_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        input_dots_spacing_spin_button.update_ui(state);

        let state = SelectionBoxState {
            label_text: "Dots center".to_string(),
            selected_option: None,
            options: vec!["false".to_string(), "true".to_string()],
        };
        let mut input_dots_center_selection_box = SelectionBox::new();
        input_dots_center_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        input_dots_center_selection_box.update_state(state.clone());
        input_dots_center_selection_box.update_ui(state.clone());

        let state = ColorSelectorState {
            label_text: "Outer color".to_string(),
            selected_color: None,
        };
        let mut outer_color_selector = ColorSelector::new();
        outer_color_selector.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        outer_color_selector.update_ui(state);

        let state = ColorSelectorState {
            label_text: "Inner color".to_string(),
            selected_color: None,
        };
        let mut inner_color_selector = ColorSelector::new();
        inner_color_selector.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        inner_color_selector.update_ui(state);

        let state = ColorSelectorState {
            label_text: "Font color".to_string(),
            selected_color: None,
        };
        let mut font_color_selector = ColorSelector::new();
        font_color_selector.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        font_color_selector.update_ui(state);

        let state = InputFieldState {
            label_text: "Placeholder text".to_string(),
            entry_text: None,
            placeholder_text: "e.g. insert password here...".to_string(),
        };
        let mut input_placeholder_text_input_field = InputField::new();
        input_placeholder_text_input_field.update_ui(state);

        let state = SelectionBoxState {
            label_text: "Hide input".to_string(),
            selected_option: None,
            options: vec!["false".to_string(), "true".to_string()],
        };
        let mut hide_input_selection_box = SelectionBox::new();
        hide_input_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        hide_input_selection_box.update_state(state.clone());
        hide_input_selection_box.update_ui(state.clone());

        let state = SpinButtonState {
            label_text: "X-Position".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 3.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut input_x_position = SpinButton::new();
        input_x_position.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        input_x_position.update_ui(state);

        let state = SpinButtonState {
            label_text: "Y-Position".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 3.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut input_y_position = SpinButton::new();
        input_y_position.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        input_y_position.update_ui(state);

        let state = SelectionBoxState {
            label_text: "Vertical alignment".to_string(),
            selected_option: None,
            options: vec!["top".to_string(), "center".to_string(), "bottom".to_string()],
        };
        let mut vertical_align_selection_box = SelectionBox::new();
        vertical_align_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        vertical_align_selection_box.update_state(state.clone());
        vertical_align_selection_box.update_ui(state.clone());

        let state = SelectionBoxState {
            label_text: "Horizontal alignment".to_string(),
            selected_option: None,
            options: vec!["start".to_string(), "center".to_string(), "end".to_string()],
        };
        let mut horizontal_align_selection_box = SelectionBox::new();
        horizontal_align_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        horizontal_align_selection_box.update_state(state.clone());
        horizontal_align_selection_box.update_ui(state.clone());

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

    fn create_text_display_section_box(&self) -> gtk::Box {
        const TEXT_DISPLAY_TITLE: &str = "Display text";
        let text_display_section_box = SectionBoxBuilder::new()
            .create_header_elements(TEXT_DISPLAY_TITLE)
            .build().expect("Failed to create text display section box");

        let state = InputFieldState {
            label_text: "Text".to_string(),
            entry_text: None,
            placeholder_text: "$time (current formatted datetime)".to_string(),
        };
        let mut display_text_input_field = InputField::new();
        display_text_input_field.update_ui(state);

        let state = ColorSelectorState {
            label_text: "Text Color".to_string(),
            selected_color: None,
        };
        let mut display_text_color_picker = ColorSelector::new();
        display_text_color_picker.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        display_text_color_picker.update_ui(state);

        let state = SpinButtonState {
            label_text: "Font size".to_string(),
            min_value: 1.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut display_text_font_size_spin_button = SpinButton::new();
        display_text_font_size_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        display_text_font_size_spin_button.update_ui(state);

        let state = InputFieldState {
            label_text: "Font".to_string(),
            entry_text: None,
            placeholder_text: "e.g. Calibri".to_string(),
        };
        let mut display_text_font = InputField::new();
        display_text_font.update_ui(state);

        let state = SpinButtonState {
            label_text: "X-Position".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut display_text_x_position_spin_button = SpinButton::new();
        display_text_x_position_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        display_text_x_position_spin_button.update_ui(state);

        let state = SpinButtonState {
            label_text: "Y-Position".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
            increment_value: 1.0,
            page_increment_value: 5.0,
            page_size: 0.0,
            climb_rate: 2.0,
            digit_count: 0,
            use_integral_numbers: false,
        };
        let mut display_text_y_position_spin_button = SpinButton::new();
        display_text_y_position_spin_button.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        display_text_y_position_spin_button.update_ui(state);

        let state = SelectionBoxState {
            label_text: "Vertical alignment".to_string(),
            selected_option: None,
            options: vec!["top".to_string(), "center".to_string(), "bottom".to_string()],
        };
        let mut display_text_vertical_align_selection_box = SelectionBox::new();
        display_text_vertical_align_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        display_text_vertical_align_selection_box.update_state(state.clone());
        display_text_vertical_align_selection_box.update_ui(state.clone());

        let state = SelectionBoxState {
            label_text: "Horizontal alignment".to_string(),
            selected_option: None,
            options: vec!["start".to_string(), "center".to_string(), "end".to_string()],
        };
        let mut display_text_horizontal_align_selection_box = SelectionBox::new();
        display_text_horizontal_align_selection_box.set_text_width(LOCKSCREEN_LABEL_WIDTH);
        display_text_horizontal_align_selection_box.update_state(state.clone());
        display_text_horizontal_align_selection_box.update_ui(state.clone());

        text_display_section_box.append(display_text_input_field.get_widget());
        text_display_section_box.append(display_text_color_picker.get_widget());
        text_display_section_box.append(display_text_font_size_spin_button.get_widget());
        text_display_section_box.append(display_text_font.get_widget());
        text_display_section_box.append(display_text_x_position_spin_button.get_widget());
        text_display_section_box.append(display_text_y_position_spin_button.get_widget());
        text_display_section_box.append(display_text_vertical_align_selection_box.get_widget());
        text_display_section_box.append(display_text_horizontal_align_selection_box.get_widget());
        text_display_section_box
    }

    fn create_lockscreen_warning(&self) {
        let lockscreen_warning = Boxes::create_warning_box(
            "⚠️ Hyprlock program module was not found. This is required to configure the lockscreen settings."
        );
        self.lockscreen_sections_box.append(&lockscreen_warning);
    }
}