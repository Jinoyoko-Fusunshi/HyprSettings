use std::collections::HashMap;
use gtk::{Orientation, Stack};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::css_styles::CSSStyles;
use crate::ui::component::Component;
use crate::ui::states::settings_switcher_state::SettingsSwitcherState;
use crate::ui::updatable_component::UpdatableComponent;

pub struct SettingsSwitcher {
    settings_switcher_box: gtk::Box,
    settings_switcher_stack: Stack,
    components: HashMap<String, Box<dyn Component>>,
}

impl Component for SettingsSwitcher {
    fn init_events(&self) {

    }

    fn get_widget(&self) -> &gtk::Box {
        &self.settings_switcher_box
    }
}

impl UpdatableComponent<SettingsSwitcherState> for SettingsSwitcher {
    fn update_ui(&mut self, state: SettingsSwitcherState) {
        self.set_component_visible(state.active_settings_name.as_str());
    }
}

impl SettingsSwitcher {
    pub fn new() -> Self {
        let settings_switcher_stack = Stack::new();
        let settings_switcher_box = gtk::Box::new(Orientation::Vertical, 10);
        settings_switcher_box.set_hexpand(true);
        settings_switcher_box.add_css_class(CSSStyles::CONTENT_PANEL);

        settings_switcher_box.append(&settings_switcher_stack);

        Self {
            settings_switcher_box,
            settings_switcher_stack,
            components: HashMap::new(),
        }
    }

    pub fn insert_component(&mut self, name: String, component: Box<dyn Component>) -> &mut Self {
        self.settings_switcher_stack.add_named(component.get_widget(), Some(name.as_str()));
        self.components.insert(name, component);
        self
    }

    pub fn set_component_visible(&self, component_name: &str) {
        let ui_component_box = self.settings_switcher_stack.child_by_name(component_name);
        if let Some(ui_component_box) = ui_component_box {
            self.settings_switcher_stack.set_child_visible(true);
            self.settings_switcher_stack.set_visible_child(&ui_component_box);
        } else {
            self.settings_switcher_stack.set_child_visible(false);
        }
    }
}