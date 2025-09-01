use std::collections::HashMap;
use std::process::Command;
use crate::models::modules::program_module::ProgramModule;
use crate::models::modules::program_module_info::ProgramModuleInfo;
use crate::models::modules::{ProgramModuleCategory, HYPRIDLE_MODULE, HYPRLAND_CORE_MODULE, HYPRLOCK_MODULE, HYPRPAPER_MODULE, HYPRPOLKIT_AGENT_MODULE, WAYLANDRANDR_MODULE};

pub struct ModuleProvider {
    program_modules: HashMap<String, ProgramModule>,
}

impl ModuleProvider {
    pub fn new() -> Self {
        Self {
            program_modules: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        let hyprland_module_infos: Vec<ProgramModuleInfo> = vec![
            ProgramModuleInfo {
                name: HYPRLAND_CORE_MODULE.to_string(),
                link: "https://wiki.hypr.land/".to_string(),
                category: ProgramModuleCategory::Hyprland
            },
            ProgramModuleInfo {
                name: HYPRIDLE_MODULE.to_string(),
                link: "https://wiki.hypr.land/Hypr-Ecosystem/hypridle/".to_string(),
                category: ProgramModuleCategory::Hyprland
            },
            ProgramModuleInfo {
                name: HYPRPAPER_MODULE.to_string(),
                link: "https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/".to_string(),
                category: ProgramModuleCategory::Hyprland
            },
            ProgramModuleInfo {
                name: HYPRLOCK_MODULE.to_string(),
                link: "https://wiki.hypr.land/Hypr-Ecosystem/hyprlock/".to_string(),
                category: ProgramModuleCategory::Hyprland
            },
            ProgramModuleInfo {
                name: HYPRPOLKIT_AGENT_MODULE.to_string(),
                link: "https://wiki.hypr.land/Hypr-Ecosystem/hyprpolkitagent/".to_string(),
                category: ProgramModuleCategory::Hyprland
            },
        ];
        self.load_modules(&hyprland_module_infos);

        let dependency_module_infos: Vec<ProgramModuleInfo> = vec![
            ProgramModuleInfo {
                name: WAYLANDRANDR_MODULE.to_string(),
                link: "https://man.archlinux.org/man/extra/wlr-randr/wlr-randr.1.en".to_string(),
                category: ProgramModuleCategory::Dependency
            },
        ];
        self.load_modules(&dependency_module_infos);
    }

    pub fn get_module(&self, module_name: String) -> Option<ProgramModule> {
        self.program_modules.iter()
            .find(|(name, _)| **name == module_name)
            .map(|(_, module)| module.clone())
    }

    pub fn get_hyprland_modules(&self) -> Vec<ProgramModule> {
        self.program_modules.iter()
            .filter(|(_, module)| matches!(module.info.category, ProgramModuleCategory::Hyprland))
            .map(|(_, module)| module.clone())
            .into_iter()
            .collect::<Vec<ProgramModule>>()
    }

    pub fn get_dependency_modules(&self) -> Vec<ProgramModule> {
        self.program_modules.iter()
            .filter(|(_, module)| matches!(module.info.category, ProgramModuleCategory::Dependency))
            .map(|(_, module)| module.clone())
            .into_iter()
            .collect::<Vec<ProgramModule>>()
    }

    fn load_modules(&mut self,  module_infos: &Vec<ProgramModuleInfo>) {
        for module_info in module_infos {
            let program_module = ModuleProvider::get_program_module(module_info.clone());
            self.program_modules.insert(module_info.name.clone(), program_module);
        }
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