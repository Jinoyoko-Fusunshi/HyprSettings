pub trait SettingsWriter {
    fn serialize_settings(&mut self);
    fn write_to_config_file(&self) -> Result<(), String>;
}