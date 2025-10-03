use gtk::{Label, LinkButton, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::models::modules::program_module::ProgramModule;
use crate::providers::application_provider::ApplicationProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::controls::Control;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::css_styles::CSSStyles;

pub struct Overview {
    overview_box: GTKBox,
}

impl Control for Overview {
    fn get_widget(&self) -> &GTKBox {
        &self.overview_box
    }
}

impl Overview {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let overview_box = BoxBuilder::new("overview")
            .set_orientation(Orientation::Vertical)
            .set_margin(DEFAULT_MARGIN)
            .build();

        let hyprland_modules_box = Self::create_hyprland_modules_section_box(&application_provider);
        let dependency_modules_box = Self::create_dependency_modules_section_box(&application_provider);
        overview_box.append(&hyprland_modules_box);
        overview_box.append(&dependency_modules_box);

        Self {
            overview_box
        }
    }

    fn create_hyprland_modules_section_box(application_provider: &ApplicationProvider) -> GTKBox {
        const HYPRLAND_MODULES_LABEL: &str = "Hyprland modules";
        let hyprland_modules_section_box = SectionBoxBuilder::new("hyprland-modules-section", 0)
            .create_header_elements(HYPRLAND_MODULES_LABEL)
            .build()
            .expect("Failed to create hyprland modules section box");

        let module_provider = application_provider.get_program_provider();
        let hyprland_modules = module_provider.borrow().get_hyprland_modules();

        for module in hyprland_modules {
            let module_entry_box = Self::create_module_entry_box(module);
            hyprland_modules_section_box.append(&module_entry_box);
        }

        hyprland_modules_section_box
    }

    fn create_dependency_modules_section_box(application_provider: &ApplicationProvider) -> GTKBox {
        const DEPENDENCY_MODULES_LABEL: &str = "Dependency modules";
        let dependency_modules_section_box = SectionBoxBuilder::new("dependency-modules-section", 0)
            .create_header_elements(DEPENDENCY_MODULES_LABEL)
            .build()
            .expect("Failed to create dependency modules section box");

        let module_provider = application_provider.get_program_provider();
        let dependency_modules = module_provider.borrow().get_dependency_modules();

        for module in dependency_modules {
            let module_entry_box = Self::create_module_entry_box(module);
            dependency_modules_section_box.append(&module_entry_box);
        }

        dependency_modules_section_box
    }

    fn create_module_entry_box(program_module: ProgramModule) -> GTKBox {
        let module_name = program_module.info.name.clone();
        let module_version = program_module.version.unwrap_or("not installed".to_string());

        let module_entry_box = BoxBuilder::new("module-entry")
            .set_orientation(Orientation::Vertical)
            .set_margin_bottom(DEFAULT_MARGIN)
            .build();

        let module_information_box = BoxBuilder::new("module-information")
            .set_orientation(Orientation::Horizontal)
            .build();

        let module_name_label = Label::new(Some(format!("{}: ", module_name).as_str()));
        module_name_label.set_valign(gtk::Align::Center);
        module_name_label.set_xalign(0.0);
        module_name_label.add_css_class(CSSStyles::HYPRLAND_MODULE_NAME);
        module_name_label.set_width_request(120);

        let module_version_label = Label::new(Some(module_version.as_str()));
        module_version_label.set_valign(gtk::Align::Center);

        let module_link = LinkButton::new(program_module.info.link.as_str());
        module_link.set_halign(gtk::Align::Start);
        module_link.set_valign(gtk::Align::Center);
        module_link.add_css_class(CSSStyles::LINK_BUTTON_TEXT);

        module_information_box.append(&module_name_label);
        module_information_box.append(&module_version_label);
        module_entry_box.append(&module_information_box);
        module_entry_box.append(&module_link);
        module_entry_box
    }
}