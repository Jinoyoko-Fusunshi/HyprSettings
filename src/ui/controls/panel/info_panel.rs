use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Label, LinkButton, Orientation, Separator};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::controls::panel::Panel;
use crate::ui::css_styles::CSSStyles;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct InfoPanel {
    widget: gtk::Box,
}

impl Panel for InfoPanel {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.widget
    }
}

impl Clone for InfoPanel {
    fn clone(&self) -> Self {
        Self {
            widget: self.widget.clone()
        }
    }   
}

impl InfoPanel {
    pub fn new() -> Self {
        const INFO_PANEL_LABEL: &str = "Program information";

        let info_panel_box = gtk::Box::new(Orientation::Vertical, 10);
        info_panel_box.set_margin_top(10);
        info_panel_box.set_margin_bottom(10);
        info_panel_box.set_margin_start(10);
        info_panel_box.set_margin_end(10);

        let info_panel_label = Label::new(Some(INFO_PANEL_LABEL));
        let separator = Separator::new(Orientation::Horizontal);

        let application_name = InfoPanel::create_label("HyprSettings");
        application_name.add_css_class(CSSStyles::APPLICATION_TITLE);

        let application_version_value = format!("v.{}", env!("CARGO_PKG_VERSION"));
        let application_version_entry = InfoPanel::create_information_entry(
            "🚀 Version:", application_version_value.as_str()
        );
        let author_entry = InfoPanel::create_information_entry(
            "👨‍💻 Author:", "Jinoyoko Fusunshi"
        );
        let github_link_entry = InfoPanel::create_link_entry(
            "📄 Github:", "https://github.com/Jinoyoko-Fusunshi/HyprSettings"
        );
        let horizontal_separator = Separator::new(Orientation::Horizontal);
        let program_description_entry = InfoPanel::create_program_description_panel();

        info_panel_box.append(&info_panel_label);
        info_panel_box.append(&separator);
        info_panel_box.append(&application_name);
        info_panel_box.append(&application_version_entry);
        info_panel_box.append(&author_entry);
        info_panel_box.append(&github_link_entry);
        info_panel_box.append(&horizontal_separator);
        info_panel_box.append(&program_description_entry);

        Self {
            widget: info_panel_box
        }
    }

    fn create_information_entry(information_name: &str, information_value: &str) -> gtk::Box {
        let entry = gtk::Box::new(Orientation::Horizontal, 10);
        let information_name_label = InfoPanel::create_label(information_name);
        let information_value_label = InfoPanel::create_label(information_value);

        entry.append(&information_name_label);
        entry.append(&information_value_label);
        entry
    }

    fn create_program_description_panel() -> gtk::Box {
        let program_description_box = gtk::Box::new(Orientation::Vertical, 10);
        program_description_box.set_hexpand(false);
        program_description_box.set_width_request(100);

        const USAGE_DESCRIPTION_TEXT: &str = "\
            HyprSettings is a simple Hyprland settings manager for configuring your hyprland instance.\n\
            All hyprland settings will be read and written into its dedicated config file path.\n\
            It accepts configuration settings for the hyprland components:\
        ";
        let usage_description_label = Label::new(Some(USAGE_DESCRIPTION_TEXT));
        usage_description_label.set_xalign(0.0);
        usage_description_label.set_wrap(true);

        const HYPRLAND_COMPONENTS: [(&str, &str); 4] = [
            ("hyprland (core)", "https://wiki.hypr.land/"),
            ("hyprpaper", "https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/"),
            ("hyprlock", "https://wiki.hypr.land/Hypr-Ecosystem/hyprlock/"),
            ("hyprpolkitagent", "https://wiki.hypr.land/Hypr-Ecosystem/hyprpolkitagent/"),
        ];
        let hyprland_component_box = gtk::Box::new(Orientation::Vertical, 6);
        hyprland_component_box.set_hexpand(false);
        hyprland_component_box.set_halign(Align::Start);
        for (component_name, component_link) in HYPRLAND_COMPONENTS {
            let component_link_name = format!("• {}", component_name);
            let component_link_button = InfoPanel::create_link_button(
                component_link_name.as_str(), component_link
            );
            hyprland_component_box.append(&component_link_button);
        }

        program_description_box.append(&usage_description_label);
        program_description_box.append(&hyprland_component_box);
        program_description_box
    }

    fn create_link_entry(link_name: &str, link_value: &str) -> gtk::Box {
        let entry = gtk::Box::new(Orientation::Horizontal, 10);
        let link_name_label = InfoPanel::create_label(link_name);
        let link_button = InfoPanel::create_link_button(link_value, link_value);

        entry.append(&link_name_label);
        entry.append(&link_button);
        entry
    }

    fn create_link_button(display_name: &str, link_value: &str) -> LinkButton {
        let link_button = LinkButton::with_label(link_value, display_name);
        link_button.add_css_class(CSSStyles::LINK_BUTTON_TEXT);
        link_button.set_hexpand(false);
        link_button.set_halign(Align::Start);
        link_button
    }

    fn create_label(label_text: &str) -> Label {
        let label = Label::new(Some(label_text));
        label.set_xalign(0.0);
        label
    }
}