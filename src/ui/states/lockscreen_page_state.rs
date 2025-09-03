use crate::models::modules::HYPRLOCK_MODULE;
use crate::models::rgba_color::RGBAColor;
use crate::providers::application_provider::ApplicationProvider;

#[derive(Clone)]
pub struct LockScreenPageState {
    pub enabled: bool,
    pub hide_cursor: bool,
    pub grace: f32,
    pub fall_timeout: u32,
    pub lockscreen_wallpaper: Option<String>,
    pub blur_size: u32,
    pub blur_passes: u32,
    pub noise: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub vibrancy: f32,
    pub input_width: u32,
    pub input_height: u32,
    pub input_outline_thickness: u32,
    pub input_dots_size: u32,
    pub input_dots_spacing: u32,
    pub input_dots_center: bool,
    pub input_outer_color: RGBAColor,
    pub input_inner_color: RGBAColor,
    pub input_font_color: RGBAColor,
    pub input_placeholder_text: Option<String>,
    pub hide_input: bool,
    pub input_x_position: u32,
    pub input_y_position: u32,
    pub input_vertical_alignment: Option<String>,
    pub input_horizontal_alignment: Option<String>,
    pub display_text: Option<String>,
    pub display_text_color: RGBAColor,
    pub display_text_font_size: u32,
    pub display_text_font: Option<String>,
    pub display_text_x_position: u32,
    pub display_text_y_position: u32,
    pub display_text_vertical_alignment: Option<String>,
    pub display_text_horizontal_alignment: Option<String>,
}

impl From<&ApplicationProvider> for LockScreenPageState {
    fn from(value: &ApplicationProvider) -> Self {
        let module_provider = value.get_module_provider();
        let has_hyprlock = module_provider.borrow()
            .get_module(HYPRLOCK_MODULE.to_string())
            .is_some();

        let settings_provider = value.get_settings_provider();
        let settings_ref = settings_provider.borrow();

        Self {
            enabled: has_hyprlock,
            hide_cursor: settings_ref.get_hide_cursor(),
            grace: settings_ref.get_grace(),
            fall_timeout: settings_ref.get_fall_timeout(),
            lockscreen_wallpaper: settings_ref.get_lockscreen_wallpaper(),
            blur_size: settings_ref.get_lockscreen_blur_size(),
            blur_passes: settings_ref.get_lockscreen_blur_passes(),
            noise: settings_ref.get_noise(),
            contrast: settings_ref.get_contrast(),
            brightness: settings_ref.get_brightness(),
            vibrancy: settings_ref.get_vibrancy(),
            input_width: settings_ref.get_input_width(),
            input_height: settings_ref.get_input_height(),
            input_outline_thickness: settings_ref.get_input_outline_thickness(),
            input_dots_size: settings_ref.get_input_dots_size(),
            input_dots_spacing: settings_ref.get_input_dots_spacing(),
            input_dots_center: settings_ref.get_input_dots_center(),
            input_outer_color: settings_ref.get_input_outer_color(),
            input_inner_color: settings_ref.get_input_inner_color(),
            input_font_color: settings_ref.get_input_font_color(),
            input_placeholder_text: settings_ref.get_input_placeholder_text(),
            hide_input: settings_ref.get_hide_input(),
            input_x_position: settings_ref.get_input_x_position(),
            input_y_position: settings_ref.get_input_y_position(),
            input_vertical_alignment: settings_ref.get_input_vertical_alignment(),
            input_horizontal_alignment: settings_ref.get_input_horizontal_alignment(),
            display_text: settings_ref.get_display_text(),
            display_text_color: settings_ref.get_display_text_color(),
            display_text_font_size: settings_ref.get_display_text_font_size(),
            display_text_font: settings_ref.get_display_text_font(),
            display_text_x_position: settings_ref.get_display_text_x_position(),
            display_text_y_position: settings_ref.get_display_text_y_position(),
            display_text_vertical_alignment: settings_ref.get_display_text_vertical_alignment(),
            display_text_horizontal_alignment: settings_ref.get_display_text_horizontal_alignment(),
        }
    }
}