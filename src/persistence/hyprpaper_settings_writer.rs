use crate::models::settings::appearance_settings::AppearanceSettings;
use crate::persistence::hyprland_writer_utils::HyprlandWriterUtils;
use crate::persistence::settings_writer::SettingsWriter;

pub struct HyprpaperSettingsWriter {
    config_lines: Vec<String>
}

impl SettingsWriter<AppearanceSettings> for HyprpaperSettingsWriter {
    fn serialize_settings(&mut self, settings: AppearanceSettings) {
        self.serialize_wallpaper_settings(&settings);
    }

    fn write_to_config(&self) {
        let hyprpaper_config_file_path = HyprlandWriterUtils::create_hyprland_config_path("hyprpaper.conf");

        if !self.config_lines.is_empty() {
            HyprlandWriterUtils::write_content_to_file(hyprpaper_config_file_path.as_str(), self.config_lines.clone())
        }
    }
}

impl HyprpaperSettingsWriter {
    pub fn new() -> Self {
        Self {
            config_lines: Vec::new()
        }
    }

    fn serialize_wallpaper_settings(&mut self, settings: &AppearanceSettings) {
        let wallpaper_path = settings.wallpaper_path.clone();
        if wallpaper_path.is_empty() {
            return
        }

        self.add_line_entry(HyprlandWriterUtils::create_value_pair(
            "preload".to_string(), wallpaper_path.clone()
        ));
        self.add_line_entry(HyprlandWriterUtils::create_value_pair(
            "wallpaper".to_string(), format!(",{}", wallpaper_path)
        ));
    }

    fn add_line_entry(&mut self, text: String) {
        self.config_lines.push(text);
    }
}