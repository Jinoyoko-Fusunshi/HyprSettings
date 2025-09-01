use std::cell::RefCell;
use std::rc::Rc;
use crate::providers::application_provider::ApplicationProvider;
use crate::providers::hyprland_settings_provider::config_files::hyprland_settings_writer::HyprlandSettingsWriter;
use crate::providers::hyprland_settings_provider::config_files::settings_writer::SettingsWriter;
use crate::providers::hyprland_settings_provider::config_files::yaml_settings_writer::YamlSettingsWriter;
use crate::ui::controls::settings_switcher::SettingsSwitcher;
use crate::ui::updatable_component::UpdatableComponent;
use crate::ui::states::settings_switcher_state::SettingsSwitcherState;

#[derive(Clone)]
pub struct SettingsSwitcherManager {
    settings_switcher: Rc<RefCell<SettingsSwitcher>>,
    application_provider: ApplicationProvider
}

pub enum SettingsSwitcherEvent {
    NewComponentName(String),
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
            SettingsSwitcherEvent::NewComponentName(name) => {
                let settings_switcher_state = SettingsSwitcherState::new(name);
                let mut settings_switcher = self.settings_switcher.borrow_mut();
                settings_switcher.update_ui(settings_switcher_state);
            },
            SettingsSwitcherEvent::SaveSettings => {
                let settings_provider = self.application_provider.get_settings_provider();
                let settings_provider_ref = settings_provider.borrow();
                let settings = settings_provider_ref.get_settings();

                let mut yaml_settings_writer = YamlSettingsWriter::new();
                yaml_settings_writer.serialize_settings(settings.clone());
                yaml_settings_writer.write_to_config();
                
                let mut conf_settings_writer = HyprlandSettingsWriter::new();
                conf_settings_writer.serialize_settings(settings.clone());
                conf_settings_writer.write_to_config();
            }
        }
    }
}