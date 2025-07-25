use gtk::gdk::RGBA;

#[derive(Debug)]
pub struct AppearanceSettings {
    pub wallpaper_path: String,
    pub force_default_wallpaper: bool,
    pub disable_hyprland_logo: bool,
    pub inner_gab: f64,
    pub outer_gab: f64,
    pub border_size: f64,
    pub active_border_color: RGBA,
    pub inactive_border_color: RGBA,
    pub resize_on_border: bool,
    pub allow_tearing: bool,
    pub rounding: f64,
    pub rounding_power: f64,
    pub dim_inactive: bool,
    pub active_opacity: f64,
    pub inactive_opacity: f64,
    pub active_shadow: bool,
    pub shadow_range: f64,
    pub shadow_render_power: f64,
    pub shadow_color: RGBA,
    pub active_blur: bool,
    pub blur_size: f64,
    pub blur_passes: usize,
    pub blur_vibrancy: f64,
    pub layout: String,
    pub master_status: String,
    pub pseudo_tiling: bool,
    pub split_preservation: bool,
}

impl AppearanceSettings {
    pub fn new(
        wallpaper_path: String,
        force_default_wallpaper: bool,
        disable_hyprland_logo: bool,
        inner_gab: f64,
        outer_gab: f64,
        border_size: f64,
        active_border_color: RGBA,
        inactive_border_color: RGBA,
        resize_on_border: bool,
        allow_tearing: bool,
        rounding: f64,
        rounding_power: f64,
        dim_inactive: bool,
        active_opacity: f64,
        inactive_opacity: f64,
        active_shadow: bool,
        shadow_range: f64,
        shadow_render_power: f64,
        shadow_color: RGBA,
        active_blur: bool,
        blur_size: f64,
        blur_passes: usize,
        blur_vibrancy: f64,
        layout: String,
        master_status: String,
        pseudo_tiling: bool,
        split_preservation: bool,
    ) -> Self {
        AppearanceSettings {
            wallpaper_path,
            force_default_wallpaper,
            disable_hyprland_logo,
            inner_gab,
            outer_gab,
            border_size,
            active_border_color,
            inactive_border_color,
            resize_on_border,
            allow_tearing,
            rounding,
            rounding_power,
            dim_inactive,
            active_opacity,
            inactive_opacity,
            active_shadow,
            shadow_range,
            shadow_render_power,
            shadow_color,
            active_blur,
            blur_size,
            blur_passes,
            blur_vibrancy,
            layout,
            master_status,
            pseudo_tiling,
            split_preservation,
        }
    }
    
    pub fn set_wallpaper_path(&mut self, wallpaper_path: String) {
        self.wallpaper_path = wallpaper_path;
    }

    pub fn set_force_default_wallpaper(&mut self, force_default_wallpaper: bool) {
        self.force_default_wallpaper = force_default_wallpaper;
    }

    pub fn set_disable_hyprland_logo(&mut self, disable_hyprland_logo: bool) {
        self.disable_hyprland_logo = disable_hyprland_logo;
    }

    pub fn set_inner_gab(&mut self, inner_gab: f64) {
        self.inner_gab = inner_gab;
    }

    pub fn set_outer_gab(&mut self, outer_gab: f64) {
        self.outer_gab = outer_gab;
    }

    pub fn set_border_size(&mut self, border_size: f64) {
        self.border_size = border_size;
    }

    pub fn set_active_border_color(&mut self, active_border_color: RGBA) {
        self.active_border_color = active_border_color;
    }

    pub fn set_inactive_border_color(&mut self, inactive_border_color: RGBA) {
        self.inactive_border_color = inactive_border_color;
    }

    pub fn set_resize_on_border(&mut self, resize_on_border: bool) {
        self.resize_on_border = resize_on_border;
    }

    pub fn set_allow_tearing(&mut self, allow_tearing: bool) {
        self.allow_tearing = allow_tearing;
    }

    pub fn set_rounding(&mut self, rounding: f64) {
        self.rounding = rounding;
    }

    pub fn set_rounding_power(&mut self, rounding_power: f64) {
        self.rounding_power = rounding_power;
    }

    pub fn set_dim_inactive(&mut self, dim_inactive: bool) {
        self.dim_inactive = dim_inactive;
    }

    pub fn set_active_opacity(&mut self, active_opacity: f64) {
        self.active_opacity = active_opacity;
    }

    pub fn set_inactive_opacity(&mut self, inactive_opacity: f64) {
        self.inactive_opacity = inactive_opacity;
    }

    pub fn set_active_shadow(&mut self, active_shadow: bool) {
        self.active_shadow = active_shadow;
    }

    pub fn set_shadow_range(&mut self, shadow_range: f64) {
        self.shadow_range = shadow_range;
    }

    pub fn set_shadow_render_power(&mut self, shadow_render_power: f64) {
        self.shadow_render_power = shadow_render_power;
    }

    pub fn set_shadow_color(&mut self, shadow_color: RGBA) {
        self.shadow_color = shadow_color;
    }

    pub fn set_active_blur(&mut self, active_blur: bool) {
        self.active_blur = active_blur;
    }

    pub fn set_blur_size(&mut self, blur_size: f64) {
        self.blur_size = blur_size;
    }

    pub fn set_blur_passes(&mut self, blur_passes: usize) {
        self.blur_passes = blur_passes;
    }

    pub fn set_blur_vibrancy(&mut self, blur_vibrancy: f64) {
        self.blur_vibrancy = blur_vibrancy;
    }

    pub fn set_layout(&mut self, layout: String) {
        self.layout = layout;
    }

    pub fn set_master_status(&mut self, master_status: String) {
        self.master_status = master_status;
    }

    pub fn set_pseudo_tiling(&mut self, pseudo_tiling: bool) {
        self.pseudo_tiling = pseudo_tiling;
    }

    pub fn set_split_preservation(&mut self, split_preservation: bool) {
        self.split_preservation = split_preservation;
    }

    pub fn get_wallpaper_path(&self) -> &str {
        &self.wallpaper_path
    }

    pub fn get_force_default_wallpaper(&self) -> bool {
        self.force_default_wallpaper
    }

    pub fn get_disable_hyprland_logo(&self) -> bool {
        self.disable_hyprland_logo
    }

    pub fn get_inner_gab(&self) -> f64 {
        self.inner_gab
    }

    pub fn get_outer_gab(&self) -> f64 {
        self.outer_gab
    }

    pub fn get_border_size(&self) -> f64 {
        self.border_size
    }

    pub fn get_active_border_color(&self) -> &RGBA {
        &self.active_border_color
    }

    pub fn get_inactive_border_color(&self) -> &RGBA {
        &self.inactive_border_color
    }

    pub fn get_resize_on_border(&self) -> bool {
        self.resize_on_border
    }

    pub fn get_allow_tearing(&self) -> bool {
        self.allow_tearing
    }

    pub fn get_rounding(&self) -> f64 {
        self.rounding
    }

    pub fn get_rounding_power(&self) -> f64 {
        self.rounding_power
    }

    pub fn get_dim_inactive(&self) -> bool {
        self.dim_inactive
    }

    pub fn get_active_opacity(&self) -> f64 {
        self.active_opacity
    }

    pub fn get_inactive_opacity(&self) -> f64 {
        self.inactive_opacity
    }

    pub fn get_active_shadow(&self) -> bool {
        self.active_shadow
    }

    pub fn get_shadow_range(&self) -> f64 {
        self.shadow_range
    }

    pub fn get_shadow_render_power(&self) -> f64 {
        self.shadow_render_power
    }

    pub fn get_shadow_color(&self) -> &RGBA {
        &self.shadow_color
    }

    pub fn get_active_blur(&self) -> bool {
        self.active_blur
    }

    pub fn get_blur_size(&self) -> f64 {
        self.blur_size
    }

    pub fn get_blur_passes(&self) -> usize {
        self.blur_passes
    }

    pub fn get_blur_vibrancy(&self) -> f64 {
        self.blur_vibrancy
    }

    pub fn get_layout(&self) -> &str {
        &self.layout
    }

    pub fn get_master_status(&self) -> &str {
        &self.master_status
    }

    pub fn get_pseudo_tiling(&self) -> bool {
        self.pseudo_tiling
    }

    pub fn get_split_preservation(&self) -> bool {
        self.split_preservation
    }
}