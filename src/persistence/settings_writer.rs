pub trait SettingsWriter<Settings> {
    fn serialize_settings(&mut self, settings: Settings);
    fn write_to_config(&self);
}