use std::cell::RefCell;
use std::rc::Rc;
use crate::providers::appearance_provider::AppearanceProvider;
use crate::providers::monitor_provider::MonitorProvider;
use crate::models::settings::appearance_settings::AppearanceSettings;
use crate::persistence::settings_reader::SettingsReader;
use crate::persistence::yaml_settings_reader::YamlSettingsReader;
use crate::models::settings::monitor_settings::MonitorSettings;
use crate::models::settings::hyprland_settings::HyprlandSettings;
use crate::models::settings::input_settings::InputSettings;
use crate::models::settings::keybind_settings::KeyBindSettings;
use crate::models::settings::lockscreen_settings::LockScreenSettings;
use crate::models::settings::program_settings::ProgramSettings;
use crate::providers::input_provider::InputProvider;
use crate::providers::keybind_provider::KeybindProvider;
use crate::providers::lockscreen_provider::LockscreenProvider;
use crate::providers::module_provider::ModuleProvider;
use crate::utils::{new_rc_mut, RcMut};

#[derive(Clone)]
pub struct ApplicationProvider {
    module_provider: RcMut<ModuleProvider>,
    monitor_provider: RcMut<MonitorProvider>,
    appearance_provider: RcMut<AppearanceProvider>,
    lockscreen_provider: RcMut<LockscreenProvider>,
    input_provider: RcMut<InputProvider>,
    keybind_provider: RcMut<KeybindProvider>,
}

impl ApplicationProvider {
    pub fn new() -> Self {
        let hyprland_settings = Self::get_config_settings();
        let module_provider = Self::create_module_provider(&hyprland_settings);
        let has_wlrrandr_module = module_provider
            .borrow()
            .has_dependency_module("wlr-randr".to_string());

        let monitor_provider = Self::create_monitor_provider(&hyprland_settings, has_wlrrandr_module);
        let appearance_provider = Self::create_appearance_provider(&hyprland_settings);
        let lockscreen_provider = Self::create_lockscreen_provider(&hyprland_settings);
        let keybind_provider = Self::create_keybind_provider(&hyprland_settings);
        let input_provider = Self::create_input_provider(&hyprland_settings);

        Self {
            module_provider,
            monitor_provider,
            appearance_provider,
            lockscreen_provider,
            input_provider,
            keybind_provider,
        }
    }

    pub fn get_program_provider(&self) -> Rc<RefCell<ModuleProvider>> {
        self.module_provider.clone()
    }

    pub fn get_monitor_provider(&self) -> Rc<RefCell<MonitorProvider>> {
        self.monitor_provider.clone()
    }

    pub fn get_appearance_provider(&self) -> Rc<RefCell<AppearanceProvider>> {
        self.appearance_provider.clone()
    }

    pub fn get_lockscreen_provider(&self) -> Rc<RefCell<LockscreenProvider>> {
        self.lockscreen_provider.clone()
    }

    pub fn get_keybinds_provider(&self) -> Rc<RefCell<KeybindProvider>> {
        self.keybind_provider.clone()
    }

    pub fn get_input_provider(&self) -> Rc<RefCell<InputProvider>> {
        self.input_provider.clone()
    }

    fn create_module_provider(settings: &Option<HyprlandSettings>) -> RcMut<ModuleProvider> {
        let mut module_provider = if let Some(settings) = settings {
            ModuleProvider::new(settings.program_settings.clone())
        } else {
            ModuleProvider::new(ProgramSettings::default())
        };
        module_provider.init();

        new_rc_mut(module_provider)
    }

    fn create_monitor_provider(settings: &Option<HyprlandSettings>, has_wlrrandr_module: bool) -> RcMut<MonitorProvider> {
        let monitor_provider = if let Some(settings) = settings {
            MonitorProvider::new(settings.monitor_settings.clone())
        } else {
            let mut monitor_provider = MonitorProvider::new(MonitorSettings::default());
            if has_wlrrandr_module {
                monitor_provider.fetch_monitors();
            }

            monitor_provider
        };

        new_rc_mut(monitor_provider)
    }

    fn create_appearance_provider(settings: &Option<HyprlandSettings>) -> RcMut<AppearanceProvider> {
        let appearance_provider = if let Some(settings) = settings {
            AppearanceProvider::new(settings.appearance_settings.clone())
        } else {
            AppearanceProvider::new(AppearanceSettings::default())
        };

        new_rc_mut(appearance_provider)
    }

    fn create_lockscreen_provider(settings: &Option<HyprlandSettings>) -> RcMut<LockscreenProvider> {
        let lockscreen_provider = if let Some(settings) = settings {
            LockscreenProvider::new(settings.lockscreen_settings.clone())
        } else {
            LockscreenProvider::new(LockScreenSettings::default())
        };

        new_rc_mut(lockscreen_provider)
    }

    fn create_keybind_provider(settings: &Option<HyprlandSettings>) -> RcMut<KeybindProvider> {
        let keybind_provider = if let Some(settings) = settings {
            KeybindProvider::new(settings.keybind_settings.clone())
        } else {
            KeybindProvider::new(KeyBindSettings::default())
        };

        new_rc_mut(keybind_provider)
    }

    fn create_input_provider(settings: &Option<HyprlandSettings>) -> RcMut<InputProvider> {
        let input_provider = if let Some(settings) = settings {
            InputProvider::new(settings.input_settings.clone())
        } else {
            InputProvider::new(InputSettings::default())
        };

        new_rc_mut(input_provider)
    }

    fn get_config_settings() -> Option<HyprlandSettings> {
        if YamlSettingsReader::config_file_exists() {
            let mut reader = YamlSettingsReader::new();
            reader.read_from_config();
            return Some(reader.deserialize_settings())
        }

        None
    }
}