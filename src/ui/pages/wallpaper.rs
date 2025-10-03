use gtk::{Entry, Orientation};
use gtk::prelude::{BoxExt, EditableExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::{Boxes, DEFAULT_MARGIN};
use crate::ui::controls::Control;
use crate::ui::controls::input_field::InputField;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::states::wallpaper_page_state::WallpaperPageState;
use crate::ui::updatable_control::UpdatableControl;

pub struct Wallpaper {
    application_provider: ApplicationProvider,
    state: WallpaperPageState,
    wallpaper_box: GTKBox,
    wallpaper_sections_box: GTKBox,
}

impl Control for Wallpaper {
    fn get_widget(&self) -> &GTKBox {
        &self.wallpaper_box
    }
}

impl UpdatableControl<WallpaperPageState> for Wallpaper {
    fn update_state(&mut self, state: WallpaperPageState) {
        Boxes::clear_box_content(&self.wallpaper_sections_box);

        if state.enabled {
            self.create_wallpaper_sections();
        } else {
            self.create_wallpaper_warning()
        }

        self.state = state;
    }

    fn get_current_state(&self) -> WallpaperPageState {
        self.state.clone()
    }
}

impl Wallpaper {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        const WALLPAPER_TITLE: &str = "Wallpaper";
        let wallpaper_box = SectionBoxBuilder::new("wallpaper", DEFAULT_MARGIN)
            .create_header_elements(WALLPAPER_TITLE)
            .build().expect("Failed to create wallpaper section box");

        let wallpaper_sections_box = BoxBuilder::new("wallpaper_sections")
            .set_orientation(Orientation::Vertical)
            .build();

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
        let appearance_provider = self.application_provider
            .get_appearance_provider();
        let appearance_provider_clone = appearance_provider.clone();
        let wallpaper_path_input_change = {
            move |entry: &Entry| {
                appearance_provider_clone.borrow_mut().set_wallpaper_path(entry.text().to_string());
            }
        };

        let mut wallpaper_path_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "wallpaper path:".to_string(),
            placeholder_text: "e.g. ~/Pictures/wallpaper.png".to_string(),
            entry_text: Some(appearance_provider.borrow().get_wallpaper_path()),
        };
        wallpaper_path_input_field.update_state(state);
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