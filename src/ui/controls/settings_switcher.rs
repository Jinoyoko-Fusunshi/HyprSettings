use std::collections::HashMap;
use gtk::{Orientation, Stack};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::css_styles::CSSStyles;
use crate::ui::controls::Control;
use crate::ui::states::settings_switcher_state::SettingsSwitcherState;
use crate::ui::updatable_control::UpdatableControl;

pub struct SettingsSwitcher {
    settings_switcher_box: gtk::Box,
    settings_switcher_stack: Stack,
    controls: HashMap<String, Box<dyn Control>>,
}

impl Control for SettingsSwitcher {
    fn init_events(&self) {

    }

    fn get_widget(&self) -> &gtk::Box {
        &self.settings_switcher_box
    }
}

impl UpdatableControl<SettingsSwitcherState> for SettingsSwitcher {
    fn update_ui(&mut self, state: SettingsSwitcherState) {
        self.set_control_visible(state.active_settings_name.as_str());
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
            controls: HashMap::new(),
        }
    }

    pub fn insert_control(&mut self, name: String, control: Box<dyn Control>) -> &mut Self {
        self.settings_switcher_stack.add_named(control.get_widget(), Some(name.as_str()));
        self.controls.insert(name, control);
        self
    }

    pub fn set_control_visible(&self, control_name: &str) {
        let control_box = self.settings_switcher_stack.child_by_name(control_name);
        if let Some(control_box) = control_box {
            self.settings_switcher_stack.set_child_visible(true);
            self.settings_switcher_stack.set_visible_child(&control_box);
        } else {
            self.settings_switcher_stack.set_child_visible(false);
        }
    }
}