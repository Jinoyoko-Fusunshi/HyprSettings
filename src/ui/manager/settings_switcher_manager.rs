use std::cell::RefCell;
use std::rc::Rc;
use crate::providers::application_provider::ApplicationProvider;
use crate::persistence::hyprland_settings_writer::HyprlandSettingsWriter;
use crate::persistence::settings_writer::SettingsWriter;
use crate::persistence::yaml_settings_writer::YamlSettingsWriter;
use crate::models::settings::hyprland_settings::HyprlandSettings;
use crate::persistence::hyprlock_settings_writer::HyprlockSettingsWriter;
use crate::persistence::hyprpaper_settings_writer::HyprpaperSettingsWriter;
use crate::ui::controls::settings_switcher::SettingsSwitcher;
use crate::ui::updatable_control::UpdatableControl;
use crate::ui::states::settings_switcher_state::SettingsSwitcherState;

#[derive(Clone)]
pub struct SettingsSwitcherManager {
    settings_switcher: Rc<RefCell<SettingsSwitcher>>,
    application_provider: ApplicationProvider
}

pub enum SettingsSwitcherEvent {
    NewControlName(String),
    SaveSettings,
}

impl SettingsSwitcherManager {
    pub fn new(
        settings_switcher: Rc<RefCell<SettingsSwitcher>>,
        application_provider: ApplicationProvider
    ) -> Self {
        Self {
            settings_switcher,
            application_provider
        }
    }

    pub fn notify_event(&self, event: SettingsSwitcherEvent) {
        match  event {
            SettingsSwitcherEvent::NewControlName(name) => {
                let settings_switcher_state = SettingsSwitcherState::new(name);
                let mut settings_switcher = self.settings_switcher.borrow_mut();
                settings_switcher.update_ui(settings_switcher_state);
            },
            SettingsSwitcherEvent::SaveSettings => {
                let program_settings = self.application_provider
                    .get_program_provider().borrow().get_settings();

                let monitor_settings = self.application_provider
                    .get_monitor_provider().borrow().get_settings();


                let appearance_settings = self.application_provider
                    .get_appearance_provider().borrow().get_settings();

                let input_settings = self.application_provider
                    .get_input_provider().borrow().get_settings();
                
                let keybind_settings = self.application_provider
                    .get_keybinds_provider().borrow().get_settings();

                let lockscreen_settings = self.application_provider
                    .get_lockscreen_provider().borrow().get_settings();

                let hyprland_settings = HyprlandSettings::new(
                    program_settings.clone(),
                    monitor_settings.clone(),
                    appearance_settings.clone(),
                    input_settings.clone(),
                    keybind_settings.clone(),
                    lockscreen_settings.clone()
                );

                let mut yaml_settings_writer = YamlSettingsWriter::new();
                yaml_settings_writer.serialize_settings(hyprland_settings.clone());
                yaml_settings_writer.write_to_config();
                
                let mut hyprland_settings_writer = HyprlandSettingsWriter::new();
                hyprland_settings_writer.serialize_settings(hyprland_settings.clone());
                hyprland_settings_writer.write_to_config();

                let mut hyprpaper_settings_writer = HyprpaperSettingsWriter::new();
                hyprpaper_settings_writer.serialize_settings(appearance_settings.clone());
                hyprpaper_settings_writer.write_to_config();
                
                let mut hyprlock_settings_writer = HyprlockSettingsWriter::new();
                hyprlock_settings_writer.serialize_settings(lockscreen_settings.clone());
                hyprlock_settings_writer.write_to_config();
            }
        }
    }
}