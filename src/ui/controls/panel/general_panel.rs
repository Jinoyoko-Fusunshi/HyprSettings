use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Entry, Label, Orientation, Separator};
use gtk::prelude::{BoxExt, EditableExt, WidgetExt};
use crate::ui::controls::{named_section::named_input_section::NamedInputSection, panel::Panel};
use crate::settings::hyprland_settings::{
    HyprlandSettings, FILE_MANAGER_ENTRY, HYPRLAND_CONFIG_ENTRY, LOCK_SCREEN_ENTRY,
    NOTIFICATION_HANDLER_ENTRY, QUICK_SEARCH_ENTRY, VIRTUAL_TERMINAL_ENTRY
};

pub struct GeneralPanel {
    widget: gtk::Box
}

impl Panel for GeneralPanel {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.widget
    }
}

impl Clone for GeneralPanel {
    fn clone(&self) -> Self {
        Self {
            widget: self.widget.clone()
        }
    }   
}

impl GeneralPanel{
    pub fn new(settings: &Rc<RefCell<HyprlandSettings>>) -> Self {
        const PROGRAMS_LABEL: &str = "Programs";
        
        let general_panel_box = gtk::Box::new(Orientation::Vertical, 10);
        general_panel_box.set_margin_top(10);
        general_panel_box.set_margin_bottom(10);
        general_panel_box.set_margin_start(10);
        general_panel_box.set_margin_end(10);

        let programs_label = Label::new(Some(PROGRAMS_LABEL));
        let separator = Separator::new(Orientation::Horizontal);
        let config_section = GeneralPanel::create_named_input_section(
            "Hyprland config path:",
            "default: ~/.config/hypr/hyprland.cfg",
            &settings,
            HYPRLAND_CONFIG_ENTRY,
        );
        let terminal_section = GeneralPanel::create_named_input_section(
            "Virtual terminal program path:",
            "e.g. /usr/bin/alacritty",
            &settings,
            VIRTUAL_TERMINAL_ENTRY,
        );
        let file_manager_section = GeneralPanel::create_named_input_section(
            "File manager program path:",
            "e.g. /usr/bin/nautilus",
            &settings,
            FILE_MANAGER_ENTRY,
        );
        let quick_search_section = GeneralPanel::create_named_input_section(
            "Quick search program path:",
            "e.g. /usr/bin/anyrun",
            &settings,
            QUICK_SEARCH_ENTRY,
        );
        let lock_screen_section = GeneralPanel::create_named_input_section(
            "Lock screen program path:",
            "e.g. /usr/bin/hyprlock",
            &settings,
            LOCK_SCREEN_ENTRY,
        );
        let notification_handler_section = GeneralPanel::create_named_input_section(
            "Notification handler program path:",
            "e.g. /usr/bin/dryrun",
            &settings,
            NOTIFICATION_HANDLER_ENTRY,
        );

        general_panel_box.append(&programs_label);
        general_panel_box.append(&separator);
        general_panel_box.append(config_section.get_container_box());
        general_panel_box.append(terminal_section.get_container_box());
        general_panel_box.append(file_manager_section.get_container_box());
        general_panel_box.append(quick_search_section.get_container_box());
        general_panel_box.append(lock_screen_section.get_container_box());
        general_panel_box.append(notification_handler_section.get_container_box());

        Self {
            widget: general_panel_box
        }
    }

    fn create_named_input_section(
        label_text: &str, input_placeholder_text: &str,
        settings: &Rc<RefCell<HyprlandSettings>>, program_name: &str
    ) -> NamedInputSection {
        let input_section_change = {
            let settings_clone = settings.clone();
            let program_name_string = program_name.to_string();
            move |input: &Entry| {
                settings_clone.borrow_mut().program_settings
                    .insert(program_name_string.clone(), input.text().to_string());
            }
        };

        let settings_clone = settings.clone();
        let hyprland_settings = settings_clone.borrow();
        let input_text = hyprland_settings.program_settings.get(program_name).cloned();

        let input_section = NamedInputSection::new(
            label_text,
            input_placeholder_text,
            input_text
        );
        input_section.set_input_callback(input_section_change);
        input_section
    }
}