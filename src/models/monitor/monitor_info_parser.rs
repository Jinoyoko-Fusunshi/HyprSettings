// This parser parses the output of the stdout output from the wlr-randr cli tool.
// Based on the output, the monitor information will be estimated, since no core packages exist to easily
// retrieve monitor information in wayland as in of July 2025.

use crate::models::monitor::monitor_information::MonitorInformation;
use crate::models::monitor::video_mode::VideoMode;

#[derive(Debug)]
pub struct MonitorInfoParser {
    monitor_infos: Vec<MonitorInformation>,
}

impl MonitorInfoParser {
    pub fn new() -> Self {
        MonitorInfoParser {
            monitor_infos: vec!()
        }
    }

    pub fn parse_output(&mut self, wlrrandr_output: &String) {
        if self.monitor_infos.len() == 0 {
            self.monitor_infos = vec!();
        }

        let mut multi_video_mode = false;
        let monitor_segments = self.get_monitor_segments(wlrrandr_output);
        for monitor_segment in monitor_segments {
            let mut port_name = String::new();
            let mut brand_name = String::new();
            let mut model_name = String::new();
            let mut serial_number = String::new();
            let mut monitor_video_modes: Vec<VideoMode> = vec!();

            let mut line_index = 0;
            for line in &monitor_segment {
                if line_index == 0 {
                    let split_monitor_name = line
                        .split(" ")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();

                    port_name = split_monitor_name[0].clone();

                    line_index += 1;
                    continue;
                }

                if line.contains("Make:") {
                    brand_name = self.parse_field(String::from("Make: "), &line);
                }

                if line.contains("Model:") {
                    model_name = self.parse_field(String::from("Model: "), &line);
                }

                if line.contains("Serial:") {
                    serial_number = self.parse_field(String::from("Serial: "), &line);
                }

                if line.contains("Modes:") {
                    multi_video_mode = true;
                    continue;
                }

                if multi_video_mode {
                    if !line.starts_with("    ") {
                        multi_video_mode = false;
                        continue;
                    }

                    let video_mode = self.parse_video_mode(&line);
                    monitor_video_modes.push(video_mode);
                }

                line_index += 1
            }

            monitor_video_modes.sort_by(|first_mode, second_mode| {
                let first_screen_area = first_mode.width_resolution * first_mode.height_resolution;
                let second_screen_area = second_mode.width_resolution * second_mode.height_resolution;

                second_screen_area.cmp(&first_screen_area)
                    .then(second_mode.refresh_rate.cmp(&first_mode.refresh_rate))
            });

            let monitor_information = MonitorInformation {
                port_name,
                brand_name,
                model_name,
                serial_number,
                min_video_mode: monitor_video_modes[monitor_video_modes.len() -1].clone(),
                max_video_mode: monitor_video_modes[0].clone(),
            };

            self.monitor_infos.push(monitor_information);
        }
    }

    pub fn get_result(&self) -> &Vec<MonitorInformation> {
        &self.monitor_infos
    }

    fn get_monitor_segments(&self, wlrrandr_output: &String) -> Vec<Vec<String>> {
        let mut last_line_index: u32 = 0;
        let mut line_index: u32 = 0;
        let mut monitor_segments_output: Vec<Vec<String>> = vec!();

        for line in wlrrandr_output.lines() {
            let leading_line_character = line.chars().nth(0).unwrap();
            if leading_line_character != ' ' && line_index > 0 || line_index == (wlrrandr_output.lines().count() - 1) as u32 {
                let monitor_segment_lines: Vec<String> = wlrrandr_output
                    .lines()
                    .skip(last_line_index as usize)
                    .take((line_index - last_line_index) as usize)
                    .map(|line | line.to_string())
                    .collect();

                monitor_segments_output.push(monitor_segment_lines);
                last_line_index = line_index;
            }

            line_index += 1;
        }


        monitor_segments_output
    }

    fn parse_field(&self, field_name: String, line_content: &String) -> String {
        const VALUE_START_OFFSET: usize = 2;
        
        let field_name_start_index = line_content.find(line_content.as_str())
            .expect("Cannot find field name start index");
        let field_name_end_index = field_name_start_index + field_name.len();
        let value_start_index = field_name_end_index + VALUE_START_OFFSET;
        line_content[value_start_index..line_content.len()].to_string()
    }

    fn parse_video_mode(&self, line_content: &String) -> VideoMode {
        const TWO_TAB_SPACES: usize = 4;
        const RESOLUTION_SPLIT_INDEX: usize = 0;
        const REFRESH_RATE_SPLIT_INDEX: usize = 2;
        const WIDTH_RESOLUTION_SPLIT_INDEX: usize = 0;
        const HEIGHT_RESOLUTION_SPLIT_INDEX: usize = 1;

        let split_mode_entry = line_content
            .split(" ")
            .map(|x| x.to_string())
            .skip(TWO_TAB_SPACES)
            .collect::<Vec<String>>();

        let frequency_part = &split_mode_entry[REFRESH_RATE_SPLIT_INDEX];
        let resolution_part = &split_mode_entry[RESOLUTION_SPLIT_INDEX];
        let split_resolution_part = resolution_part
            .split("x")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let max_width_resolution = &split_resolution_part[WIDTH_RESOLUTION_SPLIT_INDEX];
        let max_height_resolution = &split_resolution_part[HEIGHT_RESOLUTION_SPLIT_INDEX];
        let max_refresh_rate = frequency_part;

        let parsed_max_width_resolution = max_width_resolution.parse::<u32>().unwrap_or_else(|_| 0);
        let parsed_max_height_resolution = max_height_resolution.parse::<u32>().unwrap_or_else(|_| 0);
        let parsed_max_refresh_rate_fixed = max_refresh_rate.parse::<f32>().unwrap_or_else(|_| 0.0);
        let fixed_max_refresh_rate = parsed_max_refresh_rate_fixed.round() as u32;

        VideoMode {
            width_resolution: parsed_max_width_resolution, 
            height_resolution: parsed_max_height_resolution, 
            refresh_rate: fixed_max_refresh_rate,
        }
    }
}