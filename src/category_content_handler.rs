use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use gtk::Stack;
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::ui::controls::panel::{appearance_panel, display_panel, general_panel, info_panel, key_binds_panel, startup_programs_panel, Panel};

// Panel names
pub const GENERAL_PANEL_NAME: &str = "general-panel";
pub const DISPLAY_PANEL_NAME: &str = "display-panel";
pub const APPEARANCE_PANEL_NAME: &str = "appearance-panel";
pub const STARTUP_PROGRAM_PANEL_NAME: &str = "startup-panel";
pub const KEYBINDS_PANEL_NAME: &str = "keybinds-panel";
pub const INFO_PANEL_NAME: &str = "info-panel";

pub struct CategoryContentHandler {
    category_panels_map: HashMap<String, Box<dyn Panel>>,
    category_boxes_stack: Stack,
}

impl CategoryContentHandler {
    pub fn new(settings: &Rc<RefCell<HyprlandSettings>>) -> Self {
        // The category panels to be individually displayed
        let general_panel = Box::new(general_panel::GeneralPanel::new(settings));
        let display_panel = Box::new(display_panel::DisplayPanel::new(settings));
        let appearance_panel = Box::new(appearance_panel::AppearancePanel::new(settings));
        let startup_programs_panel = Box::new(startup_programs_panel::StartupProgramsPanel::new(settings));
        let keybinds_panel = Box::new(key_binds_panel::KeyBindsPanel::new(settings));
        let info_panel = Box::new(info_panel::InfoPanel::new());

        let mut category_panels_map: HashMap<String, Box<dyn Panel>> = HashMap::new();

        category_panels_map.insert(GENERAL_PANEL_NAME.to_string(), general_panel.clone());
        category_panels_map.insert(DISPLAY_PANEL_NAME.to_string(), display_panel.clone());
        category_panels_map.insert(APPEARANCE_PANEL_NAME.to_string(), appearance_panel.clone());
        category_panels_map.insert(STARTUP_PROGRAM_PANEL_NAME.to_string(), startup_programs_panel.clone());
        category_panels_map.insert(KEYBINDS_PANEL_NAME.to_string(), keybinds_panel.clone());
        category_panels_map.insert(INFO_PANEL_NAME.to_string(), info_panel.clone());

        let category_boxes_stack = Stack::new();
        category_boxes_stack.add_named(general_panel.get_container_box(), Some(GENERAL_PANEL_NAME));
        category_boxes_stack.add_named(display_panel.get_container_box(), Some(DISPLAY_PANEL_NAME));
        category_boxes_stack.add_named(appearance_panel.get_container_box(), Some(APPEARANCE_PANEL_NAME));
        category_boxes_stack.add_named(startup_programs_panel.get_container_box(), Some(STARTUP_PROGRAM_PANEL_NAME));
        category_boxes_stack.add_named(keybinds_panel.get_container_box(), Some(KEYBINDS_PANEL_NAME));
        category_boxes_stack.add_named(info_panel.get_container_box(), Some(INFO_PANEL_NAME));

        Self {
            category_panels_map,
            category_boxes_stack
        }
    }

    pub fn select_panel_active(&self, panel_name: &str, settings: &Rc<RefCell<HyprlandSettings>>) {
        if let Some(panel) = self.category_panels_map.get(panel_name) {
            panel.reload_settings(settings)
        }

        self.category_boxes_stack.set_visible_child_name(panel_name);
    }

    pub fn get_panels_stack(&self) -> &Stack {
        &self.category_boxes_stack
    }
}