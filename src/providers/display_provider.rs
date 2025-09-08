use std::collections::HashMap;
use std::process::Command;
use crate::models::monitor::monitor_configuration::MonitorConfiguration;
use crate::models::monitor::monitor_info_parser::MonitorInfoParser;
use crate::models::settings::display_settings::DisplaySettings;

pub struct DisplayProvider {
    settings: DisplaySettings
}

impl DisplayProvider {
    pub fn new(settings: DisplaySettings) -> Self {
        Self {
            settings
        }
    }

    pub fn init_monitors(&mut self) {
        let command_result = Command::new("wlr-randr")
            .output()
            .expect("Error during wlrandr execution");

        let output = String::from_utf8(command_result.stdout)
            .expect("Failed to parse wlr-randr output");

        let mut monitor_info_parser = MonitorInfoParser::new();
        monitor_info_parser.parse_output(&output);
        let monitor_information = monitor_info_parser.get_result();

        let monitor_configurations: HashMap<String, MonitorConfiguration> = monitor_information
            .iter()
            .map(|monitor_information| {

                let port = monitor_information.port_name.clone();
                let configuration = MonitorConfiguration {
                    enabled: true,
                    information: monitor_information.clone(),
                    video_mode: monitor_information.max_video_mode.clone()
                };

                (port, configuration)
            })
            .collect();

        self.settings.monitor_configurations = monitor_configurations;
    }

    pub fn set_monitor_state(&mut self, monitor_port: String, state: bool) {
        let configuration = self.settings.monitor_configurations
            .get_mut(&monitor_port)
            .unwrap();

        configuration.enabled = state;
    }

    pub fn set_monitor_width(&mut self, monitor_port: String, width: u32) {
        let configuration = self.settings.monitor_configurations
            .get_mut(&monitor_port)
            .unwrap();

        configuration.video_mode.width_resolution = width;
    }

    pub fn set_monitor_height(&mut self, monitor_port: String, height: u32) {
        let configuration = self.settings.monitor_configurations
            .get_mut(&monitor_port)
            .unwrap();

        configuration.video_mode.height_resolution = height;
    }

    pub fn set_monitor_refresh_rate(&mut self, monitor_port: String, refresh_rate: u32) {
        let configuration = self.settings.monitor_configurations
            .get_mut(&monitor_port)
            .unwrap();
        
        configuration.video_mode.refresh_rate = refresh_rate;
    }

    pub fn get_monitor_configurations(&self) -> HashMap<String, MonitorConfiguration> {
        self.settings.monitor_configurations.clone()
    }

    pub fn get_settings(&self) -> DisplaySettings {
        self.settings.clone()
    }
}