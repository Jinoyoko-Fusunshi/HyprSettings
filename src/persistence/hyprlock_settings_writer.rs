use crate::models::settings::lockscreen_settings::LockScreenSettings;
use crate::persistence::hyprland_writer_utils::{ConfigSectionBuilder, HyprlandWriterUtils};
use crate::persistence::settings_writer::SettingsWriter;

pub struct HyprlockSettingsWriter {
    config_lines: Vec<String>
}

impl SettingsWriter<LockScreenSettings> for HyprlockSettingsWriter {
    fn serialize_settings(&mut self, settings: LockScreenSettings) {
        let general_config_section_lines = self.create_general_config_section(&settings);
        self.add_line_entries(general_config_section_lines);

        let background_config_section_lines = self.create_background_config_section(&settings);
        self.add_line_entries(background_config_section_lines);

        let password_config_section_lines = self.create_password_config_section(&settings);
        self.add_line_entries(password_config_section_lines);

        let text_display_config_section_lines = self.create_text_display_config_section(&settings);
        self.add_line_entries(text_display_config_section_lines);
    }

    fn write_to_config(&self) {
        let hyprlock_config_file_path = HyprlandWriterUtils::create_hyprland_config_path("hyprlock.conf");
        HyprlandWriterUtils::write_content_to_file(hyprlock_config_file_path.as_str(), self.config_lines.clone());
    }
}

impl HyprlockSettingsWriter {
    pub fn new() -> Self {
        Self {
            config_lines: Vec::new()
        }
    }

    fn create_general_config_section(&self, settings: &LockScreenSettings) -> Vec<String> {
        ConfigSectionBuilder::new("general".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair(
                "hide_cursor".to_string(), settings.hide_cursor.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "grace".to_string(), settings.grace.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "fall_timeout".to_string(), settings.fall_timeout.to_string()
            ))
            .build()
    }

    fn create_background_config_section(&mut self, settings: &LockScreenSettings) -> Vec<String> {
        ConfigSectionBuilder::new("background".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair(
                "path".to_string(), settings.lockscreen_wallpaper.clone()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "blur_size".to_string(), settings.blur_size.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "blur_passes".to_string(), settings.blur_passes.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "noise".to_string(), settings.noise.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "contrast".to_string(), settings.contrast.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "brightness".to_string(), settings.brightness.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "vibrancy".to_string(), settings.vibrancy.to_string()
            ))
            .build()
    }

    fn create_password_config_section(&mut self, settings: &LockScreenSettings) -> Vec<String> {
        ConfigSectionBuilder::new("input-field".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair(
                "size".to_string(),
                format!("{}, {}",
                    settings.input_width, settings.input_height
                )
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "outline_thickness".to_string(), settings.grace.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "dots_size".to_string(), settings.input_dots_size.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "dots_spacing".to_string(), settings.input_dots_spacing.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "dots_center".to_string(), settings.input_dots_center.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "outer_color".to_string(), settings.input_outer_color.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "inner_color".to_string(), settings.input_inner_color.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "font_color".to_string(), settings.input_font_color.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "placeholder_text".to_string(), settings.input_placeholder_text.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "hide_input".to_string(), settings.hide_input.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "position".to_string(),
                format!("{}, {}",
                    settings.input_x_position, settings.input_y_position
                )
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "halign".to_string(), settings.input_horizontal_alignment.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "valign".to_string(), settings.input_vertical_alignment.to_string()
            ))
            .build()
    }

    fn create_text_display_config_section(&mut self, settings: &LockScreenSettings) -> Vec<String> {
        ConfigSectionBuilder::new("label".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair(
                "text".to_string(), settings.display_text.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "color".to_string(), settings.display_text_color.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "font_size".to_string(), settings.display_text_font_size.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "font_family".to_string(), settings.display_text_font.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "position".to_string(),
                format!("{}, {}",
                    settings.display_text_x_position, settings.display_text_y_position
                )
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "halign".to_string(), settings.display_text_horizontal_alignment.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "valign".to_string(), settings.display_text_vertical_alignment.to_string()
            ))
            .build()
    }

    fn add_line_entries(&mut self, lines: Vec<String>) {
        for line in lines {
            self.add_line_entry(line);
        }
    }

    fn add_line_entry(&mut self, text: String) {
        self.config_lines.push(text);
    }
}