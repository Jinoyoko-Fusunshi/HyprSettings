mod key_bind_entry_field;
mod key_bind_entry;
mod custom_key_bind_entry_field;
mod key_symbol;

use gtk::{Button, ScrolledWindow, Separator};
use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Label, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::ui::controls::panel::{
    Panel, key_binds_panel::key_bind_entry_field::{KeyBindEntryField}
};
use crate::ui::controls::panel::key_binds_panel::custom_key_bind_entry_field::CustomKeyBindEntryField;

pub struct KeyBindsPanel {
    key_binds_panel_box: gtk::Box
}

impl Panel for KeyBindsPanel {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.key_binds_panel_box
    }
}

impl Clone for KeyBindsPanel {
    fn clone(&self) -> Self {
        Self {
            key_binds_panel_box: self.key_binds_panel_box.clone()
        }
    }
}

impl KeyBindsPanel {
    pub fn new() -> Self {
        let key_binds_panel_scroll_box = gtk::Box::new(Orientation::Vertical, 10);
        key_binds_panel_scroll_box.set_margin_top(10);
        key_binds_panel_scroll_box.set_margin_bottom(10);
        key_binds_panel_scroll_box.set_margin_start(10);
        key_binds_panel_scroll_box.set_margin_end(10);
        key_binds_panel_scroll_box.set_vexpand(true);

        let scroll_window = ScrolledWindow::new();
        scroll_window.set_vexpand(true);

        let key_binds_panel_box = gtk::Box::new(Orientation::Vertical, 10);

        let system_key_binds_section = Self::create_section("System", Self::create_system_key_binds_box);
        let focus_key_binds_section = Self::create_section("Focus window", Self::create_focus_key_binds_box);
        let workspace_key_binds_section = Self::create_section("Workspace", Self::create_workspace_key_binds_box);
        let move_window_key_binds_section = Self::create_section("Move window", Self::create_move_window_key_binds_box);
        let custom_key_binds_section = Self::create_section("Custom", Self::create_custom_key_binds_box);

        key_binds_panel_box.append(&system_key_binds_section);
        key_binds_panel_box.append(&focus_key_binds_section);
        key_binds_panel_box.append(&workspace_key_binds_section);
        key_binds_panel_box.append(&move_window_key_binds_section);
        key_binds_panel_box.append(&custom_key_binds_section);

        scroll_window.set_child(Some(&key_binds_panel_box));
        key_binds_panel_scroll_box.append(&scroll_window);

        Self {
            key_binds_panel_box: key_binds_panel_scroll_box
        }
    }

    fn create_system_key_binds_box(section_box: &gtk::Box) {
        let terminal_key_bind_field = KeyBindEntryField::new("Terminal".to_string());
        let close_window_key_bind_field = KeyBindEntryField::new("Close window".to_string());
        let exit_hyprland_key_bind_field = KeyBindEntryField::new("Exit hyprland".to_string());
        let file_manager_key_bind_field = KeyBindEntryField::new("File manager".to_string());
        let toggle_floating_key_bind_field = KeyBindEntryField::new("Toggle floating window".to_string());
        let run_program_key_bind_field = KeyBindEntryField::new("Run program".to_string());
        let pseudo_key_bind_field = KeyBindEntryField::new("Pseudo".to_string());
        let split_window_key_bind_field = KeyBindEntryField::new("Split window".to_string());
        let screenshot_key_bind_field = KeyBindEntryField::new("Screenshot".to_string());
        let screenshot_window_key_bind_field = KeyBindEntryField::new("Screenshot window".to_string());
        let lock_screen_key_bind_field = KeyBindEntryField::new("Lock screen".to_string());
        let emoji_selector_key_bind_field = KeyBindEntryField::new("Emoji Selector".to_string());

        section_box.append(terminal_key_bind_field.get_container_box());
        section_box.append(close_window_key_bind_field.get_container_box());
        section_box.append(exit_hyprland_key_bind_field.get_container_box());
        section_box.append(file_manager_key_bind_field.get_container_box());
        section_box.append(toggle_floating_key_bind_field.get_container_box());
        section_box.append(run_program_key_bind_field.get_container_box());
        section_box.append(pseudo_key_bind_field.get_container_box());
        section_box.append(split_window_key_bind_field.get_container_box());
        section_box.append(screenshot_key_bind_field.get_container_box());
        section_box.append(screenshot_window_key_bind_field.get_container_box());
        section_box.append(lock_screen_key_bind_field.get_container_box());
        section_box.append(emoji_selector_key_bind_field.get_container_box());
    }

    fn create_focus_key_binds_box(section_box: &gtk::Box) {
        let focus_left_window_bind_field = KeyBindEntryField::new("Focus left window".to_string());
        let focus_right_window_bind_field = KeyBindEntryField::new("Focus right window".to_string());
        let focus_top_window_key_bind_field = KeyBindEntryField::new("Focus top window".to_string());
        let focus_bottom_window_key_bind_field = KeyBindEntryField::new("Focus bottom window".to_string());

        section_box.append(focus_left_window_bind_field.get_container_box());
        section_box.append(focus_right_window_bind_field.get_container_box());
        section_box.append(focus_top_window_key_bind_field.get_container_box());
        section_box.append(focus_bottom_window_key_bind_field.get_container_box());
    }

    fn create_workspace_key_binds_box(section_box: &gtk::Box) {
        let switch_workspace_one = KeyBindEntryField::new("Workspace 1".to_string());
        let switch_workspace_two = KeyBindEntryField::new("Workspace 2".to_string());
        let switch_workspace_three = KeyBindEntryField::new("Workspace 3".to_string());
        let switch_workspace_four = KeyBindEntryField::new("Workspace 4".to_string());
        let switch_workspace_five = KeyBindEntryField::new("Workspace 5".to_string());
        let switch_workspace_six = KeyBindEntryField::new("Workspace 6".to_string());
        let switch_workspace_seven = KeyBindEntryField::new("Workspace 7".to_string());
        let switch_workspace_eight = KeyBindEntryField::new("Workspace 8".to_string());
        let switch_workspace_nine = KeyBindEntryField::new("Workspace 9".to_string());
        let switch_workspace_zero = KeyBindEntryField::new("Workspace 0".to_string());

        section_box.append(switch_workspace_one.get_container_box());
        section_box.append(switch_workspace_two.get_container_box());
        section_box.append(switch_workspace_three.get_container_box());
        section_box.append(switch_workspace_four.get_container_box());
        section_box.append(switch_workspace_five.get_container_box());
        section_box.append(switch_workspace_six.get_container_box());
        section_box.append(switch_workspace_seven.get_container_box());
        section_box.append(switch_workspace_eight.get_container_box());
        section_box.append(switch_workspace_nine.get_container_box());
        section_box.append(switch_workspace_zero.get_container_box());
    }

    fn create_move_window_key_binds_box(section_box: &gtk::Box) {
        let switch_window_workspace_one = KeyBindEntryField::new("Move to Workspace 1".to_string());
        let switch_window_workspace_two = KeyBindEntryField::new("Move to Workspace 2".to_string());
        let switch_window_workspace_three = KeyBindEntryField::new("Move to Workspace 3".to_string());
        let switch_window_workspace_four = KeyBindEntryField::new("Move to Workspace 4".to_string());
        let switch_window_workspace_five = KeyBindEntryField::new("Move to Workspace 5".to_string());
        let switch_window_workspace_six = KeyBindEntryField::new("Move to Workspace 6".to_string());
        let switch_window_workspace_seven = KeyBindEntryField::new("Move to Workspace 7".to_string());
        let switch_window_workspace_eight = KeyBindEntryField::new("Move to Workspace 8".to_string());
        let switch_window_workspace_nine = KeyBindEntryField::new("Move to Workspace 9".to_string());
        let switch_window_workspace_zero = KeyBindEntryField::new("Move to Workspace 0".to_string());

        section_box.append(switch_window_workspace_one.get_container_box());
        section_box.append(switch_window_workspace_two.get_container_box());
        section_box.append(switch_window_workspace_three.get_container_box());
        section_box.append(switch_window_workspace_four.get_container_box());
        section_box.append(switch_window_workspace_five.get_container_box());
        section_box.append(switch_window_workspace_six.get_container_box());
        section_box.append(switch_window_workspace_seven.get_container_box());
        section_box.append(switch_window_workspace_eight.get_container_box());
        section_box.append(switch_window_workspace_nine.get_container_box());
        section_box.append(switch_window_workspace_zero.get_container_box());
    }

    fn create_custom_key_binds_box(section_box: &gtk::Box) {
        let custom_key_bind_entries_box = gtk::Box::new(Orientation::Vertical, 10);
        let custom_key_bind_entries_box_clone = custom_key_bind_entries_box.clone();

        let add_custom_key_bind_entry_button_callback = move |_: &Button| {
            let custom_key_bind_entry_field = CustomKeyBindEntryField::new(&custom_key_bind_entries_box_clone);
            custom_key_bind_entries_box_clone.append(custom_key_bind_entry_field.get_container_box());
        };
        let add_custom_key_bind_entry_button = Button::with_label("Add custom key bind");
        add_custom_key_bind_entry_button.connect_clicked(add_custom_key_bind_entry_button_callback);

        section_box.append(&custom_key_bind_entries_box);
        section_box.append(&add_custom_key_bind_entry_button);
    }

    fn create_section(header_label: &str, section_content_callback: impl Fn(&gtk::Box)) -> gtk::Box {
        let section_box = gtk::Box::new(Orientation::Vertical, 10);
        section_box.set_margin_bottom(20);

        let section_header_label = Label::new(Some(header_label));
        let separator = Separator::new(Orientation::Horizontal);
        separator.set_margin_bottom(10);

        section_box.append(&section_header_label);
        section_box.append(&separator);

        section_content_callback(&section_box);

        section_box
    }
}