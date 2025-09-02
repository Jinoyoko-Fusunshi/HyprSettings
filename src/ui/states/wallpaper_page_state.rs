use crate::models::modules::HYPRPAPER_MODULE;
use crate::providers::application_provider::ApplicationProvider;

#[derive(Clone)]
pub struct WallpaperPageState {
    pub enabled: bool,
}

impl From<&ApplicationProvider> for WallpaperPageState {
    fn from(value: &ApplicationProvider) -> Self {
        let module_provider = value.get_module_provider();
        let has_hyprpaper = module_provider.borrow()
            .get_module(HYPRPAPER_MODULE.to_string())
            .is_some();

        Self {
            enabled: has_hyprpaper,
        }
    }
}