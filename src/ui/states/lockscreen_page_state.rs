use crate::models::modules::HYPRLOCK_MODULE;
use crate::providers::application_provider::ApplicationProvider;

#[derive(Clone)]
pub struct LockScreenPageState {
    pub enabled: bool,
}

impl From<&ApplicationProvider> for LockScreenPageState {
    fn from(value: &ApplicationProvider) -> Self {
        let module_provider = value.get_module_provider();
        let has_hyprlock = module_provider.borrow()
            .get_module(HYPRLOCK_MODULE.to_string())
            .is_some();

        Self {
            enabled: has_hyprlock
        }
    }
}