use gtk::{Entry, Orientation};
use gtk::prelude::{BoxExt, EditableExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::boxes::Boxes;
use crate::ui::controls::Control;
use crate::ui::controls::input_field::InputField;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::statable_control::StatableControl;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::states::wallpaper_page_state::WallpaperPageState;
use crate::ui::updatable_control::UpdatableControl;

pub struct WallpaperSettings {
    application_provider: ApplicationProvider,
    state: WallpaperPageState,
    wallpaper_box: gtk::Box,
    wallpaper_sections_box: gtk::Box,
}

impl Control for WallpaperSettings {
    fn init_events(&self) {

    }

    fn get_widget(&self) -> &gtk::Box {
        &self.wallpaper_box
    }
}

impl UpdatableControl<WallpaperPageState> for WallpaperSettings {
    fn update_ui(&mut self, state: WallpaperPageState) {
        Boxes::clear_box_content(&self.wallpaper_sections_box);

        if state.enabled {
            self.create_wallpaper_sections();
        } else {
            self.create_wallpaper_warning()
        }
    }
}

impl StatableControl<WallpaperPageState> for WallpaperSettings {
    fn update_state(&mut self, state: WallpaperPageState) {
        self.state = state;
    }
}

impl WallpaperSettings {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        const WALLPAPER_TITLE: &str = "Wallpaper";
        let wallpaper_box = SectionBoxBuilder::new()
            .create_header_elements(WALLPAPER_TITLE)
            .build().expect("Failed to create wallpaper section box");
        Boxes::set_margin(&wallpaper_box, 10);

        let wallpaper_sections_box = gtk::Box::new(Orientation::Vertical, 10);
        wallpaper_box.append(&wallpaper_sections_box);

        let state = WallpaperPageState {
            enabled: true,
        };

        Self {
            application_provider,
            state,
            wallpaper_box,
            wallpaper_sections_box,
        }
    }

    fn create_wallpaper_sections(&self) {
        let settings_provider = self.application_provider
            .get_settings_provider();
        let settings_provider_clone = settings_provider.clone();
        let wallpaper_path_input_change = {
            move |entry: &Entry| {
                settings_provider_clone.borrow_mut().set_wallpaper_path(entry.text().to_string());
            }
        };

        let mut wallpaper_path_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "wallpaper path:".to_string(),
            placeholder_text: "e.g. ~/Pictures/wallpaper.png".to_string(),
            entry_text: Some(settings_provider.borrow().get_wallpaper_path()),
        };
        wallpaper_path_input_field.update_ui(state);
        wallpaper_path_input_field.set_input_callback(wallpaper_path_input_change);

        self.wallpaper_sections_box.append(wallpaper_path_input_field.get_widget());
    }

    fn create_wallpaper_warning(&self) {
        let wallpaper_warning = Boxes::create_warning_box(
            "⚠️ Hyprpaper program module was not found. This is required to configure the wallpaper settings."
        );
        self.wallpaper_sections_box.append(&wallpaper_warning);
    }
}