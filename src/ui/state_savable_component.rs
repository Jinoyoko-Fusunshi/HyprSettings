use crate::providers::application_provider::ApplicationProvider;

pub trait StateSavableComponent {
    fn save_settings(&self, application_provider: ApplicationProvider);
    fn remove_settings(&self, application_provider: ApplicationProvider);
}