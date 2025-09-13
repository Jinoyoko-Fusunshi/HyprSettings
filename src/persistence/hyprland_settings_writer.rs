use std::collections::HashMap;
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::models::keybinds::system_keybind::SystemKeybind;
use crate::models::monitor::monitor_configuration::MonitorOrientation;
use crate::persistence::settings_writer::SettingsWriter;
use crate::models::settings::hyprland_settings::HyprlandSettings;
use crate::models::settings::keybind_settings::KeyBindSettings;
use crate::persistence::hyprland_writer_utils::{ConfigSectionBuilder, HyprlandWriterUtils, COMMENT_CHARACTER};
use crate::providers::module_provider::{
    FILE_MANAGER_ENTRY, NOTIFICATION_HANDLER_ENTRY, QUICK_SEARCH_ENTRY, VIRTUAL_TERMINAL_ENTRY
};

pub struct HyprlandSettingsWriter {
    program_variables: HashMap<String, (String, String)>,
    config_lines: Vec<String>
}

impl SettingsWriter<HyprlandSettings> for HyprlandSettingsWriter {
    fn serialize_settings(&mut self, settings: HyprlandSettings) {
        self.index_program_variables(&settings);

        self.serialize_program_settings();
        self.serialize_startup_settings(&settings);
        self.serialize_monitor_settings(&settings);
        self.serialize_appearance_settings(&settings);
        self.serialize_input_settings(&settings);
        self.serialize_keybinds_settings(&settings);
    }

    fn write_to_config(&self) {
        let hyprland_config_file_path = HyprlandWriterUtils::create_hyprland_config_path("hyprland.conf");
        HyprlandWriterUtils::write_content_to_file(hyprland_config_file_path.as_str(), self.config_lines.clone())
    }
}

impl HyprlandSettingsWriter {
    pub fn new() -> Self {
        Self {
            program_variables: HashMap::new(),
            config_lines: Vec::new(),
        }
    }

    fn index_program_variables(&mut self, settings: &HyprlandSettings) {
        let default_value = "".to_string();
        let program_settings = &settings.program_settings.programs;
        let terminal_value = program_settings.get(VIRTUAL_TERMINAL_ENTRY).unwrap_or(&default_value);
        let file_manager_value = program_settings.get(FILE_MANAGER_ENTRY).unwrap_or(&default_value);
        let quick_search_value = program_settings.get(QUICK_SEARCH_ENTRY).unwrap_or(&default_value);
        let notifications_value = program_settings.get(NOTIFICATION_HANDLER_ENTRY).unwrap_or(&default_value);

        self.program_variables.insert(VIRTUAL_TERMINAL_ENTRY.to_string(), ("$terminal".to_string(), terminal_value.clone()));
        self.program_variables.insert(FILE_MANAGER_ENTRY.to_string(), ("$fileManager".to_string(), file_manager_value.clone()));
        self.program_variables.insert(QUICK_SEARCH_ENTRY.to_string(), ("$quickSearch".to_string(), quick_search_value.clone()));
        self.program_variables.insert(NOTIFICATION_HANDLER_ENTRY.to_string(), ("$notifications".to_string(), notifications_value.clone()));
    }

    fn serialize_program_settings(&mut self) {
        self.add_comment_section("PROGRAMS".to_string());

        self.add_line_entry(HyprlandWriterUtils::create_comment("Set programs that you use"));
        for (_, program_pair) in self.program_variables.clone() {
            let (program_variable, program_command) = program_pair;
            let program_pair = HyprlandWriterUtils::create_value_pair(program_variable, program_command);
            self.add_line_entry(program_pair);
        }
    }

    fn serialize_startup_settings(&mut self, settings: &HyprlandSettings) {
        self.add_comment_section("AUTOSTART".to_string());

        let startup_programs = settings.program_settings.startup_programs.clone();
        for (program_name, program_command) in startup_programs {
            let execution_command = if let Some(program_pair) = self.program_variables.get(&program_name) {
                let (program_variable, _) = program_pair;
                program_variable.clone()
            } else {
                program_command.clone()
            };

            let startup_entry = format!("exec-once = {}", execution_command);
            self.add_line_entry(startup_entry);
        }
    }

    fn serialize_monitor_settings(&mut self, settings: &HyprlandSettings) {
        self.add_comment_section("MONITORS".to_string());

        let display_settings = settings.display_settings.monitor_configurations.clone();
        for (monitor_port, monitor_configuration) in display_settings {
            if !monitor_configuration.enabled {
                continue;
            }

            let video_mode = monitor_configuration.video_mode;
            let transformation_settings = match monitor_configuration.orientation {
                MonitorOrientation::None => 0,
                MonitorOrientation::Rotation90 => 1,
                MonitorOrientation::Rotation180 => 2,
                MonitorOrientation::Rotation270 => 3,
                MonitorOrientation::Flipped => 4,
                MonitorOrientation::FlippedRotation90 => 5,
                MonitorOrientation::FlippedRotation180 => 6,
                MonitorOrientation::FlippedRotation270 => 7,
            };

            let display_entry = format!(
                "monitor = {}, {}x{}@{}, {}x{}, {}, transform, {}",
                monitor_port,
                video_mode.width_resolution,
                video_mode.height_resolution,
                video_mode.refresh_rate,
                monitor_configuration.x_offset,
                monitor_configuration.y_offset,
                monitor_configuration.resolution_scale,
                transformation_settings
            );
            self.add_line_entry(display_entry);
        }
    }

    fn serialize_appearance_settings(&mut self, settings: &HyprlandSettings) {
        let cursor_section_lines = Self::create_cursor_config_section(settings);
        let general_section_lines = Self::create_general_config_section(settings);
        let decoration_section_lines = Self::create_decorations_config_section(settings);
        let dwindle_section_lines = Self::create_dwindle_config_section(settings);
        let master_section_lines = Self::create_master_config_section(settings);
        let misc_section_lines = Self::create_misc_config_section(settings);

        self.add_comment_section("LOOK AND FEEL".to_string());
        self.add_line_entries(cursor_section_lines);
        self.add_new_line();
        self.add_line_entries(general_section_lines);
        self.add_new_line();
        self.add_line_entries(decoration_section_lines);
        self.add_new_line();
        self.add_line_entries(dwindle_section_lines);
        self.add_new_line();
        self.add_line_entries(master_section_lines);
        self.add_new_line();
        self.add_line_entries(misc_section_lines);
    }

    fn serialize_input_settings(&mut self, settings: &HyprlandSettings) {
        let input_settings = &settings.input_settings;

        let input_section_lines = ConfigSectionBuilder::new("input".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair(
                "kb_layout".to_string(), input_settings.keyboard_layout.clone()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "numlock_by_default".to_string(), input_settings.numlock_enabled.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "repeat_rate".to_string(), input_settings.keyboard_repeat_rate.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "repeat_delay".to_string(), input_settings.keyboard_repeat_delay.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "sensitivity".to_string(), input_settings.mouse_sensitivity.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "left_handed".to_string(), input_settings.mouse_left_handed.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "scroll_factor".to_string(), input_settings.mouse_scroll_factor.to_string()
            ))
            .add_line(HyprlandWriterUtils::create_value_pair(
                "natural_scroll".to_string(), input_settings.mouse_natural_scroll.to_string()
            ))
            .build();

        self.add_comment_section("INPUT".to_string());
        self.add_new_line();
        self.add_line_entries(input_section_lines);
    }

    fn serialize_keybinds_settings(&mut self, settings: &HyprlandSettings) {
        let keybinds_settings = &settings.keybind_settings;

        self.add_comment_section("KEYBINDINGS".to_string());
        self.create_program_keybind_section(
            keybinds_settings, VIRTUAL_TERMINAL_ENTRY, SystemKeybind::Terminal);
        self.create_program_keybind_section(
            keybinds_settings, FILE_MANAGER_ENTRY, SystemKeybind::FileManager);
        self.create_program_keybind_section(
            keybinds_settings, QUICK_SEARCH_ENTRY, SystemKeybind::RunProgram);
        self.add_new_line();

        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::CloseWindow, vec!["killactive"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::ExitHyprland, vec!["exit"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::ToggleFloatingWindow, vec!["togglefloating"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::Pseudo, vec!["pseudo"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SplitWindow, vec!["togglesplit"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::FocusLeftWindow, vec!["movefocus", "l"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::FocusRightWindow, vec!["movefocus", "r"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::FocusTopWindow, vec!["movefocus", "u"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::FocusBottomWindow, vec!["movefocus", "d"]
        );
        self.add_new_line();

        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceOne, vec!["workspace", "1"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceTwo, vec!["workspace", "2"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceThree, vec!["workspace", "3"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceFour, vec!["workspace", "4"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceFive, vec!["workspace", "5"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceSix, vec!["workspace", "6"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceSeven, vec!["workspace", "7"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceEight, vec!["workspace", "8"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceNine, vec!["workspace", "9"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::SwitchWorkspaceZero, vec!["workspace", "0"]
        );
        self.add_new_line();

        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceOne, vec!["movetoworkspace", "1"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceTwo, vec!["movetoworkspace", "2"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceThree, vec!["movetoworkspace", "3"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceFour, vec!["movetoworkspace", "4"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceFive, vec!["movetoworkspace", "5"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceSix, vec!["movetoworkspace", "6"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceSeven, vec!["movetoworkspace", "7"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceEight, vec!["movetoworkspace", "8"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceNine, vec!["movetoworkspace", "9"]
        );
        self.create_action_keybind_section(
            keybinds_settings, SystemKeybind::MoveWorkspaceZero, vec!["movetoworkspace", "0"]
        );
    }

    fn create_program_keybind_section(&mut self, settings: &KeyBindSettings, program_name: &str, system_keybind: SystemKeybind) {
        let program_keybind = settings.program_keybinds.get(&system_keybind);
        if let Some(keybind) = program_keybind {
            let program_pair = self.program_variables.get(program_name);
            if let Some((program_variable, _)) = program_pair {
                let arguments = vec!["exec", program_variable.as_str()];
                self.add_line_entry(Self::create_keybind_entry(keybind.clone(), arguments));
            }
        }
    }

    fn create_action_keybind_section(&mut self, settings: &KeyBindSettings, system_keybind: SystemKeybind, arguments: Vec<&str>) {
        if let Some(keybind) = settings.program_keybinds.get(&system_keybind) {
            self.add_line_entry(Self::create_keybind_entry(keybind.clone(), arguments));
        }
    }

    fn create_keybind_entry(keybind_configuration: KeyBindConfiguration, arguments: Vec<&str>) -> String {
        let key_modifiers = keybind_configuration.get_modifier_keys().join(" ");
        let mut key_arguments = key_modifiers;
        if let Some(key) = keybind_configuration.get_key() {
            key_arguments.push_str(format!(", {}", key).as_str());
        }
        
        let command_arguments = arguments.join(", ");
        format!("bind = {}, {}", key_arguments, command_arguments)
    }

    fn create_cursor_config_section(settings: &HyprlandSettings) -> Vec<String> {
        let cursor_size_value = settings.appearance_settings.cursor_size.to_string();
        let cursor_theme_value = settings.appearance_settings.cursor_theme.clone();

        vec![
            HyprlandWriterUtils::create_environment_variable("XCURSOR_SIZE".to_string(), cursor_size_value),
            HyprlandWriterUtils::create_environment_variable("XCURSOR_THEME".to_string(), cursor_theme_value),
        ]
    }

    fn create_general_config_section(settings: &HyprlandSettings) -> Vec<String> {
        let gaps_in_value = settings.appearance_settings.inner_gab.to_string();
        let gaps_out_value = settings.appearance_settings.outer_gab.to_string();
        let border_size_value = settings.appearance_settings.border_size.to_string();
        let column_active_border_value = settings.appearance_settings.active_border_color.to_string();
        let column_inactive_border_value = settings.appearance_settings.inactive_border_color.to_string();
        let resize_on_border_value = settings.appearance_settings.resize_on_border.to_string();
        let allow_tearing_value = settings.appearance_settings.allow_tearing.to_string();
        let layout_value = settings.appearance_settings.layout.to_string();

        let general_section_lines = ConfigSectionBuilder::new("general".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair("gaps_in".to_string(), gaps_in_value))
            .add_line(HyprlandWriterUtils::create_value_pair("gaps_out".to_string(), gaps_out_value))
            .add_line(HyprlandWriterUtils::create_value_pair("border_size".to_string(), border_size_value))
            .add_line(HyprlandWriterUtils::create_value_pair("col.active_border".to_string(), column_active_border_value))
            .add_line(HyprlandWriterUtils::create_value_pair("col.inactive_border".to_string(), column_inactive_border_value))
            .add_line(HyprlandWriterUtils::create_value_pair("resize_on_border".to_string(), resize_on_border_value))
            .add_line(HyprlandWriterUtils::create_value_pair("allow_tearing".to_string(), allow_tearing_value))
            .add_line(HyprlandWriterUtils::create_value_pair("layout".to_string(), layout_value))
            .build();
        general_section_lines
    }

    fn create_decorations_config_section(settings: &HyprlandSettings) -> Vec<String> {
        let rounding_value = settings.appearance_settings.rounding.to_string();
        let rounding_power_value = settings.appearance_settings.rounding_power.to_string();
        let dim_inactive = settings.appearance_settings.dim_inactive;
        let active_dim_opacity = settings.appearance_settings.active_opacity.to_string();
        let inactive_dim_opacity = settings.appearance_settings.inactive_opacity.to_string();

        let decoration_section_lines = ConfigSectionBuilder::new("decoration".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair("rounding".to_string(), rounding_value))
            .add_line(HyprlandWriterUtils::create_value_pair("rounding_power".to_string(), rounding_power_value))
            .add_line(HyprlandWriterUtils::create_value_pair("dim_inactive".to_string(), dim_inactive.to_string()))
            .add_line(HyprlandWriterUtils::create_value_pair("active_opacity".to_string(), active_dim_opacity))
            .add_line(HyprlandWriterUtils::create_value_pair("inactive_opacity".to_string(), inactive_dim_opacity))
            .add_line(HyprlandWriterUtils::create_new_line())
            .add_lines(Self::create_shadow_config_section(settings))
            .add_line(HyprlandWriterUtils::create_new_line())
            .add_lines(Self::create_blur_config_section(settings))
            .build();
        decoration_section_lines
    }

    fn create_shadow_config_section(settings: &HyprlandSettings) -> Vec<String> {
        let shadows_enabled = settings.appearance_settings.active_shadow.to_string();
        let shadow_range = settings.appearance_settings.shadow_range.to_string();
        let shadow_render_power = settings.appearance_settings.shadow_render_power.to_string();
        let shadow_color = settings.appearance_settings.shadow_color.to_string();

        let shadow_section_lines = ConfigSectionBuilder::new("shadow".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair("enabled".to_string(), shadows_enabled))
            .add_line(HyprlandWriterUtils::create_value_pair("range".to_string(), shadow_range))
            .add_line(HyprlandWriterUtils::create_value_pair("render_power".to_string(), shadow_render_power))
            .add_line(HyprlandWriterUtils::create_value_pair("color".to_string(), shadow_color))
            .build();
        shadow_section_lines
    }

    fn create_blur_config_section(settings: &HyprlandSettings) -> Vec<String> {
        let blur_enabled = settings.appearance_settings.active_blur.to_string();
        let blur_size = settings.appearance_settings.blur_size.to_string();
        let blur_passed = settings.appearance_settings.blur_passes.to_string();
        let blur_vibrancy = settings.appearance_settings.blur_vibrancy.to_string();

        let blur_section_lines = ConfigSectionBuilder::new("blur".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair("enabled".to_string(), blur_enabled))
            .add_line(HyprlandWriterUtils::create_value_pair("size".to_string(), blur_size))
            .add_line(HyprlandWriterUtils::create_value_pair("passes".to_string(), blur_passed))
            .add_line(HyprlandWriterUtils::create_value_pair("vibrancy".to_string(), blur_vibrancy))
            .build();
        blur_section_lines
    }

    fn create_dwindle_config_section(settings: &HyprlandSettings) -> Vec<String> {
        let pseudo_tiling_value = settings.appearance_settings.pseudo_tiling.to_string();
        let preserve_split_value = settings.appearance_settings.split_preservation.to_string();

        let dwindle_status_section_lines = ConfigSectionBuilder::new("dwindle".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair("pseudotile".to_string(), pseudo_tiling_value))
            .add_line(HyprlandWriterUtils::create_value_pair("preserve_split".to_string(), preserve_split_value))
            .build();

        dwindle_status_section_lines
    }

    fn create_master_config_section(settings: &HyprlandSettings) -> Vec<String> {
        let master_status_value = settings.appearance_settings.master_status.to_string();
        let master_status_section_lines = ConfigSectionBuilder::new("master".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair("new_status".to_string(), master_status_value))
            .build();

        master_status_section_lines
    }

    fn create_misc_config_section(settings: &HyprlandSettings) -> Vec<String> {
        let force_default_wallpaper_value = settings.appearance_settings.force_default_wallpaper.to_string();
        let disable_hyprland_logo_value = settings.appearance_settings.disable_hyprland_logo.to_string();

        let misc_section_lines = ConfigSectionBuilder::new("misc".to_string())
            .add_line(HyprlandWriterUtils::create_value_pair("force_default_wallpaper".to_string(), force_default_wallpaper_value))
            .add_line(HyprlandWriterUtils::create_value_pair("disable_hyprland_logo".to_string(), disable_hyprland_logo_value))
            .build();
        misc_section_lines
    }

    fn add_new_line(&mut self) {
        self.add_line_entry(HyprlandWriterUtils::create_new_line());
    }

    fn add_line_entries(&mut self, lines: Vec<String>) {
        for line in lines {
            self.add_line_entry(line);
        }
    }

    fn add_line_entry(&mut self, text: String) {
        self.config_lines.push(text);
    }

    fn add_comment_section(&mut self, section_name: String) {
        const SPACE_BETWEEN_NAME_AND_COMMENT_CHARACTERS: usize = 1;
        const MIN_SECTION_COMMENT_CHARACTER_COUNT: usize = 3;

        self.add_new_line();

        let section_name_length = section_name.len();
        let section_comment_character_count = (MIN_SECTION_COMMENT_CHARACTER_COUNT * 2)
            + (SPACE_BETWEEN_NAME_AND_COMMENT_CHARACTERS * 2) + section_name_length;

        let comment_row = Self::create_comment_row(section_comment_character_count);
        self.add_line_entry(comment_row.clone());

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
        self.add_line_entry(section_name_comment);

        self.add_line_entry(comment_row.clone());
        self.add_new_line();
    }

    fn create_comment_row(count: usize) -> String {
        let mut comment_row = String::new();
        for _ in 0..count {
            comment_row.push(COMMENT_CHARACTER);
        }

        comment_row
    }
}