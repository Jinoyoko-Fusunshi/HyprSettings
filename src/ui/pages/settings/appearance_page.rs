use gtk::{ColorButton, ComboBoxText, Entry, Orientation, PolicyType, ScrolledWindow, SpinButton as GTKSpinButton};
use gtk::prelude::{BoxExt, ColorChooserExt, EditableExt, WidgetExt};
use crate::models::rgba_color::RGBAColor;
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::controls::color_selector::ColorSelector;
use crate::ui::controls::input_field::InputField;
use crate::ui::controls::selection_box::SelectionBox;
use crate::ui::controls::Control;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::controls::color_selector::ColorSelectorState;
use crate::ui::controls::input_field::InputFieldState;
use crate::ui::controls::selection_box::SelectionBoxState;
use crate::ui::controls::spin_button::SpinButton;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::statable_control::StatableControl;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_control::UpdatableControl;

const APPEARANCE_LABEL_WIDTH: u32 = 180;

pub struct AppearanceSettings {
    widget: gtk::Box,
}

impl Control for AppearanceSettings {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.widget
    }
}

impl AppearanceSettings {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let appearance_scroll_box = gtk::Box::new(Orientation::Vertical, 10);
        appearance_scroll_box.set_vexpand(true);

        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_policy(PolicyType::Never, PolicyType::Automatic);
        scrolled_window.set_vexpand(true);

        let appearance_box = gtk::Box::new(Orientation::Vertical, 10);
        appearance_box.set_margin_top(10);
        appearance_box.set_margin_bottom(10);
        appearance_box.set_margin_start(10);
        appearance_box.set_margin_end(10);

        let wallpaper_section = AppearanceSettings::create_wallpaper_section_box(&application_provider);
        let styling_section = AppearanceSettings::create_styling_section(&application_provider);
        let decoration_section = AppearanceSettings::create_decorations_section(&application_provider);
        let animations_section = AppearanceSettings::create_animations_section(&application_provider);
        let layouts_section = AppearanceSettings::create_layouts_section(&application_provider);

        appearance_box.append(&wallpaper_section);
        appearance_box.append(&styling_section);
        appearance_box.append(&decoration_section);
        appearance_box.append(&animations_section);
        appearance_box.append(&layouts_section);

        scrolled_window.set_child(Some(&appearance_box));
        appearance_scroll_box.append(&scrolled_window);

        Self {
            widget: appearance_scroll_box
        }
    }

    fn create_wallpaper_section_box(application_provider: &ApplicationProvider) -> gtk::Box {
        const WALLPAPER_LABEL: &str = "Wallpaper";
        let wallpaper_section_box = SectionBoxBuilder::new()
            .create_header_elements(WALLPAPER_LABEL)
            .build().expect("Cannot create wallpaper section");

        let settings_provider = application_provider.get_settings_provider();

        // Force default wallpaper option
        let settings_provider_clone = settings_provider.clone();
        let force_default_wallpaper_selection_change = {
            move |entry: &ComboBoxText| {
                let selected_text = entry.active_text().expect("Cannot read active selection text");
                let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
                settings_provider_clone.borrow_mut().set_force_default_wallpaper(bool_value);
            }
        };
        let mut force_default_wallpaper_selection = SelectionBox::new();
        force_default_wallpaper_selection.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Force default wallpaper:".to_string(),
            selected_option: Some(settings_provider.borrow().get_force_default_wallpaper().to_string()),
            options: vec!["false".to_string(), "true".to_string()],
        };
        force_default_wallpaper_selection.update_state(state.clone());
        force_default_wallpaper_selection.update_ui(state.clone());
        force_default_wallpaper_selection.set_selection_change(force_default_wallpaper_selection_change);

        // Disable hyprland logo option
        let settings_provider_clone = settings_provider.clone();
        let disable_hyprland_logo_selection_change = {
            move |entry: &ComboBoxText| {
                let selected_text = entry.active_text().expect("Cannot read active selection text");
                let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
                settings_provider_clone.borrow_mut().disable_hyprland_logo(bool_value);
            }
        };
        let mut disable_hyprland_logo_selection = SelectionBox::new();
        disable_hyprland_logo_selection.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Disable hyprland logo:".to_string(),
            selected_option: Some(settings_provider.borrow().get_disable_hyprland_logo().to_string()),
            options: vec!["false".to_string(), "true".to_string()],
        };
        disable_hyprland_logo_selection.update_state(state.clone());
        disable_hyprland_logo_selection.update_ui(state.clone());
        disable_hyprland_logo_selection.set_selection_change(disable_hyprland_logo_selection_change);
        
        wallpaper_section_box.append(force_default_wallpaper_selection.get_widget());
        wallpaper_section_box.append(disable_hyprland_logo_selection.get_widget());
        wallpaper_section_box
    }

    fn create_styling_section(application_provider: &ApplicationProvider) -> gtk::Box {
        const STYLING_LABEL: &str = "Styling";
        let styling_section_box = SectionBoxBuilder::new()
            .create_header_elements(STYLING_LABEL)
            .build().expect("Cannot create styling section");

        // Inner gap option
        let settings_provider = application_provider.get_settings_provider();
        let settings_provider_clone = settings_provider.clone();
        let inner_gap_spin_button_change = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_inner_gab(spin_button.value());
        };
        let mut inner_gap_spin_button = SpinButton::new();
        inner_gap_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text:  "Inner gab:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_inner_gab(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        inner_gap_spin_button.update_ui(state);
        inner_gap_spin_button.set_value_change(inner_gap_spin_button_change);

        // Outer gap option
        let settings_provider_clone = settings_provider.clone();
        let outer_gap_spin_button_change = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_outer_gab(spin_button.value());
        };
        let mut outer_gap_spin_button = SpinButton::new();
        outer_gap_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Outer gab:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_outer_gab(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        outer_gap_spin_button.update_ui(state);
        outer_gap_spin_button.set_value_change(outer_gap_spin_button_change);

        // Border size option
        let settings_provider_clone = settings_provider.clone();
        let border_size_spin_button_change = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_border_size(spin_button.value());
        };
        let mut border_size_spin_button = SpinButton::new();
        border_size_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Border size:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_border_size(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        border_size_spin_button.update_ui(state);
        border_size_spin_button.set_value_change(border_size_spin_button_change);

        // Active border option
        let settings_provider_clone = settings_provider.clone();
        let active_border_color_change = move |color_button: &ColorButton| {
            settings_provider_clone.borrow_mut().set_active_border_color(RGBAColor::new(color_button.rgba()));
        };
        let mut active_border_color_selector = ColorSelector::new();
        active_border_color_selector.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = ColorSelectorState {
            label_text: "Active Border:".to_string(),
            selected_color: Some(settings_provider.borrow().get_active_border_color()),
        };
        active_border_color_selector.update_ui(state);
        active_border_color_selector.set_color_change(active_border_color_change);

        // Inactive border option
        let settings_provider_clone = settings_provider.clone();
        let inactive_border_color_change = move |color_button: &ColorButton| {
            settings_provider_clone.borrow_mut().set_inactive_border_color(RGBAColor::new(color_button.rgba()));
        };
        let mut inactive_border_color_selector = ColorSelector::new();
        inactive_border_color_selector.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = ColorSelectorState {
            label_text: "Inactive Border:".to_string(),
            selected_color: Some(settings_provider.borrow().get_inactive_border_color()),
        };
        inactive_border_color_selector.update_ui(state);
        inactive_border_color_selector.set_color_change(inactive_border_color_change);

        // Resize on border option
        let settings_provider_clone = settings_provider.clone();
        let border_resize_selection_change = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_provider_clone.borrow_mut().set_resize_on_border(bool_value);
        };
        let mut border_resize_selection_box = SelectionBox::new();
        border_resize_selection_box.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Resize on border:".to_string(),
            selected_option: Some(settings_provider.borrow().get_resize_on_border().to_string()),
            options: vec!["false".to_string(), "true".to_string()],
        };
        border_resize_selection_box.update_state(state.clone());
        border_resize_selection_box.update_ui(state.clone());
        border_resize_selection_box.set_selection_change(border_resize_selection_change);

        // Allow tearing option
        let settings_provider_clone = settings_provider.clone();
        let tearing_selection_change = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_provider_clone.borrow_mut().set_allow_tearing(bool_value);
        };
        let mut allow_tearing_selection_box = SelectionBox::new();
        allow_tearing_selection_box.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Allow tearing:".to_string(),
            selected_option: Some(settings_provider.borrow().get_allow_tearing().to_string()),
            options: vec!["false".to_string(), "true".to_string()],
        };
        allow_tearing_selection_box.update_state(state.clone());
        allow_tearing_selection_box.update_ui(state.clone());
        allow_tearing_selection_box.set_selection_change(tearing_selection_change);

        styling_section_box.append(inner_gap_spin_button.get_widget());
        styling_section_box.append(outer_gap_spin_button.get_widget());
        styling_section_box.append(border_size_spin_button.get_widget());
        styling_section_box.append(active_border_color_selector.get_widget());
        styling_section_box.append(inactive_border_color_selector.get_widget());
        styling_section_box.append(border_resize_selection_box.get_widget());
        styling_section_box.append(allow_tearing_selection_box.get_widget());
        styling_section_box
    }

    fn create_decorations_section(application_provider: &ApplicationProvider) -> gtk::Box {
        const DECORATION_LABEL: &str = "Decoration";
        let decorations_section_box = SectionBoxBuilder::new()
            .create_header_elements(DECORATION_LABEL)
            .build().expect("Cannot create styling section");

        let settings_provider = application_provider.get_settings_provider();

        // Rounding option
        let settings_provider_clone = settings_provider.clone();
        let rounding_spin_button_changed_callback = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_rounding(spin_button.value());
        };
        let mut rounding_spin_button = SpinButton::new();
        rounding_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Rounding:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_rounding(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        rounding_spin_button.update_ui(state);
        rounding_spin_button.set_value_change(rounding_spin_button_changed_callback);

        // Rounding power option
        let settings_provider_clone = settings_provider.clone();
        let rounding_spin_power_button_changed_callback = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_rounding_power(spin_button.value());
        };
        let mut rounding_spin_power_button = SpinButton::new();
        rounding_spin_power_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Rounding power:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_rounding_power(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        rounding_spin_power_button.update_ui(state);
        rounding_spin_power_button.set_value_change(rounding_spin_power_button_changed_callback);

        // Dim inactive option
        let settings_provider_clone = settings_provider.clone();
        let dim_inactive_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_provider_clone.borrow_mut().set_dim_inactive(bool_value);
        };
        let mut dim_inactive_selection_box = SelectionBox::new();
        dim_inactive_selection_box.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Dim inactive:".to_string(),
            options: vec!["false".to_string(), "true".to_string()],
            selected_option: Some(settings_provider.borrow().get_dim_inactive().to_string()),
        };
        dim_inactive_selection_box.update_state(state.clone());
        dim_inactive_selection_box.update_ui(state.clone());
        dim_inactive_selection_box.set_selection_change(dim_inactive_selection_changed_callback);

        // Active opacity option
        let settings_provider_clone = settings_provider.clone();
        let active_opacity_spin_button_changed_callback = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_active_opacity(spin_button.value());
        };
        let mut active_opacity_spin_button = SpinButton::new();
        active_opacity_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Active opacity:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_active_opacity(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        active_opacity_spin_button.update_ui(state);
        active_opacity_spin_button.set_value_change(active_opacity_spin_button_changed_callback);

        // Inactive opacity option
        let settings_provider_clone = settings_provider.clone();
        let inactive_opacity_spin_button_changed_callback = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_inactive_opacity(spin_button.value());
        };
        let mut inactive_opacity_spin_button = SpinButton::new();
        inactive_opacity_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Inactive opacity:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_inactive_opacity(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        inactive_opacity_spin_button.update_ui(state);
        inactive_opacity_spin_button.set_value_change(inactive_opacity_spin_button_changed_callback);

        // Active shadows option
        let settings_provider_clone = settings_provider.clone();
        let active_shadows_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_provider_clone.borrow_mut().set_active_shadow(bool_value);
        };
        let mut active_shadows_selection_box = SelectionBox::new();
        active_shadows_selection_box.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Active shadows:".to_string(),
            options: vec!["false".to_string(), "true".to_string()],
            selected_option: Some(settings_provider.borrow().get_active_shadow().to_string()),
        };
        active_shadows_selection_box.update_state(state.clone());
        active_shadows_selection_box.update_ui(state.clone());
        active_shadows_selection_box.set_selection_change(active_shadows_selection_changed_callback);

        // Shadow range option
        let settings_provider_clone = settings_provider.clone();
        let shadow_range_spin_button_changed_callback = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_shadow_range(spin_button.value());
        };
        let mut shadow_range_spin_button = SpinButton::new();
        shadow_range_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Shadow range:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_shadow_range(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        shadow_range_spin_button.update_ui(state);
        shadow_range_spin_button.set_value_change(shadow_range_spin_button_changed_callback);

        // Shadow render power option
        let settings_provider_clone = settings_provider.clone();
        let shadow_render_power_spin_button_changed_callback = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_shadow_render_power(spin_button.value());
        };
        let mut shadow_render_power_spin_button = SpinButton::new();
        shadow_render_power_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Shadow render power:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_shadow_render_power(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        shadow_render_power_spin_button.update_ui(state);
        shadow_render_power_spin_button.set_value_change(shadow_render_power_spin_button_changed_callback);

        // Shadow color option
        let settings_provider_clone = settings_provider.clone();
        let shadow_color_button_changed_callback = move |color_button: &ColorButton| {
            settings_provider_clone.borrow_mut().set_shadow_color(RGBAColor::new(color_button.rgba()));
        };
        let mut shadow_color_selector = ColorSelector::new();
        shadow_color_selector.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = ColorSelectorState {
            label_text: "Shadow color:".to_string(),
            selected_color: Some(settings_provider.borrow().get_shadow_color()),
        };
        shadow_color_selector.update_ui(state);
        shadow_color_selector.set_color_change(shadow_color_button_changed_callback);

        // Active blur option
        let settings_provider_clone = settings_provider.clone();
        let active_blur_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_provider_clone.borrow_mut().set_active_blur(bool_value);
        };
        let mut active_blur_selection_box = SelectionBox::new();
        active_blur_selection_box.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Active blur:".to_string(),
            options: vec!["false".to_string(), "true".to_string()],
            selected_option: Some(settings_provider.borrow().get_active_blur().to_string()),
        };
        active_blur_selection_box.update_state(state.clone());
        active_blur_selection_box.update_ui(state.clone());
        active_blur_selection_box.set_selection_change(active_blur_selection_changed_callback);

        // Blur size option
        let settings_provider_clone = settings_provider.clone();
        let blur_size_spin_button_changed_callback = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_blur_size(spin_button.value());
        };
        let mut blur_size_spin_button = SpinButton::new();
        blur_size_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Blur size:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_blur_size(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        blur_size_spin_button.update_ui(state);
        blur_size_spin_button.set_value_change(blur_size_spin_button_changed_callback);

        // Blur passes option
        let settings_provider_clone = settings_provider.clone();
        let blur_passes_spin_button_changed_callback = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_blur_passes(spin_button.value() as usize);
        };
        let mut blur_passes_spin_button = SpinButton::new();
        blur_passes_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Blur passes:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_blur_passes() as f64,
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        blur_passes_spin_button.update_ui(state);
        blur_passes_spin_button.set_value_change(blur_passes_spin_button_changed_callback);

        // Blur vibrancy option
        let settings_provider_clone = settings_provider.clone();
        let blur_vibrancy_spin_button_change = move |spin_button: &GTKSpinButton| {
            settings_provider_clone.borrow_mut().set_blur_vibrancy(spin_button.value());
        };
        let mut blur_vibrancy_spin_button = SpinButton::new();
        blur_vibrancy_spin_button.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SpinButtonState {
            label_text: "Blur vibrancy:".to_string(),
            min_value: 0.0,
            max_value: 100.0,
            current_value: settings_provider.borrow().get_blur_vibrancy(),
            increment_value: 0.1,
            page_increment_value: 1.0,
            page_size: 0.0,
            climb_rate: 1.0,
            digit_count: 1,
            use_integral_numbers: false,
        };
        blur_vibrancy_spin_button.update_ui(state);
        blur_vibrancy_spin_button.set_value_change(blur_vibrancy_spin_button_change);

        decorations_section_box.append(rounding_spin_button.get_widget());
        decorations_section_box.append(rounding_spin_power_button.get_widget());
        decorations_section_box.append(dim_inactive_selection_box.get_widget());
        decorations_section_box.append(active_opacity_spin_button.get_widget());
        decorations_section_box.append(inactive_opacity_spin_button.get_widget());
        decorations_section_box.append(active_shadows_selection_box.get_widget());
        decorations_section_box.append(shadow_range_spin_button.get_widget());
        decorations_section_box.append(shadow_render_power_spin_button.get_widget());
        decorations_section_box.append(shadow_color_selector.get_widget());
        decorations_section_box.append(active_blur_selection_box.get_widget());
        decorations_section_box.append(blur_size_spin_button.get_widget());
        decorations_section_box.append(blur_passes_spin_button.get_widget());
        decorations_section_box.append(blur_vibrancy_spin_button.get_widget());
        decorations_section_box
    }

    fn create_animations_section(_: &ApplicationProvider) -> gtk::Box {
        const ANIMATIONS_LABEL: &str = "Animations";
        let animations_section_box = SectionBoxBuilder::new()
            .create_header_elements(ANIMATIONS_LABEL)
            .build().expect("Cannot create animations section");

        animations_section_box
    }

    fn create_layouts_section(application_provider: &ApplicationProvider) -> gtk::Box {
        const LAYOUT_LABEL: &str = "Layout";
        let layout_section_box = SectionBoxBuilder::new()
            .create_header_elements(LAYOUT_LABEL)
            .build().expect("Cannot create layout section");

        // Layout option
        let settings_provider = application_provider.get_settings_provider();
        let settings_provider_clone = settings_provider.clone();
        let layout_input_change = move |entry: &Entry| {
            settings_provider_clone.borrow_mut().set_layout(entry.text().to_string());
        };
        let mut layout_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Layout:".to_string(),
            placeholder_text: "e.g. dwindle, ".to_string(),
            entry_text: Some(settings_provider.borrow().get_layout()),
        };
        layout_input_field.update_ui(state);
        layout_input_field.set_input_callback(layout_input_change);

        // Pseudo tiling option
        let settings_provider_clone = settings_provider.clone();
        let pseudo_tiling_selection_change = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_provider_clone.borrow_mut().set_pseudo_tiling(bool_value);
        };
        let mut pseudo_tiling_selection_box = SelectionBox::new();
        pseudo_tiling_selection_box.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Pseudo tiling:".to_string(),
            options: vec!["false".to_string(), "true".to_string()],
            selected_option: Some(settings_provider.borrow().get_pseudo_tiling().to_string()),
        };
        pseudo_tiling_selection_box.update_state(state.clone());
        pseudo_tiling_selection_box.update_ui(state.clone());
        pseudo_tiling_selection_box.set_selection_change(pseudo_tiling_selection_change);

        // Split preservation option
        let settings_provider_clone = settings_provider.clone();
        let split_preservation_selection_change = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_provider_clone.borrow_mut().set_split_preservation(bool_value);
        };
        let mut split_preservation_selection_box = SelectionBox::new();
        split_preservation_selection_box.set_text_width(APPEARANCE_LABEL_WIDTH);
        let state = SelectionBoxState {
            label_text: "Split preservation:".to_string(),
            options: vec!["false".to_string(), "true".to_string()],
            selected_option: Some(settings_provider.borrow().get_split_preservation().to_string()),
        };
        split_preservation_selection_box.update_state(state.clone());
        split_preservation_selection_box.update_ui(state.clone());
        split_preservation_selection_box.set_selection_change(split_preservation_selection_change);

        // Master status option
        let settings_provider_clone = settings_provider.clone();
        let master_status_input_change = move |entry: &Entry| {
            settings_provider_clone.borrow_mut().set_master_status(entry.text().to_string());
        };
        let mut master_status_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Master status:".to_string(),
            placeholder_text: "e.g. master, ".to_string(),
            entry_text: Some(settings_provider.borrow().get_master_status()),
        };
        master_status_input_field.update_ui(state);
        master_status_input_field.set_input_callback(master_status_input_change);

        layout_section_box.append(layout_input_field.get_widget());
        layout_section_box.append(master_status_input_field.get_widget());
        layout_section_box.append(pseudo_tiling_selection_box.get_widget());
        layout_section_box.append(split_preservation_selection_box.get_widget());
        layout_section_box
    }
}