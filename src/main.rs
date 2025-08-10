mod settings;
mod ui;
mod category_content_handler;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation, STYLE_PROVIDER_PRIORITY_USER};
use gtk::gdk::Display;
use gtk::gio::File;
use ui::controls::button::create_button;
use settings::hyprland_settings::HyprlandSettings;
use settings::monitor::{monitor_info_parser::MonitorInfoParser, monitor_configuration::MonitorConfiguration};
use ui::css_styles::CSSStyles;
use crate::category_content_handler::{
    CategoryContentHandler, APPEARANCE_PANEL_NAME, DISPLAY_PANEL_NAME, GENERAL_PANEL_NAME,
    INFO_PANEL_NAME, KEYBINDS_PANEL_NAME, STARTUP_PROGRAM_PANEL_NAME
};
use crate::settings::config_files::settings_reader::SettingsReader;
use crate::settings::config_files::settings_writer::SettingsWriter;
use crate::settings::config_files::yaml_settings_reader::YamlSettingsReader;
use crate::settings::config_files::yaml_settings_writer::YamlSettingsWriter;

fn main() {
    // GTK application initialization
    let application = Application::builder()
        .application_id("de.jinoworks.HyprSettings")
        .build();
    
    application.connect_activate(application_activation_setup);
    application.run();
}

fn application_activation_setup(application: &Application) {
    // Load CSS style files
    load_css_styles();

    // Create the main window application
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Hypr Settings")
        .default_width(800)
        .default_height(600)
        .build();

    // The settings container being updated by the individual panels
    let settings = Rc::new(RefCell::new(HyprlandSettings::new()));
    load_settings(&settings);
    
    

    // Basic window layout of a navigation and content panel
    let window_container = gtk::Box::new(Orientation::Horizontal, 10);
    window_container.set_margin_start(10);
    window_container.set_margin_end(10);
    window_container.set_margin_top(10);
    window_container.set_margin_bottom(10);

    let category_buttons_box = gtk::Box::new(Orientation::Vertical, 10);
    category_buttons_box.set_width_request(320);
    category_buttons_box.add_css_class(CSSStyles::NAVIGATION_PANEL);

    let category_content_box = gtk::Box::new(Orientation::Vertical, 10);
    category_content_box.set_hexpand(true);
    category_content_box.add_css_class(CSSStyles::CONTENT_PANEL);

    // The navigation buttons to toggle the individual category panel
    let settings_clone = settings.clone();
    let category_panels = Rc::new(RefCell::new(CategoryContentHandler::new(&settings_clone)));
    let general_button = create_category_button("general", GENERAL_PANEL_NAME, &settings_clone, &category_panels);
    let display_button = create_category_button("display", DISPLAY_PANEL_NAME, &settings_clone, &category_panels);
    let appearance_button = create_category_button("appearance", APPEARANCE_PANEL_NAME, &settings_clone, &category_panels);
    let startup_button = create_category_button("startup", STARTUP_PROGRAM_PANEL_NAME, &settings_clone, &category_panels);
    let keybinds_button = create_category_button("keybinds", KEYBINDS_PANEL_NAME, &settings_clone, &category_panels);
    let info_button = create_category_button("info", INFO_PANEL_NAME, &settings_clone, &category_panels);

    let save_button_click_callback = move |_: &Button| {
        let mut toml_writer = YamlSettingsWriter::new(&settings);
        toml_writer.serialize_settings();
        let _ = toml_writer.write_to_config_file();
    };
    let save_button = create_button("Save", Some(save_button_click_callback));
    save_button.set_margin_top(10);
    save_button.add_css_class(CSSStyles::SAVE_BUTTON);

    category_buttons_box.append(&general_button);
    category_buttons_box.append(&display_button);
    category_buttons_box.append(&appearance_button);
    category_buttons_box.append(&startup_button);
    category_buttons_box.append(&keybinds_button);
    category_buttons_box.append(&info_button);
    category_buttons_box.append(&save_button);

    // Adding GTK UI controls to the window container
    category_content_box.append(category_panels.borrow().get_panels_stack());
    window_container.append(&category_buttons_box);
    window_container.append(&category_content_box);
    window.set_child(Some(&window_container));
    window.present();
}

fn load_css_styles() {
    let provider = gtk::CssProvider::new();
    let css_file = File::for_path("res/style.css");
    provider.load_from_file(&css_file);

    let display = Display::default().expect("Could not get default display");
    gtk::style_context_add_provider_for_display(&display, &provider, STYLE_PROVIDER_PRIORITY_USER);
}

fn load_settings(settings: &Rc<RefCell<HyprlandSettings>>) {
    if fs::exists("hyprsettings.yaml").expect("Cannot verify existence of settings file") {
        let mut settings_reader = YamlSettingsReader::new();
        settings_reader.read_from_config();
        settings_reader.apply_settings(settings);
    } else {
        load_monitor_default_settings(settings)
    }
}

fn load_monitor_default_settings(settings: &Rc<RefCell<HyprlandSettings>>) {
    let output = Command::new("wlr-randr")
        .output()
        .expect("Error during wlrandr execution");

    let output_string = String::from_utf8(output.stdout)
        .expect("Failed to parse wlr-randr output");

    let mut monitor_info_parser = MonitorInfoParser::new();
    monitor_info_parser.parse_output(&output_string);
    let monitor_information = monitor_info_parser.get_result();

    let max_monitor_configurations: HashMap<String, MonitorConfiguration> = monitor_information
        .iter()
        .map(|monitor_information| {

            let port = monitor_information.port_name.clone();
            let configuration = MonitorConfiguration {
                enabled: true,
                information: monitor_information.clone(),
                video_mode: monitor_information.max_video_mode.clone()
            };

            (port, configuration)
        })
        .collect();

    settings.borrow_mut().monitor_configurations = max_monitor_configurations;
}

fn create_category_button(
    title: &str, click_element_name: &'static str,
    settings: &Rc<RefCell<HyprlandSettings>>, content_handler: &Rc<RefCell<CategoryContentHandler>>
) -> Button {
    let content_handler_clone = content_handler.clone();
    let settings_clone = settings.clone();
    let navigation_button_callback = move |_: &Button| {
        content_handler_clone.borrow().select_panel_active(click_element_name, &settings_clone);
    };

    let category_button = create_button(title, Some(navigation_button_callback));
    category_button.set_height_request(48);
    category_button
}