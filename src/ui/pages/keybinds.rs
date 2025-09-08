use gtk::{Button, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;
use gtk::Orientation;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::models::keybinds::custom_keybind::CustomKeybind;
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::models::keybinds::system_keybind::SystemKeybind;
use crate::providers::application_provider::ApplicationProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::controls::Control;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::controls::editable_control_element::{EditMode, EditableControlElement, EditableControlElementManager};
use crate::ui::controls::keybinds::custom_keybind_input_field::CustomKeyBindInputField;
use crate::ui::controls::keybinds::keybind_input_field::KeybindInputField;
use crate::ui::statable_control::StatableControl;
use crate::ui::state_savable_control::StateSavableControl;
use crate::ui::states::custom_keybind_input_field_state::CustomKeybindInputFieldState;
use crate::ui::states::editable_control_element_state::EditableControlElementState;
use crate::ui::states::keybind_input_field_state::KeybindInputFieldState;
use crate::ui::updatable_control::UpdatableControl;

pub const CUSTOM_ITEM: &str = "Custom";

pub struct Keybinds {
    key_binds_panel_box: GTKBox
}

impl Control for Keybinds {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.key_binds_panel_box
    }
}

impl Clone for Keybinds {
    fn clone(&self) -> Self {
        Self {
            key_binds_panel_box: self.key_binds_panel_box.clone()
        }
    }
}

impl Keybinds {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        let key_binds_panel_scroll_box = BoxBuilder::new("keybinds")
            .set_orientation(Orientation::Vertical)
            .set_margin(DEFAULT_MARGIN)
            .set_full_height(true)
            .build();

        let scroll_window = ScrolledWindow::new();
        scroll_window.set_vexpand(true);

        let system_keybinds_section_box = Self::create_system_keybinds_section_box(&application_provider);
        let focus_keybinds_section_box = Self::create_focus_keybinds_section_box(&application_provider);
        let workspace_keybinds_section_box = Self::create_workspace_keybinds_section_box(&application_provider);
        let move_window_keybinds_section_box = Self::create_move_window_keybinds_section_box(&application_provider);
        let custom_keybinds_section_box = Self::create_custom_keybinds_section_box(&application_provider);

        let key_binds_panel_box = BoxBuilder::new("keybinds")
            .set_orientation(Orientation::Vertical)
            .build();

        key_binds_panel_box.append(&system_keybinds_section_box);
        key_binds_panel_box.append(&focus_keybinds_section_box);
        key_binds_panel_box.append(&workspace_keybinds_section_box);
        key_binds_panel_box.append(&move_window_keybinds_section_box);
        key_binds_panel_box.append(&custom_keybinds_section_box);

        scroll_window.set_child(Some(&key_binds_panel_box));
        key_binds_panel_scroll_box.append(&scroll_window);

        Self {
            key_binds_panel_box: key_binds_panel_scroll_box
        }
    }

    fn create_system_keybinds_section_box(application_provider: &ApplicationProvider) -> GTKBox {
        let system_keybinds_section_box = SectionBoxBuilder::new("system-keybinds-section", 0)
            .create_header_elements("System keybinds")
            .build()
            .expect("Cannot build system keybinds section box");

        let terminal_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Terminal".to_string(), SystemKeybind::Terminal
        );
        let close_window_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Close window".to_string(), SystemKeybind::CloseWindow
        );
        let exit_hyprland_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Exit hyprland".to_string(), SystemKeybind::ExitHyprland
        );
        let file_manager_key_bind_field = Self::create_keybind_input_field(
            application_provider, "File manager".to_string(), SystemKeybind::FileManager
        );
        let toggle_floating_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Toggle floating window".to_string(), SystemKeybind::ToggleFloatingWindow
        );
        let run_program_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Run program".to_string(), SystemKeybind::RunProgram
        );
        let pseudo_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Pseudo".to_string(), SystemKeybind::Pseudo
        );
        let split_window_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Split window".to_string(), SystemKeybind::SplitWindow
        );
        let screenshot_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Screenshot".to_string(), SystemKeybind::Screenshot
        );
        let screenshot_window_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Screenshot window".to_string(), SystemKeybind::ScreenshotWindow
        );
        let lock_screen_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Lock screen".to_string(), SystemKeybind::LockScreen
        );
        let emoji_selector_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Emoji selector".to_string(), SystemKeybind::EmojiSelector
        );

        system_keybinds_section_box.append(terminal_key_bind_field.get_widget());
        system_keybinds_section_box.append(close_window_key_bind_field.get_widget());
        system_keybinds_section_box.append(exit_hyprland_key_bind_field.get_widget());
        system_keybinds_section_box.append(file_manager_key_bind_field.get_widget());
        system_keybinds_section_box.append(toggle_floating_key_bind_field.get_widget());
        system_keybinds_section_box.append(run_program_key_bind_field.get_widget());
        system_keybinds_section_box.append(pseudo_key_bind_field.get_widget());
        system_keybinds_section_box.append(split_window_key_bind_field.get_widget());
        system_keybinds_section_box.append(screenshot_key_bind_field.get_widget());
        system_keybinds_section_box.append(screenshot_window_key_bind_field.get_widget());
        system_keybinds_section_box.append(lock_screen_key_bind_field.get_widget());
        system_keybinds_section_box.append(emoji_selector_key_bind_field.get_widget());
        system_keybinds_section_box
    }

    fn create_focus_keybinds_section_box(application_provider: &ApplicationProvider) -> GTKBox {
        let focus_window_keybinds_section_box = SectionBoxBuilder::new("focus-keybinds-section", 0)
            .create_header_elements("Focus window")
            .build()
            .expect("Cannot build system keybinds section box");

        let focus_left_window_bind_field = Self::create_keybind_input_field(
            application_provider, "Focus left window".to_string(), SystemKeybind::FocusLeftWindow
        );
        let focus_right_window_bind_field = Self::create_keybind_input_field(
            application_provider, "Focus right window".to_string(), SystemKeybind::FocusRightWindow
        );
        let focus_top_window_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Focus top window".to_string(), SystemKeybind::FocusTopWindow
        );
        let focus_bottom_window_key_bind_field = Self::create_keybind_input_field(
            application_provider, "Focus bottom window".to_string(), SystemKeybind::FocusBottomWindow
        );

        focus_window_keybinds_section_box.append(focus_left_window_bind_field.get_widget());
        focus_window_keybinds_section_box.append(focus_right_window_bind_field.get_widget());
        focus_window_keybinds_section_box.append(focus_top_window_key_bind_field.get_widget());
        focus_window_keybinds_section_box.append(focus_bottom_window_key_bind_field.get_widget());
        focus_window_keybinds_section_box
    }

    fn create_workspace_keybinds_section_box(application_provider: &ApplicationProvider) -> GTKBox {
        let switch_workspace_keybinds_section_box = SectionBoxBuilder::new("workspace-keybinds-section", 0)
            .create_header_elements("Workspace")
            .build()
            .expect("Cannot build system keybinds section box");

        let switch_workspace_one = Self::create_keybind_input_field(
            application_provider, "Workspace 1".to_string(), SystemKeybind::SwitchWorkspaceOne
        );
        let switch_workspace_two = Self::create_keybind_input_field(
            application_provider, "Workspace 2".to_string(), SystemKeybind::SwitchWorkspaceTwo
        );
        let switch_workspace_three = Self::create_keybind_input_field(
            application_provider, "Workspace 3".to_string(), SystemKeybind::SwitchWorkspaceThree
        );
        let switch_workspace_four = Self::create_keybind_input_field(
            application_provider, "Workspace 4".to_string(), SystemKeybind::SwitchWorkspaceFour
        );
        let switch_workspace_five = Self::create_keybind_input_field(
            application_provider, "Workspace 5".to_string(), SystemKeybind::SwitchWorkspaceFive
        );
        let switch_workspace_six = Self::create_keybind_input_field(
            application_provider, "Workspace 6".to_string(), SystemKeybind::SwitchWorkspaceSix
        );
        let switch_workspace_seven = Self::create_keybind_input_field(
            application_provider, "Workspace 7".to_string(), SystemKeybind::SwitchWorkspaceSeven
        );
        let switch_workspace_eight = Self::create_keybind_input_field(
            application_provider, "Workspace 8".to_string(), SystemKeybind::SwitchWorkspaceEight
        );
        let switch_workspace_nine = Self::create_keybind_input_field(
            application_provider, "Workspace 9".to_string(), SystemKeybind::SwitchWorkspaceNine
        );
        let switch_workspace_zero = Self::create_keybind_input_field(
            application_provider, "Workspace 0".to_string(), SystemKeybind::SwitchWorkspaceZero
        );

        switch_workspace_keybinds_section_box.append(switch_workspace_one.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_two.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_three.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_four.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_five.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_six.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_seven.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_eight.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_nine.get_widget());
        switch_workspace_keybinds_section_box.append(switch_workspace_zero.get_widget());
        switch_workspace_keybinds_section_box
    }

    fn create_move_window_keybinds_section_box(application_provider: &ApplicationProvider) -> GTKBox {
        let move_window_keybinds_section_box = SectionBoxBuilder::new("move-window-keybinds-section", 0)
            .create_header_elements("Move window")
            .build()
            .expect("Cannot build system keybinds section box");

        let switch_window_workspace_one = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 1".to_string(), SystemKeybind::MoveWorkspaceOne
        );

        let switch_window_workspace_two = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 2".to_string(), SystemKeybind::MoveWorkspaceTwo
        );
        let switch_window_workspace_three = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 3".to_string(), SystemKeybind::MoveWorkspaceThree
        );
        let switch_window_workspace_four = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 4".to_string(), SystemKeybind::MoveWorkspaceFour
        );
        let switch_window_workspace_five = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 5".to_string(), SystemKeybind::MoveWorkspaceFive
        );
        let switch_window_workspace_six = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 6".to_string(), SystemKeybind::MoveWorkspaceSix
        );
        let switch_window_workspace_seven = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 7".to_string(), SystemKeybind::MoveWorkspaceSeven
        );
        let switch_window_workspace_eight = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 8".to_string(), SystemKeybind::MoveWorkspaceEight
        );
        let switch_window_workspace_nine = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 9".to_string(), SystemKeybind::MoveWorkspaceNine
        );
        let switch_window_workspace_zero = Self::create_keybind_input_field(
            application_provider, "Move to Workspace 0".to_string(), SystemKeybind::MoveWorkspaceZero
        );

        move_window_keybinds_section_box.append(switch_window_workspace_one.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_two.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_three.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_four.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_five.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_six.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_seven.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_eight.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_nine.get_widget());
        move_window_keybinds_section_box.append(switch_window_workspace_zero.get_widget());
        move_window_keybinds_section_box
    }

    fn create_custom_keybinds_section_box(application_provider: &ApplicationProvider) -> GTKBox {
        let custom_keybinds_section_box = SectionBoxBuilder::new("custom-keybinds-section", 0)
            .create_header_elements(CUSTOM_ITEM)
            .build()
            .expect("Cannot build system keybinds section box");

        let custom_keybind_entries_box = BoxBuilder::new("custom-keybind-entries")
            .set_orientation(Orientation::Vertical)
            .build();

        let custom_keybind_entries_box_clone = custom_keybind_entries_box.clone();
        let application_provider_clone = application_provider.clone();
        let create_custom_keybind_entry_button_callback = move |_: &Button| {
            Self::create_custom_keybind(
                application_provider_clone.clone(), &custom_keybind_entries_box_clone, None, None, EditMode::Edit
            );
        };

        let add_custom_key_bind_entry_button = Button::with_label("➕ Add custom keybind");
        add_custom_key_bind_entry_button.connect_clicked(create_custom_keybind_entry_button_callback);

        let custom_key_bind_entries_box_clone = custom_keybind_entries_box.clone();
        Self::create_custom_keybinds_from_settings(application_provider, &custom_key_bind_entries_box_clone);
        custom_keybinds_section_box.append(&custom_key_bind_entries_box_clone);
        custom_keybinds_section_box.append(&add_custom_key_bind_entry_button);
        custom_keybinds_section_box
    }

    fn create_custom_keybinds_from_settings(
        application_manager: &ApplicationProvider, custom_keybind_entries_box: &GTKBox
    ) {
        let settings_provider = application_manager.get_keybinds_provider();
        let custom_keybinds = settings_provider.borrow().get_custom_keybinds();

        for (keybind_name, keybind) in custom_keybinds {
            Self::create_custom_keybind(application_manager.clone(), custom_keybind_entries_box,
            Some(keybind_name.clone()), Some(keybind.clone()), EditMode::Locked);
        }
    }

    fn create_custom_keybind(
        application_provider: ApplicationProvider, custom_keybind_entries_box: &GTKBox,
        custom_keybind_name: Option<String>, custom_keybind: Option<CustomKeybind>, edit_mode: EditMode
    ) {
        let mut custom_keybind_input_field = CustomKeyBindInputField::new();
        let command = match custom_keybind.clone() {
            Some(custom_keybind) => Some(custom_keybind.command),
            None => None
        };

        let keybind = match custom_keybind.clone() {
            Some(custom_keybind) => Some(custom_keybind.keybind),
            None => None
        };

        let custom_keybind_input_field_state = CustomKeybindInputFieldState {
            previous_shortcut_name: custom_keybind_name.clone(),
            shortcut_name: custom_keybind_name.clone(),
            keybind,
            command
        };
        custom_keybind_input_field.update_state(custom_keybind_input_field_state.clone());
        custom_keybind_input_field.update_ui(custom_keybind_input_field_state.clone());
        custom_keybind_input_field.init_events();

        let custom_keybind_input_field_rc = Rc::new(RefCell::new(custom_keybind_input_field));
        let editable_control_element_state = EditableControlElementState {
            edit_mode
        };
        let mut editable_control_element = EditableControlElement::new(custom_keybind_input_field_rc.clone());
        editable_control_element.update_ui(editable_control_element_state.clone());
        editable_control_element.update_state(editable_control_element_state.clone());

        let editable_control_element_rc = Rc::new(
            RefCell::new(editable_control_element
        ));

        let editable_control_element_manager =
            EditableControlElementManager::new(editable_control_element_rc.clone(), application_provider.clone());
        editable_control_element_rc.borrow_mut().init_events(editable_control_element_manager);

        let custom_keybind_entries_box_clone = custom_keybind_entries_box.clone();
        let editable_control_element_clone = editable_control_element_rc.clone();
        let custom_keybind_entry_field_clone = custom_keybind_input_field_rc.clone();
        let application_provider_clone = application_provider.clone();
        let delete_button_click = move |_: &Button| {
            custom_keybind_entries_box_clone.remove(editable_control_element_clone.borrow().get_widget());
            custom_keybind_entry_field_clone.borrow().remove_settings(application_provider_clone.clone());
        };
        custom_keybind_input_field_rc.borrow().set_delete_button_callback(delete_button_click);
        custom_keybind_entries_box.append(editable_control_element_rc.borrow().get_widget());
    }

    fn create_keybind_input_field(
        application_provider: &ApplicationProvider, entry_field_name: String, system_keybind: SystemKeybind
    ) -> KeybindInputField {
        let keybind_entry_changed_callback = Self::create_keybind_input_field_change(
            application_provider.clone(), system_keybind.clone()
        );

        let keybind_provider = application_provider.get_keybinds_provider();
        let program_keybind = keybind_provider.borrow()
            .get_keybind(system_keybind.clone());

        let mut keybind_iput_field = KeybindInputField::new();
        let state = KeybindInputFieldState {
            input_text: entry_field_name,
            configuration: program_keybind,
        };
        keybind_iput_field.update_ui(state);
        keybind_iput_field.set_input_callback(keybind_entry_changed_callback);
        keybind_iput_field
    }

    fn create_keybind_input_field_change(
        application_provider: ApplicationProvider,
        system_keybind: SystemKeybind
    ) -> impl Fn(KeyBindConfiguration) {
        let keybind_provider = application_provider.get_keybinds_provider();
        let input_field_change = move |keybind_configuration: KeyBindConfiguration| {
            keybind_provider.borrow_mut()
                .set_keybind(system_keybind.clone(), keybind_configuration.clone());
        };
        input_field_change
    }
}