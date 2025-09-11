use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub const COMMENT_CHARACTER: char = '#';

pub struct ConfigSectionBuilder {
    section_title: String,
    section_lines: Vec<String>
}

impl ConfigSectionBuilder {
    pub fn new(section_title: String) -> Self {
        Self {
            section_title,
            section_lines: Vec::new()
        }
    }

    pub fn add_lines(&mut self, lines: Vec<String>) -> &mut Self {
        for line in lines {
            self.add_line(line);
        }
        self
    }

    pub fn add_line(&mut self, line: String) -> &mut Self {
        self.section_lines.push(line);
        self
    }

    pub fn build(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("{} {{", self.section_title));

        for line in self.section_lines.clone() {
            lines.push(format!("\t{}", line));
        }

        lines.push("}".to_string());
        lines
    }
}

pub struct HyprlandWriterUtils;

impl HyprlandWriterUtils {
    pub fn create_new_line() -> String {
        "".to_string()
    }

    pub fn create_value_pair(name: String, value: String) -> String {
        format!("{} = {}", name, value)
    }

    pub fn create_comment(text: &str) -> String {
        let mut comment = String::new();
        comment.push(COMMENT_CHARACTER);
        comment.push_str(" ");
        comment.push_str(text);
        comment
    }
    
    pub fn write_content_to_file(file_path: &str, config_lines: Vec<String>) {
        if config_lines.len() == 0 {
            return;
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
            .expect("Cannot open hyprland config file");
        let mut buffer_writer = BufWriter::new(file);

        let file_content = config_lines.join("\n");
        buffer_writer.write_all(file_content.as_bytes()).expect("Cannot write to config file");
    }
}