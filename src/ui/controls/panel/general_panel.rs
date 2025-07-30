use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Entry, Label, Orientation, Separator};
use gtk::prelude::{BoxExt, EditableExt, WidgetExt};
use crate::ui::controls::{named_section::named_input_section::NamedInputSection, panel::Panel};
use crate::settings::hyprland_settings::HyprlandSettings;

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
        
        let settings_clone = settings.clone();
        let config_input_section_change = move |input: &Entry| {
            settings_clone.borrow_mut().program_settings
                .insert("HyprLandConfig".to_string(), String::from(input.text()));
        };
        let config_section = NamedInputSection::new(
            "Hyprland config path:",
            "default: ~/.config/hypr/hyprland.cfg",
            Some(config_input_section_change),
        );

        let settings_clone = settings.clone();
        let terminal_input_section_change = move |input: &Entry| {
            settings_clone.borrow_mut().program_settings
                .insert("VirtualTerminal".to_string(), String::from(input.text()));
        };
        let terminal_section = NamedInputSection::new(
            "Virtual terminal program path:",
            "e.g. /usr/bin/alacritty",
            Some(terminal_input_section_change),
        );

        let settings_clone = settings.clone();
        let file_manager_input_section_change = move |input: &Entry| {
            settings_clone.borrow_mut().program_settings
                .insert("FileManager".to_string(), String::from(input.text()));
        };
        let file_manager_section = NamedInputSection::new(
            "File manager program path:",
            "e.g. /usr/bin/nautilus",
            Some(file_manager_input_section_change),
        );

        let settings_clone = settings.clone();
        let quick_search_input_section_change = move |input: &Entry| {
            settings_clone.borrow_mut().program_settings
                .insert("QuickSearch".to_string(), String::from(input.text()));
        };
        let quick_search_section = NamedInputSection::new(
            "Quick search program path:",
            "e.g. /usr/bin/anyrun",
            Some(quick_search_input_section_change),
        );

        let settings_clone = settings.clone();
        let lock_screen_input_section_change = move |input: &Entry| {
            settings_clone.borrow_mut().program_settings
                .insert("LockScreen".to_string(), String::from(input.text()));
        };
        let lock_screen_section = NamedInputSection::new(
            "Lock screen program path:",
            "e.g. /usr/bin/hyprlock",
            Some(lock_screen_input_section_change),
        );

        let settings_clone = settings.clone();
        let notification_handler_input_section_change = move |input: &Entry| {
            settings_clone.borrow_mut().program_settings
                .insert("NotificationHandler".to_string(), String::from(input.text()));
        };
        let notification_handler_section = NamedInputSection::new(
            "Notification handler program path:",
            "e.g. /usr/bin/dryrun",
            Some(notification_handler_input_section_change),
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
}