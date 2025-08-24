use std::cell::RefCell;
use std::rc::Rc;
use crate::settings::config_files::settings_writer::SettingsWriter;
use crate::settings::hyprland_settings::HyprlandSettings;

const COMMENT_CHARACTER: char = '#';

pub struct HyprlandSettingsWriter {
    settings: Rc<RefCell<HyprlandSettings>>,
    config_lines: Vec<String>
}

impl SettingsWriter<HyprlandSettings> for HyprlandSettingsWriter {
    fn serialize_settings(&mut self, _: HyprlandSettings) {
        self.serialize_program_settings();
        self.serialize_appearance_settings();
        self.serialize_monitor_settings();
    }

    fn write_to_config(&self) {
        if self.config_lines.len() == 0 {
            return;
        }
    }
}

impl HyprlandSettingsWriter {
    pub fn new(settings: Rc<RefCell<HyprlandSettings>>) -> Self {
        Self {
            settings,
            config_lines: Vec::new()
        }
    }

    fn serialize_program_settings(&self) {

    }

    fn serialize_appearance_settings(&self) {

    }

    fn serialize_monitor_settings(&self) {

    }

    fn create_section_comment(section_name: String) {
        const SPACE_BETWEEN_NAME_AND_COMMENT_CHARACTERS: usize = 1;
        const MIN_SECTION_COMMENT_CHARACTER_COUNT: usize = 3;

        let section_name_length = section_name.len();
        let section_comment_character_count = (MIN_SECTION_COMMENT_CHARACTER_COUNT * 2)
            + (SPACE_BETWEEN_NAME_AND_COMMENT_CHARACTERS * 2) + section_name_length;

        Self::create_comment_row(section_comment_character_count);

        let mut section_name_comment = String::new();
        for _ in 0..MIN_SECTION_COMMENT_CHARACTER_COUNT {
            section_name_comment.push(COMMENT_CHARACTER);
        }

        section_name_comment.push(' ');
        section_name_comment.push_str(&section_name);
        section_name_comment.push(' ');

        for _ in 0..MIN_SECTION_COMMENT_CHARACTER_COUNT {
            section_name_comment.push(COMMENT_CHARACTER);
        }

        Self::create_comment_row(section_comment_character_count);
    }

    fn create_comment_row(count: usize) -> String {
        let mut comment_row = String::new();
        for _ in 0..count {
            comment_row.push(COMMENT_CHARACTER);
        }

        comment_row
    }
}