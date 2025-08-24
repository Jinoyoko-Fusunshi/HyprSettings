pub trait SettingsReader<Config> {
    fn read_from_config(&mut self);
    fn deserialize_settings(&mut self) -> Config;
}