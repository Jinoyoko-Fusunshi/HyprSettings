use gtk::{Align, Label, LinkButton, Orientation, Separator};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::css_styles::CSSStyles;
use crate::ui::controls::Control;
use crate::ui::section_box_builder::SectionBoxBuilder;

pub struct Infos {
    info_settings_box: GTKBox,
}

impl Control for Infos {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.info_settings_box
    }
}

impl Infos {
    pub fn new() -> Self {
        const INFO_LABEL: &str = "Program information";

        let info_box = SectionBoxBuilder::new("info-panel", DEFAULT_MARGIN)
            .create_header_elements(INFO_LABEL)
            .build().expect("Failed to create info panel");

        let application_name = Self::create_label("HyprSettings");
        application_name.set_widget_name("application-title");
        application_name.set_margin_bottom(DEFAULT_MARGIN as i32);

        let application_version_value = format!("v.{}", env!("CARGO_PKG_VERSION"));
        let application_version_entry = Self::create_information_entry_box(
            "🚀 Version:", application_version_value.as_str()
        );
        let author_entry = Self::create_information_entry_box(
            "👨‍💻 Author:", "Jinoyoko Fusunshi"
        );
        let github_link_entry = Self::create_link_entry_box(
            "📄 Github:", "https://github.com/Jinoyoko-Fusunshi/HyprSettings"
        );
        let horizontal_separator = Separator::new(Orientation::Horizontal);
        let program_description_entry = Self::create_program_description_panel();

        info_box.append(&application_name);
        info_box.append(&application_version_entry);
        info_box.append(&author_entry);
        info_box.append(&github_link_entry);
        info_box.append(&horizontal_separator);
        info_box.append(&program_description_entry);

        Self {
            info_settings_box: info_box
        }
    }

    fn create_information_entry_box(information_name: &str, information_value: &str) -> GTKBox {
        let information_entry_box = BoxBuilder::new("information-entry")
            .set_orientation(Orientation::Horizontal)
            .set_margin_bottom(DEFAULT_MARGIN)
            .build();

        let information_name_label = Self::create_label(information_name);
        let information_value_label = Self::create_label(information_value);

        information_entry_box.append(&information_name_label);
        information_entry_box.append(&information_value_label);
        information_entry_box
    }

    fn create_program_description_panel() -> GTKBox {
        let program_description_box = BoxBuilder::new("program-description")
            .set_orientation(Orientation::Vertical)
            .set_width(100)
            .set_full_width(true)
            .build();

        const USAGE_DESCRIPTION_TEXT: &str = "\
            HyprSettings is a simple Hyprland settings manager for configuring your hyprland instance.\n\
            All hyprland settings will be read and written into its dedicated config file path.\n\
            It accepts configurations for the hyprland modules listed in the overview page.";

        let usage_description_label = Label::new(Some(USAGE_DESCRIPTION_TEXT));
        usage_description_label.set_xalign(0.0);
        usage_description_label.set_wrap(true);

        program_description_box.append(&usage_description_label);
        program_description_box
    }

    fn create_link_entry_box(link_name: &str, link_value: &str) -> GTKBox {
        let link_entry_box = BoxBuilder::new("link-entry")
            .set_orientation(Orientation::Horizontal)
            .build();

        let link_name_label = Self::create_label(link_name);
        let link_button = Self::create_link_button(link_value, link_value);

        link_entry_box.append(&link_name_label);
        link_entry_box.append(&link_button);
        link_entry_box
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