use std::collections::HashMap;
use gtk::{Orientation, Stack};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::states::settings_switcher_state::SettingsSwitcherState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::RcMut;

pub struct SettingsSwitcher {
    state: SettingsSwitcherState,
    settings_switcher_box: GTKBox,
    settings_switcher_stack: Stack,
    controls: HashMap<String, RcMut<dyn Control>>,
}

impl Control for SettingsSwitcher {
    fn init_events(&self) {

    }

    fn get_widget(&self) -> &GTKBox {
        &self.settings_switcher_box
    }
}

impl UpdatableControl<SettingsSwitcherState> for SettingsSwitcher {
    fn update_state(&mut self, state: SettingsSwitcherState) {
        self.set_control_visible(state.active_settings_name.as_str());
        self.state = state.clone();
    }
    fn get_current_state(&self) -> SettingsSwitcherState {
        self.state.clone()
    }
}

impl SettingsSwitcher {
    pub fn new() -> Self {
        let settings_switcher_stack = Stack::new();
        let settings_switcher_box = BoxBuilder::new("settings-switcher")
            .set_orientation(Orientation::Vertical)
            .set_full_height(true)
            .set_full_width(true)
            .build();
        settings_switcher_box.append(&settings_switcher_stack);

        let state = Default::default();
        let controls = HashMap::new();
        
        Self {
            state,
            settings_switcher_box,
            settings_switcher_stack,
            controls,
        }
    }

    pub fn insert_control(&mut self, name: String, control: RcMut<dyn Control>) -> &mut Self {
        self.settings_switcher_stack.add_named(control.borrow().get_widget(), Some(name.as_str()));
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