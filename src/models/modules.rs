pub mod program_module;
pub mod program_module_info;

pub const HYPRLAND_CORE_MODULE: &str = "hyprland";
pub const HYPRPAPER_MODULE: &str = "hyprpaper";
pub const HYPRIDLE_MODULE: &str = "hypridle";
pub const HYPRLOCK_MODULE: &str = "hyprlock";
pub const HYPRPOLKIT_AGENT_MODULE: &str = "hyprpolkitagent";
pub const WAYLANDRANDR_MODULE: &str = "wlr-randr";

#[derive(Clone)]
pub enum ProgramModuleCategory {
    Hyprland,
    Dependency
}