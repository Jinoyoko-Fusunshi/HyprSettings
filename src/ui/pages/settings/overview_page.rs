use std::process::Command;
use gtk::{Label, LinkButton, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::component::Component;
use crate::ui::component_section_builder::SectionBoxBuilder;
use crate::ui::css_styles::CSSStyles;

#[derive(Clone)]
struct ProgramModule {
    info: ProgramModuleInfo,
    version: Option<String>,
}

#[derive(Clone)]
struct ProgramModuleInfo {
    name: String,
    link: String,
}

pub struct OverviewPage {
    overview_box: gtk::Box,
}

impl Component for OverviewPage {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.overview_box
    }
}

impl OverviewPage {
    pub fn new() -> Self {
        let overview_box = gtk::Box::new(Orientation::Vertical, 10);
        overview_box.set_margin_top(10);
        overview_box.set_margin_bottom(10);
        overview_box.set_margin_start(10);
        overview_box.set_margin_end(10);

        let hyprland_modules_box = Self::create_hyprland_modules_box();
        let dependency_modules_box = Self::create_dependency_program_modules_box();
        overview_box.append(&hyprland_modules_box);
        overview_box.append(&dependency_modules_box);

        Self {
            overview_box
        }
    }

    fn create_hyprland_modules_box() -> gtk::Box {
        const HYPRLAND_MODULES_LABEL: &str = "Hyprland modules";
        let hyprland_modules_section_box = SectionBoxBuilder::new()
            .create_header_elements(HYPRLAND_MODULES_LABEL)
            .build()
            .expect("Failed to create hyprland modules section box");

        let hyprland_module_infos: Vec<ProgramModuleInfo> = vec![
            ProgramModuleInfo {
                name: "hyprland".to_string(),
                link: "https://wiki.hypr.land/".to_string()
            },
            ProgramModuleInfo {
                name: "hypridle".to_string(),
                link: "https://wiki.hypr.land/Hypr-Ecosystem/hypridle/".to_string()
            },
            ProgramModuleInfo {
                name: "hyprpaper".to_string(),
                link: "https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/".to_string()
            },
            ProgramModuleInfo {
                name: "hyprlock".to_string(),
                link: "https://wiki.hypr.land/Hypr-Ecosystem/hyprlock/".to_string()
            },
            ProgramModuleInfo {
                name: "hyprpolkitagent".to_string(),
                link: "https://wiki.hypr.land/Hypr-Ecosystem/hyprpolkitagent/".to_string()
            },
        ];

        for module_name in hyprland_module_infos {
            let module_entry_box = Self::create_module_entry_box(module_name);
            hyprland_modules_section_box.append(&module_entry_box);
        }

        hyprland_modules_section_box
    }

    fn create_dependency_program_modules_box() -> gtk::Box {
        const DEPENDENCY_MODULES_LABEL: &str = "Program dependency modules";
        let dependency_modules_section_box = SectionBoxBuilder::new()
            .create_header_elements(DEPENDENCY_MODULES_LABEL)
            .build()
            .expect("Failed to create dependant modules section box");

        let dependency_module_infos: Vec<ProgramModuleInfo> = vec![
            ProgramModuleInfo {
                name: "wlr-randr".to_string(),
                link: "https://man.archlinux.org/man/extra/wlr-randr/wlr-randr.1.en".to_string()
            },
        ];

        for module_name in dependency_module_infos {
            let module_entry_box = Self::create_module_entry_box(module_name);
            dependency_modules_section_box.append(&module_entry_box);
        }

        dependency_modules_section_box
    }

    fn create_module_entry_box(module_info: ProgramModuleInfo) -> gtk::Box {
        let program_module = Self::get_program_module(module_info.clone());
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

        let module_link = LinkButton::new(module_info.link.as_str());
        module_link.set_halign(gtk::Align::Start);
        module_link.set_valign(gtk::Align::Center);
        module_link.add_css_class(CSSStyles::LINK_BUTTON_TEXT);

        module_information_box.append(&module_name_label);
        module_information_box.append(&module_version_label);
        module_entry_box.append(&module_information_box);
        module_entry_box.append(&module_link);
        module_entry_box
    }

    fn get_program_module(module_info: ProgramModuleInfo) -> ProgramModule {
        let program_result = Command::new("pacman")
            .arg("-Q")
            .arg(module_info.name.clone())
            .output();

        match program_result {
            Ok(output) => {
                if output.status.success() {
                    let output_string = String::from_utf8(output.stdout).unwrap();
                    let split_output_string = output_string.split(" ")
                        .collect::<Vec<&str>>();

                    let hyprland_module_version = split_output_string[1].to_string()
                        .replace("\n", "");

                    ProgramModule {
                        info: module_info,
                        version: Some(hyprland_module_version)
                    }
                } else {
                    ProgramModule {
                        info: module_info,
                        version: None
                    }
                }
            }
            Err(_) => ProgramModule {
                info: module_info,
                version: None
            }
        }
    }
}