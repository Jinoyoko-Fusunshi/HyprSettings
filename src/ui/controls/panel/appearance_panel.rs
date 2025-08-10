use std::cell::{RefCell};
use std::rc::Rc;
use gtk::{ColorButton, ComboBoxText, Entry, Label, Orientation, PolicyType, ScrolledWindow, Separator, SpinButton};
use gtk::prelude::{BoxExt, ColorChooserExt, EditableExt, WidgetExt};
use crate::ui::controls::named_section::{
    named_color_button_section::NamedColorButtonSection,
    named_input_section::NamedInputSection,named_selection_box::NamedSelectionBox,
    named_spin_button_section::NamedSpinButtonSection
};
use crate::ui::controls::panel::Panel;
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::settings::rgba_color::RGBAColor;

pub struct AppearancePanel {
    widget: gtk::Box,
}

impl Panel for AppearancePanel {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.widget
    }
}

impl Clone for AppearancePanel {
    fn clone(&self) -> Self {
        Self {
            widget: self.widget.clone()
        }
    }   
}

impl AppearancePanel {
    const APPEARANCE_LABEL_WIDTH: i32 = 180;

    pub fn new(settings: &Rc<RefCell<HyprlandSettings>>) -> Self {
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

        let settings_clone = settings.clone();
        let wallpaper_section = AppearancePanel::create_wallpaper_section(&settings_clone);
        let styling_section = AppearancePanel::create_styling_section(&settings_clone);
        let decoration_section = AppearancePanel::create_decorations_section(&settings_clone);
        let layouts_section = AppearancePanel::create_layouts_section(&settings_clone);

        appearance_box.append(&wallpaper_section);
        appearance_box.append(&styling_section);
        appearance_box.append(&decoration_section);
        appearance_box.append(&layouts_section);

        scrolled_window.set_child(Some(&appearance_box));
        appearance_scroll_box.append(&scrolled_window);

        Self {
            widget: appearance_scroll_box
        }
    }

    fn create_wallpaper_section(settings: &Rc<RefCell<HyprlandSettings>>) -> gtk::Box {
        const WALLPAPER_LABEL: &str = "Wallpaper";

        let wallpaper_box = gtk::Box::new(Orientation::Vertical, 10);
        let wallpaper_label = Label::new(Some(WALLPAPER_LABEL));
        let separator = Separator::new(Orientation::Horizontal);

        // Wallpaper image path option
        let wallpaper_image_input_callback = {
            let settings_clone = settings.clone();
            move |entry: &Entry| {
                settings_clone.borrow_mut().appearance_settings.wallpaper_path = entry.text().to_string();
            }
        };
        let wallpaper_image_input_section = NamedInputSection::new(
            "wallpaper path:",
            "e.g. ~/Pictures/wallpaper.png",
            Some(settings.clone().borrow().appearance_settings.wallpaper_path.clone())
        );
        wallpaper_image_input_section.set_input_callback(wallpaper_image_input_callback);

        // Force default wallpaper option
        let force_default_wallpaper_selection_changed_callback = {
            let settings_clone = settings.clone();
            move |entry: &ComboBoxText| {
                let selected_text = entry.active_text().expect("Cannot read active selection text");
                let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
                settings_clone.borrow_mut().appearance_settings.force_default_wallpaper = bool_value;
            }
        };
        let force_default_wallpaper_options = vec!["false", "true"];
        let mut force_default_wallpaper_selection = NamedSelectionBox::new(
            "Force default wallpaper:", force_default_wallpaper_options,
            Some(settings.clone().borrow().appearance_settings.force_default_wallpaper.to_string().as_str()),
            Some(force_default_wallpaper_selection_changed_callback)
        );
        force_default_wallpaper_selection.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Disable hyprland logo option
        let disable_hyprland_logo_selection_changed_callback = {
            let settings_clone = settings.clone();
            move |entry: &ComboBoxText| {
                let selected_text = entry.active_text().expect("Cannot read active selection text");
                let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
                settings_clone.borrow_mut().appearance_settings.disable_hyprland_logo = bool_value;
            }
        };
        let disable_hyprland_logo_options = vec!["false", "true"];
        let mut disable_hyprland_logo_selection = NamedSelectionBox::new(
            "Disable hyprland logo:", disable_hyprland_logo_options,
            Some(settings.clone().borrow().appearance_settings.disable_hyprland_logo.to_string().as_str()),
            Some(disable_hyprland_logo_selection_changed_callback)
        );
        disable_hyprland_logo_selection.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        wallpaper_box.append(&wallpaper_label);
        wallpaper_box.append(&separator);
        wallpaper_box.append(wallpaper_image_input_section.get_container_box());
        wallpaper_box.append(force_default_wallpaper_selection.get_container_box());
        wallpaper_box.append(disable_hyprland_logo_selection.get_container_box());
        wallpaper_box
    }

    fn create_styling_section(settings: &Rc<RefCell<HyprlandSettings>>) -> gtk::Box {
        const STYLING_LABEL: &str = "Styling";

        let general_box = gtk::Box::new(Orientation::Vertical, 10);
        let styling_label = Label::new(Some(STYLING_LABEL));
        let separator = Separator::new(Orientation::Horizontal);

        // Inner gap option
        let settings_clone = settings.clone();
        let inner_gap_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.inner_gab = spin_button.value();
        };
        let mut inner_gap_spin_button_section = NamedSpinButtonSection::new(
            "Inner gab:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.inner_gab,
            0.1, 1.0,
            0.0, 1.0, 1, false
        );
        inner_gap_spin_button_section.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        inner_gap_spin_button_section.set_change_callback(inner_gap_spin_button_changed_callback);

        // Outer gap option
        let settings_clone = settings.clone();
        let outer_gap_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.outer_gab = spin_button.value();
        };
        let mut outer_gap_spin_button_section = NamedSpinButtonSection::new(
            "Outer gab:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.outer_gab,
            0.1, 1.0,
            0.0, 1.0, 1, false
        );
        outer_gap_spin_button_section.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        outer_gap_spin_button_section.set_change_callback(outer_gap_spin_button_changed_callback);

        // Border size option
        let settings_clone = settings.clone();
        let border_size_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.border_size = spin_button.value();
        };
        let mut border_size_spin_button_section = NamedSpinButtonSection::new(
            "Border size:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.border_size,
            0.1, 1.0,
            0.0, 1.0, 1, false
        );
        border_size_spin_button_section.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        border_size_spin_button_section.set_change_callback(border_size_spin_button_changed_callback);

        // Active border option
        let settings_clone = settings.clone();
        let active_border_color_changed_callback = move |color_button: &ColorButton| {
            settings_clone.borrow_mut().appearance_settings.active_border_color = RGBAColor::new(color_button.rgba());
        };
        let mut active_border_color_section = NamedColorButtonSection::new(
            "Active Border:", 
            Some(settings.clone().borrow().appearance_settings.active_border_color.clone()),
            active_border_color_changed_callback
        );
        active_border_color_section.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Inactive border option
        let settings_clone = settings.clone();
        let inactive_border_color_changed_callback = move |color_button: &ColorButton| {
            settings_clone.borrow_mut().appearance_settings.inactive_border_color = RGBAColor::new(color_button.rgba());
        };
        let mut inactive_border_color_section = NamedColorButtonSection::new(
            "Inactive Border:", 
            Some(settings.clone().borrow().appearance_settings.inactive_border_color.clone()),
            inactive_border_color_changed_callback
        );
        inactive_border_color_section.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Resize on border option
        let settings_clone = settings.clone();
        let border_resize_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_clone.borrow_mut().appearance_settings.resize_on_border = bool_value;
        };
        let resize_options = vec!["false", "true"];
        let mut border_resize_selection_box = NamedSelectionBox::new(
            "Resize on border:", resize_options,
            Some(settings.clone().borrow().appearance_settings.resize_on_border.to_string().as_str()),
            Some(border_resize_selection_changed_callback)
        );
        border_resize_selection_box.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Allow tearing option
        let settings_clone = settings.clone();
        let tearing_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_clone.borrow_mut().appearance_settings.allow_tearing = bool_value;
        };
        let tearing_options = vec!["false", "true"];
        let mut allow_tearing_selection_box = NamedSelectionBox::new(
            "Allow tearing:", tearing_options,
            Some(settings.clone().borrow().appearance_settings.allow_tearing.to_string().as_str()),
            Some(tearing_selection_changed_callback)
        );
        allow_tearing_selection_box.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        general_box.append(&styling_label);
        general_box.append(&separator);
        general_box.append(inner_gap_spin_button_section.get_container_box());
        general_box.append(outer_gap_spin_button_section.get_container_box());
        general_box.append(border_size_spin_button_section.get_container_box());
        general_box.append(active_border_color_section.get_container_box());
        general_box.append(inactive_border_color_section.get_container_box());
        general_box.append(border_resize_selection_box.get_container_box());
        general_box.append(allow_tearing_selection_box.get_container_box());
        general_box
    }

    fn create_decorations_section(settings: &Rc<RefCell<HyprlandSettings>>) -> gtk::Box {
        const DECORATION_LABEL: &str = "Decoration";

        let decorations_box = gtk::Box::new(Orientation::Vertical, 10);
        let decoration_label = Label::new(Some(DECORATION_LABEL));
        let separator = Separator::new(Orientation::Horizontal);

        // Rounding option
        let settings_clone = settings.clone();
        let rounding_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.rounding = spin_button.value();
        };
        let mut rounding_spin_button = NamedSpinButtonSection::new(
            "Rounding:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.rounding,
            0.1, 1.0,
            0.0, 1.0, 1, false
        );
        rounding_spin_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        rounding_spin_button.set_change_callback(rounding_spin_button_changed_callback);

        // Rounding power option
        let settings_clone = settings.clone();
        let rounding_spin_power_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.rounding_power = spin_button.value();
        };
        let mut rounding_spin_power_button = NamedSpinButtonSection::new(
            "Rounding power:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.rounding_power,
            0.1, 1.0,
            0.0, 1.0, 1, false
        );
        rounding_spin_power_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        rounding_spin_power_button.set_change_callback(rounding_spin_power_button_changed_callback);

        // Dim inactive option
        let settings_clone = settings.clone();
        let dim_inactive_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_clone.borrow_mut().appearance_settings.dim_inactive = bool_value;
        };
        let dim_inactive_options = vec!["false", "true"];
        let mut dim_inactive_selection_box = NamedSelectionBox::new(
            "Dim inactive:", dim_inactive_options,
            Some(settings.clone().borrow().appearance_settings.dim_inactive.to_string().as_str()),
            Some(dim_inactive_selection_changed_callback)
        );
        dim_inactive_selection_box.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Active opacity option
        let settings_clone = settings.clone();
        let active_opacity_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.active_opacity = spin_button.value();
        };
        let mut active_opacity_spin_button = NamedSpinButtonSection::new(
            "Active opacity:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.active_opacity,
            0.1, 1.0,
            0.0, 1.0, 1, false
        );
        active_opacity_spin_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        active_opacity_spin_button.set_change_callback(active_opacity_spin_button_changed_callback);

        // Inactive opacity option
        let settings_clone = settings.clone();
        let inactive_opacity_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.inactive_opacity = spin_button.value();
        };
        let mut inactive_opacity_spin_button = NamedSpinButtonSection::new(
            "Inactive opacity:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.inactive_opacity,
            0.1, 1.0,
            0.0, 1.0, 1, false
        );
        inactive_opacity_spin_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        inactive_opacity_spin_button.set_change_callback(inactive_opacity_spin_button_changed_callback);

        // Active shadows option
        let settings_clone = settings.clone();
        let active_shadows_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_clone.borrow_mut().appearance_settings.active_shadow = bool_value;
        };
        let mut active_shadows_selection_box = NamedSelectionBox::new(
          "Active shadows:", vec!["false", "true"],
            Some(settings.clone().borrow().appearance_settings.active_shadow.to_string().as_str()),
            Some(active_shadows_selection_changed_callback)
        );
        active_shadows_selection_box.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Shadow range option
        let settings_clone = settings.clone();
        let shadow_range_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.shadow_range = spin_button.value();
        };
        let mut shadow_range_spin_button = NamedSpinButtonSection::new(
            "Shadow range:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.shadow_range,
            0.1, 1.0,
            0.0, 1.0, 1, false,
        );
        shadow_range_spin_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        shadow_range_spin_button.set_change_callback(shadow_range_spin_button_changed_callback);

        // Shadow render power option
        let settings_clone = settings.clone();
        let shadow_render_power_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.shadow_render_power = spin_button.value();
        };
        let mut shadow_render_power_spin_button = NamedSpinButtonSection::new(
            "Shadow render power:",
            0.0, 100.0,
            settings.clone().borrow().appearance_settings.shadow_render_power,
            0.1, 1.0,
            0.0, 1.0, 1, false
        );
        shadow_render_power_spin_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        shadow_render_power_spin_button.set_change_callback(shadow_render_power_spin_button_changed_callback);

        // Shadow color option
        let settings_clone = settings.clone();
        let shadow_color_button_changed_callback = move |color_button: &ColorButton| {
            settings_clone.borrow_mut().appearance_settings.shadow_color = RGBAColor::new(color_button.rgba());
        };
        let mut shadow_color_button = NamedColorButtonSection::new(
            "Shadow color:", 
            Some(settings.clone().borrow().appearance_settings.shadow_color.clone()),
            shadow_color_button_changed_callback
        );
        shadow_color_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Active blur option
        let settings_clone = settings.clone();
        let active_blur_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_clone.borrow_mut().appearance_settings.active_blur = bool_value;
        };
        let active_blur_options = vec!["false", "true"];
        let mut active_blur_selection_box = NamedSelectionBox::new(
            "Active blur:", active_blur_options,
            Some(settings.clone().borrow().appearance_settings.active_blur.to_string().as_str()),
            Some(active_blur_selection_changed_callback)
        );
        active_blur_selection_box.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Blur size option
        let settings_clone = settings.clone();
        let blur_size_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.blur_size = spin_button.value();
        };
        let mut blur_size_spin_button = NamedSpinButtonSection::new(
            "Blur size:", 0.0, 100.0,
            settings.clone().borrow().appearance_settings.blur_size,
            0.1,
            1.0, 0.0, 1.0, 1, false
        );
        blur_size_spin_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        blur_size_spin_button.set_change_callback(blur_size_spin_button_changed_callback);

        // Blur passes option
        let settings_clone = settings.clone();
        let blur_passes_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.blur_passes = spin_button.value() as usize;
        };
        let mut blur_passes_spin_button = NamedSpinButtonSection::new(
            "Blur passes:", 0.0, 100.0,
            settings.clone().borrow().appearance_settings.blur_passes as f64,
            0.1,
            1.0, 0.0, 1.0, 1, false
        );
        blur_passes_spin_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        blur_passes_spin_button.set_change_callback(blur_passes_spin_button_changed_callback);

        // Blur vibrancy option
        let settings_clone = settings.clone();
        let blur_vibrancy_spin_button_changed_callback = move |spin_button: &SpinButton| {
            settings_clone.borrow_mut().appearance_settings.blur_vibrancy = spin_button.value();
        };
        let mut blur_vibrancy_spin_button = NamedSpinButtonSection::new(
            "Blur vibrancy:", 0.0, 100.0,
            settings.clone().borrow().appearance_settings.blur_vibrancy,
            0.1,
            1.0, 0.0, 1.0, 1, false
        );
        blur_vibrancy_spin_button.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);
        blur_vibrancy_spin_button.set_change_callback(blur_vibrancy_spin_button_changed_callback);

        decorations_box.append(&decoration_label);
        decorations_box.append(&separator);
        decorations_box.append(rounding_spin_button.get_container_box());
        decorations_box.append(rounding_spin_power_button.get_container_box());
        decorations_box.append(dim_inactive_selection_box.get_container_box());
        decorations_box.append(active_opacity_spin_button.get_container_box());
        decorations_box.append(inactive_opacity_spin_button.get_container_box());
        decorations_box.append(active_shadows_selection_box.get_container_box());
        decorations_box.append(shadow_range_spin_button.get_container_box());
        decorations_box.append(shadow_render_power_spin_button.get_container_box());
        decorations_box.append(shadow_color_button.get_container_box());
        decorations_box.append(active_blur_selection_box.get_container_box());
        decorations_box.append(blur_size_spin_button.get_container_box());
        decorations_box.append(blur_passes_spin_button.get_container_box());
        decorations_box.append(blur_vibrancy_spin_button.get_container_box());
        decorations_box
    }

    fn create_animations_section(_: &Rc<RefCell<HyprlandSettings>>) -> gtk::Box {
        gtk::Box::new(Orientation::Vertical, 10)
    }

    fn create_layouts_section(settings: &Rc<RefCell<HyprlandSettings>>) -> gtk::Box {
        const LAYOUT_LABEL: &str = "Layout";
        let layouts_box = gtk::Box::new(Orientation::Vertical, 10);
        let layout_label = Label::new(Some(LAYOUT_LABEL));
        let separator = Separator::new(Orientation::Horizontal);

        // Layout option
        let settings_clone = settings.clone();
        let layout_input_changed_callback = move |entry: &Entry| {
          settings_clone.borrow_mut().appearance_settings.layout = entry.text().to_string();
        };
        let layout_input_section = NamedInputSection::new(
            "Layout:", "e.g. dwindle, ",
            Some(settings.clone().borrow().appearance_settings.layout.clone())
        );
        layout_input_section.set_input_callback(layout_input_changed_callback);

        // Pseudo tiling option
        let settings_clone = settings.clone();
        let pseudo_tiling_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_clone.borrow_mut().appearance_settings.pseudo_tiling = bool_value;
        };
        let pseudo_tiling_options = vec!["false", "true"];
        let mut pseudo_tiling_selection_box = NamedSelectionBox::new(
          "Pseudo tiling:", pseudo_tiling_options,
            Some(settings.clone().borrow().appearance_settings.pseudo_tiling.to_string().as_str()),
            Some(pseudo_tiling_selection_changed_callback)
        );
        pseudo_tiling_selection_box.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Split preservation option
        let settings_clone = settings.clone();
        let split_preservation_selection_changed_callback = move |entry: &ComboBoxText| {
            let selected_text = entry.active_text().expect("Cannot read active selection text");
            let bool_value = selected_text.parse::<bool>().expect("Cannot parse bool value");
            settings_clone.borrow_mut().appearance_settings.split_preservation = bool_value;
        };
        let split_preservation_options = vec!["false", "true"];
        let mut split_preservation_selection_box = NamedSelectionBox::new(
            "Split preservation:", split_preservation_options,
            Some(settings.clone().borrow().appearance_settings.force_default_wallpaper.to_string().as_str()),
            Some(split_preservation_selection_changed_callback)
        );
        split_preservation_selection_box.set_label_width(AppearancePanel::APPEARANCE_LABEL_WIDTH);

        // Master status option
        let settings_clone = settings.clone();
        let master_status_input_changed_callback = move |entry: &Entry| {
            settings_clone.borrow_mut().appearance_settings.master_status = entry.text().to_string();
        };
        let master_status_input_section = NamedInputSection::new(
            "Master status:", "e.g. master, ",
            Some(settings.clone().borrow().appearance_settings.master_status.clone())
        );
        master_status_input_section.set_input_callback(master_status_input_changed_callback);

        layouts_box.append(&layout_label);
        layouts_box.append(&separator);
        layouts_box.append(layout_input_section.get_container_box());
        layouts_box.append(master_status_input_section.get_container_box());
        layouts_box.append(pseudo_tiling_selection_box.get_container_box());
        layouts_box.append(split_preservation_selection_box.get_container_box());
        layouts_box
    }
}