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
use crate::settings::key_binds::custom_keybind::CustomKeybind;
use crate::settings::key_binds::key_bind_configuration::KeyBindConfiguration;
use crate::settings::key_binds::system_keybind::SystemKeybind;
use crate::ui::controls::editable_control_element::{EditMode, EditableControlElement};
use crate::ui::controls::panel::Panel;
use crate::ui::controls::panel::key_binds_panel::key_bind_entry_field::KeyBindEntryField;
use crate::ui::controls::panel::key_binds_panel::custom_key_bind_entry_field::{CustomKeyBindEntryField};

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
    pub fn new(settings: &Rc<RefCell<HyprlandSettings>>) -> Self {
        let key_binds_panel_scroll_box = gtk::Box::new(Orientation::Vertical, 10);
        key_binds_panel_scroll_box.set_margin_top(10);
        key_binds_panel_scroll_box.set_margin_bottom(10);
        key_binds_panel_scroll_box.set_margin_start(10);
        key_binds_panel_scroll_box.set_margin_end(10);
        key_binds_panel_scroll_box.set_vexpand(true);

        let scroll_window = ScrolledWindow::new();
        scroll_window.set_vexpand(true);

        let key_binds_panel_box = gtk::Box::new(Orientation::Vertical, 10);
        let system_key_binds_section = Self::create_keybinds_section_box(
            "System", settings, Self::create_system_key_binds_box
        );
        let focus_key_binds_section = Self::create_keybinds_section_box(
            "Focus window", settings, Self::create_focus_key_binds_box
        );
        let workspace_key_binds_section = Self::create_keybinds_section_box(
            "Workspace", settings, Self::create_workspace_key_binds_box
        );
        let move_window_key_binds_section = Self::create_keybinds_section_box(
            "Move window", settings, Self::create_move_window_key_binds_box
        );
        let custom_key_binds_section = Self::create_keybinds_section_box(
            "Custom", settings, Self::create_custom_key_binds_box
        );

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

    fn create_system_key_binds_box(section_box: &gtk::Box, settings: &Rc<RefCell<HyprlandSettings>>) {
        let terminal_key_bind_field = Self::create_keybind_entry_field(
            settings, "Terminal".to_string(), SystemKeybind::Terminal
        );
        let close_window_key_bind_field = Self::create_keybind_entry_field(
            settings, "Close window".to_string(), SystemKeybind::CloseWindow
        );
        let exit_hyprland_key_bind_field = Self::create_keybind_entry_field(
            settings, "Exit hyprland".to_string(), SystemKeybind::ExitHyprland
        );
        let file_manager_key_bind_field = Self::create_keybind_entry_field(
            settings, "File manager".to_string(), SystemKeybind::FileManager
        );
        let toggle_floating_key_bind_field = Self::create_keybind_entry_field(
            settings, "Toggle floating window".to_string(), SystemKeybind::ToggleFloatingWindow
        );
        let run_program_key_bind_field = Self::create_keybind_entry_field(
            settings, "Run program".to_string(), SystemKeybind::RunProgram
        );
        let pseudo_key_bind_field = Self::create_keybind_entry_field(
            settings, "Pseudo".to_string(), SystemKeybind::Pseudo
        );
        let split_window_key_bind_field = Self::create_keybind_entry_field(
            settings, "Split window".to_string(), SystemKeybind::SplitWindow
        );
        let screenshot_key_bind_field = Self::create_keybind_entry_field(
            settings, "Screenshot".to_string(), SystemKeybind::Screenshot
        );
        let screenshot_window_key_bind_field = Self::create_keybind_entry_field(
            settings, "Screenshot window".to_string(), SystemKeybind::ScreenshotWindow
        );
        let lock_screen_key_bind_field = Self::create_keybind_entry_field(
            settings, "Lock screen".to_string(), SystemKeybind::LockScreen
        );
        let emoji_selector_key_bind_field = Self::create_keybind_entry_field(
            settings, "Emoji selector".to_string(), SystemKeybind::EmojiSelector
        );

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

    fn create_focus_key_binds_box(section_box: &gtk::Box, settings: &Rc<RefCell<HyprlandSettings>>) {
        let focus_left_window_bind_field = Self::create_keybind_entry_field(
            settings, "Focus left window".to_string(), SystemKeybind::FocusLeftWindow
        );
        let focus_right_window_bind_field = Self::create_keybind_entry_field(
            settings, "Focus right window".to_string(), SystemKeybind::FocusRightWindow
        );
        let focus_top_window_key_bind_field = Self::create_keybind_entry_field(
            settings, "Focus top window".to_string(), SystemKeybind::FocusTopWindow
        );
        let focus_bottom_window_key_bind_field = Self::create_keybind_entry_field(
            settings, "Focus bottom window".to_string(), SystemKeybind::FocusBottomWindow
        );

        section_box.append(focus_left_window_bind_field.get_container_box());
        section_box.append(focus_right_window_bind_field.get_container_box());
        section_box.append(focus_top_window_key_bind_field.get_container_box());
        section_box.append(focus_bottom_window_key_bind_field.get_container_box());
    }

    fn create_workspace_key_binds_box(section_box: &gtk::Box, settings: &Rc<RefCell<HyprlandSettings>>) {
        let switch_workspace_one = Self::create_keybind_entry_field(
            settings, "Workspace 1".to_string(), SystemKeybind::SwitchWorkspaceOne
        );
        let switch_workspace_two = Self::create_keybind_entry_field(
            settings, "Workspace 2".to_string(), SystemKeybind::SwitchWorkspaceTwo
        );
        let switch_workspace_three = Self::create_keybind_entry_field(
            settings, "Workspace 3".to_string(), SystemKeybind::SwitchWorkspaceThree
        );
        let switch_workspace_four = Self::create_keybind_entry_field(
            settings, "Workspace 4".to_string(), SystemKeybind::SwitchWorkspaceFour
        );
        let switch_workspace_five = Self::create_keybind_entry_field(
            settings, "Workspace 5".to_string(), SystemKeybind::SwitchWorkspaceFive
        );
        let switch_workspace_six = Self::create_keybind_entry_field(
            settings, "Workspace 6".to_string(), SystemKeybind::SwitchWorkspaceSix
        );
        let switch_workspace_seven = Self::create_keybind_entry_field(
            settings, "Workspace 7".to_string(), SystemKeybind::SwitchWorkspaceSeven
        );
        let switch_workspace_eight = Self::create_keybind_entry_field(
            settings, "Workspace 8".to_string(), SystemKeybind::SwitchWorkspaceEight
        );
        let switch_workspace_nine = Self::create_keybind_entry_field(
            settings, "Workspace 9".to_string(), SystemKeybind::SwitchWorkspaceNine
        );
        let switch_workspace_zero = Self::create_keybind_entry_field(
            settings, "Workspace 0".to_string(), SystemKeybind::SwitchWorkspaceZero
        );

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

    fn create_move_window_key_binds_box(section_box: &gtk::Box, settings: &Rc<RefCell<HyprlandSettings>>) {
        let switch_window_workspace_one = Self::create_keybind_entry_field(
            settings, "Move to Workspace 1".to_string(), SystemKeybind::MoveWorkspaceOne
        );
        let switch_window_workspace_two = Self::create_keybind_entry_field(
            settings, "Move to Workspace 2".to_string(), SystemKeybind::MoveWorkspaceTwo
        );
        let switch_window_workspace_three = Self::create_keybind_entry_field(
            settings, "Move to Workspace 3".to_string(), SystemKeybind::MoveWorkspaceThree
        );
        let switch_window_workspace_four = Self::create_keybind_entry_field(
            settings, "Move to Workspace 4".to_string(), SystemKeybind::MoveWorkspaceFour
        );
        let switch_window_workspace_five = Self::create_keybind_entry_field(
            settings, "Move to Workspace 5".to_string(), SystemKeybind::MoveWorkspaceFive
        );
        let switch_window_workspace_six = Self::create_keybind_entry_field(
            settings, "Move to Workspace 6".to_string(), SystemKeybind::MoveWorkspaceSix
        );
        let switch_window_workspace_seven = Self::create_keybind_entry_field(
            settings, "Move to Workspace 7".to_string(), SystemKeybind::MoveWorkspaceSeven
        );
        let switch_window_workspace_eight = Self::create_keybind_entry_field(
            settings, "Move to Workspace 8".to_string(), SystemKeybind::MoveWorkspaceEight
        );
        let switch_window_workspace_nine = Self::create_keybind_entry_field(
            settings, "Move to Workspace 9".to_string(), SystemKeybind::MoveWorkspaceNine
        );
        let switch_window_workspace_zero = Self::create_keybind_entry_field(
            settings, "Move to Workspace 0".to_string(), SystemKeybind::MoveWorkspaceZero
        );

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

    fn create_custom_key_binds_box(section_box: &gtk::Box, settings: &Rc<RefCell<HyprlandSettings>>) {
        let custom_key_bind_entries_box = gtk::Box::new(Orientation::Vertical, 10);

        let custom_key_bind_entries_box_clone = custom_key_bind_entries_box.clone();
        let settings_clone = settings.clone();
        let create_custom_key_bind_entry_button_callback = move |_: &Button| {
            Self::create_custom_keybind(
                &settings_clone, &custom_key_bind_entries_box_clone, None, None, EditMode::Edit
            );
        };
        let add_custom_key_bind_entry_button = Button::with_label("➕ Add custom keybind");
        add_custom_key_bind_entry_button.connect_clicked(create_custom_key_bind_entry_button_callback);

        let custom_key_bind_entries_box_clone = custom_key_bind_entries_box.clone();
        Self::create_custom_keybinds_from_settings(&settings, &custom_key_bind_entries_box_clone);
        section_box.append(&custom_key_bind_entries_box_clone);
        section_box.append(&add_custom_key_bind_entry_button);
    }

    fn create_custom_keybinds_from_settings(
        settings: &Rc<RefCell<HyprlandSettings>>, custom_keybind_entries_box: &gtk::Box
    ) {
        for (custom_keybind_name, custom_keybind) in settings.borrow().key_bind_settings.get_custom_key_binds() {
            Self::create_custom_keybind(settings, custom_keybind_entries_box,
            Some(custom_keybind_name.clone()), Some(custom_keybind.clone()), EditMode::Locked);
        }
    }

    fn create_custom_keybind(
        settings: &Rc<RefCell<HyprlandSettings>>, custom_keybind_entries_box: &gtk::Box,
        custom_keybind_name: Option<String>, custom_keybind: Option<CustomKeybind>, edit_mode: EditMode
    ) {
        let custom_keybind_entry_field = CustomKeyBindEntryField::new(
            custom_keybind_name, custom_keybind
        );
        custom_keybind_entry_field.init_events();

        let editable_control_element = EditableControlElement::new(
            custom_keybind_entry_field.clone(), edit_mode
        );

        let custom_keybind_entry_field_clone = custom_keybind_entry_field.clone();
        let editable_control_element_button_click_callback = move |settings: Rc<RefCell<HyprlandSettings>>| {
            custom_keybind_entry_field_clone.save_setting(settings);
        };
        editable_control_element.set_toggle_button_click_callback(
            settings.clone(), editable_control_element_button_click_callback
        );

        let custom_keybind_entries_box_clone = custom_keybind_entries_box.clone();
        let editable_control_element_clone = editable_control_element.clone();
        let settings_clone = settings.clone();
        let custom_keybind_entry_field_clone = custom_keybind_entry_field.clone();
        let delete_button_click_callback = move |_: &Button| {
            custom_keybind_entries_box_clone.remove(editable_control_element_clone.get_container_box());
            custom_keybind_entry_field_clone.remove_setting(settings_clone.clone());
        };
        custom_keybind_entry_field.set_delete_button_callback(delete_button_click_callback);
        custom_keybind_entries_box.append(editable_control_element.get_container_box());
    }

    fn create_keybinds_section_box(
        header_label: &str, settings: &Rc<RefCell<HyprlandSettings>>,
        section_content_callback: impl Fn(&gtk::Box, &Rc<RefCell<HyprlandSettings>>)
    ) -> gtk::Box {
        let section_box = gtk::Box::new(Orientation::Vertical, 10);
        section_box.set_margin_bottom(20);

        let section_header_label = Label::new(Some(header_label));
        let separator = Separator::new(Orientation::Horizontal);
        separator.set_margin_bottom(10);

        section_box.append(&section_header_label);
        section_box.append(&separator);

        section_content_callback(&section_box, settings);
        section_box
    }

    fn create_keybind_entry_field(
        settings: &Rc<RefCell<HyprlandSettings>>, entry_field_name: String, system_keybind: SystemKeybind
    ) -> KeyBindEntryField {
        let keybind_entry_changed_callback = Self::create_key_bind_entry_field_change_callback(
            settings.clone(), system_keybind.clone()
        );

        let program_keybind = settings.borrow().key_bind_settings
            .get_program_key_bind(system_keybind);
        let keybind_entry_field = KeyBindEntryField::new(
            entry_field_name, program_keybind
        );
        keybind_entry_field.set_input_callback(keybind_entry_changed_callback);
        keybind_entry_field
    }

    fn create_key_bind_entry_field_change_callback(
        settings: Rc<RefCell<HyprlandSettings>>,
        system_keybind: SystemKeybind
    ) -> impl Fn(KeyBindConfiguration) {
        let callback_function = move |keybind_configuration: KeyBindConfiguration| {
            settings.borrow_mut().key_bind_settings
                .set_program_key_bind(system_keybind.clone(), keybind_configuration.clone());
        };

        callback_function
    }
}