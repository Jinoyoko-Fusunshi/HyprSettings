use std::collections::HashMap;
use std::process::Command;
use crate::models::modules::program_module::ProgramModule;
use crate::models::modules::program_module_info::ProgramModuleInfo;
use crate::models::modules::{
    ProgramModuleCategory, HYPRIDLE_MODULE, HYPRLAND_CORE_MODULE, HYPRLOCK_MODULE, HYPRPAPER_MODULE,
    HYPRPOLKIT_AGENT_MODULE, WAYLANDRANDR_MODULE
};
use crate::models::settings::program_settings::ProgramSettings;

pub const VIRTUAL_TERMINAL_ENTRY: &str = "VirtualTerminal";
pub const FILE_MANAGER_ENTRY: &str = "FileManager";
pub const QUICK_SEARCH_ENTRY: &str = "QuickSearch";
pub const NOTIFICATION_HANDLER_ENTRY: &str = "NotificationHandler";

pub struct ModuleProvider {
    program_modules: HashMap<String, ProgramModule>,
    settings: ProgramSettings
}

impl ModuleProvider {
    pub fn new(settings: ProgramSettings) -> Self {
        Self {
            program_modules: HashMap::new(),
            settings
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

    pub fn add_program(&mut self, name: String, path: String) {
        self.settings.programs.insert(name, path);
    }

    pub fn set_program_path(&mut self, name: String, path: String) {
        if path.is_empty() {
            self.settings.programs.remove(&name);
            return;
        }

        self.settings.programs.insert(name, path);
    }

    pub fn get_program_path(&self, name: String) -> Option<String> {
        let program_path = self.settings.programs.get(&name);
        program_path.cloned()
    }

    pub fn remove_program(&mut self, name: String) {
        self.settings.programs.remove(&name);
    }

    pub fn get_program_path_or_module(&self, program_module_name: String) -> Option<String> {
        if let Some(path) = self.get_program_path(program_module_name.clone()) {
            return Some(path.clone())
        }

        if let Some(module) = self.program_modules.get(&program_module_name) {
            return Some(module.info.name.clone())
        }

        None
    }

    pub fn get_program_and_module_names(&self) -> Vec<String> {
        let module_names: Vec<String> = self.program_modules.iter()
            .map(|(name, _)| name.to_string())
            .collect();

        let mut program_names: Vec<String> = self.settings.programs.iter()
            .map(|(program_name, _)| program_name.to_string())
            .collect();

        let mut program_module_names = module_names;
        program_module_names.append(&mut program_names);
        program_module_names
    }

    pub fn add_startup_program(&mut self, name: String, path: String) {
        self.settings.startup_programs.insert(name, path);
    }

    pub fn remove_startup_program(&mut self, name: String) {
        self.settings.startup_programs.remove(&name);
    }

    pub fn get_startup_programs(&self) -> HashMap<String, String> {
        self.settings.startup_programs.clone()
    }

    pub fn get_settings(&self) -> ProgramSettings {
        self.settings.clone()
    }

    fn load_modules(&mut self,  infos: &Vec<ProgramModuleInfo>) {
        for module_info in infos {
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