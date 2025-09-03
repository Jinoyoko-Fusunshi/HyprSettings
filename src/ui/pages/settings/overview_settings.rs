use gtk::{Label, LinkButton, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::models::modules::program_module::ProgramModule;
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::controls::Control;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::css_styles::CSSStyles;

pub struct OverviewSettings {
    overview_box: gtk::Box,
}

impl Control for OverviewSettings {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.overview_box
    }
}

impl OverviewSettings {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let overview_box = gtk::Box::new(Orientation::Vertical, 10);
        overview_box.set_margin_top(10);
        overview_box.set_margin_bottom(10);
        overview_box.set_margin_start(10);
        overview_box.set_margin_end(10);

        let hyprland_modules_box = Self::create_hyprland_modules_box(&application_provider);
        let dependency_modules_box = Self::create_dependency_program_modules_box(&application_provider);
        overview_box.append(&hyprland_modules_box);
        overview_box.append(&dependency_modules_box);

        Self {
            overview_box
        }
    }

    fn create_hyprland_modules_box(application_provider: &ApplicationProvider) -> gtk::Box {
        const HYPRLAND_MODULES_LABEL: &str = "Hyprland modules";
        let hyprland_modules_section_box = SectionBoxBuilder::new()
            .create_header_elements(HYPRLAND_MODULES_LABEL)
            .build()
            .expect("Failed to create hyprland modules section box");

        let module_provider = application_provider.get_module_provider();
        let hyprland_modules = module_provider.borrow().get_hyprland_modules();

        for module in hyprland_modules {
            let module_entry_box = Self::create_module_entry_box(module);
            hyprland_modules_section_box.append(&module_entry_box);
        }

        hyprland_modules_section_box
    }

    fn create_dependency_program_modules_box(application_provider: &ApplicationProvider) -> gtk::Box {
        const DEPENDENCY_MODULES_LABEL: &str = "Program dependency modules";
        let dependency_modules_section_box = SectionBoxBuilder::new()
            .create_header_elements(DEPENDENCY_MODULES_LABEL)
            .build()
            .expect("Failed to create dependant modules section box");

        let module_provider = application_provider.get_module_provider();
        let dependency_modules = module_provider.borrow().get_dependency_modules();

        for module in dependency_modules {
            let module_entry_box = Self::create_module_entry_box(module);
            dependency_modules_section_box.append(&module_entry_box);
        }

        dependency_modules_section_box
    }

    fn create_module_entry_box(program_module: ProgramModule) -> gtk::Box {
        let module_name = program_module.info.name.clone();
        let module_version = program_module.version.unwrap_or("not installed".to_string());

        let module_entry_box = gtk::Box::new(Orientation::Vertical, 0);
        module_entry_box.set_margin_bottom(10);

        let module_information_box = gtk::Box::new(Orientation::Horizontal, 10);
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