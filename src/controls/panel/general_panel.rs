use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Entry, Orientation};
use gtk::prelude::{BoxExt, EditableExt, WidgetExt};
use crate::controls::named_input_section::NamedInputSection;
use crate::controls::panel::Panel;
use crate::settings_container::SettingsContainer;

pub struct GeneralPanel {
    widget: gtk::Box
}

impl Panel for GeneralPanel {
    fn get_widget(&self) -> &gtk::Box {
        &self.widget
    }
}

impl GeneralPanel{
    pub fn new(settings: &Rc<RefCell<SettingsContainer>>) -> Self {
        let widget = gtk::Box::new(Orientation::Vertical, 0);
        widget.set_margin_top(10);
        widget.set_margin_bottom(10);
        widget.set_margin_start(10);
        widget.set_margin_end(10);

        let settings_closure_clone = settings.clone();
        let config_input_section_change = move |input: &Entry| {
            settings_closure_clone.borrow_mut().set_hyprland_config_path(String::from(input.text()));
        };
        let config_section = NamedInputSection::new(
            "Hyprland config path:",
            "default: ~/.config/hypr/hyprland.cfg",
            Some(config_input_section_change),
        );

        let settings_closure_clone = settings.clone();
        let terminal_input_section_change = move |input: &Entry| {
            settings_closure_clone.borrow_mut().set_virtual_terminal_program_path(String::from(input.text()));
        };
        let terminal_section = NamedInputSection::new(
            "Virtual terminal program path:",
            "e.g. /usr/bin/alacritty",
            Some(terminal_input_section_change),
        );

        let settings_closure_clone = settings.clone();
        let file_manager_input_section_change = move |input: &Entry| {
            settings_closure_clone.borrow_mut().set_file_manager_program_path(String::from(input.text()));
        };
        let file_manager_section = NamedInputSection::new(
            "File manager program path:",
            "e.g. /usr/bin/nautilus",
            Some(file_manager_input_section_change),
        );

        let settings_closure_clone = settings.clone();
        let quick_search_input_section_change = move |input: &Entry| {
            settings_closure_clone.borrow_mut().set_quick_search_program_path(String::from(input.text()));
        };
        let quick_search_section = NamedInputSection::new(
            "Quick search program path:",
            "e.g. /usr/bin/anyrun",
            Some(quick_search_input_section_change),
        );

        let settings_closure_clone = settings.clone();
        let lock_screen_input_section_change = move |input: &Entry| {
            settings_closure_clone.borrow_mut().set_lock_screen_program_path(String::from(input.text()));
        };
        let lock_screen_section = NamedInputSection::new(
            "Lock screen program path:",
            "e.g. /usr/bin/hyprlock",
            Some(lock_screen_input_section_change),
        );

        let settings_closure_clone = settings.clone();
        let notification_handler_input_section_change = move |input: &Entry| {
            settings_closure_clone.borrow_mut().set_notification_handler_program_path(String::from(input.text()));
        };
        let notification_handler_section = NamedInputSection::new(
            "Notification handler program path:",
            "e.g. /usr/bin/dryrun",
            Some(notification_handler_input_section_change),
        );

        widget.append(config_section.get_widget());
        widget.append(terminal_section.get_widget());
        widget.append(file_manager_section.get_widget());
        widget.append(quick_search_section.get_widget());
        widget.append(lock_screen_section.get_widget());
        widget.append(notification_handler_section.get_widget());

        Self {
            widget
        }
    }
}