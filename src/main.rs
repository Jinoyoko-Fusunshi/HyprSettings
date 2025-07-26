mod settings;
mod ui;

use std::cell::RefCell;
use std::collections::HashMap;
use std::process::Command;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation, Stack};
use gtk::gdk::Display;
use gtk::gio::File;
use ui::controls::button::create_button;
use ui::controls::panel::{appearance_panel, display_panel, general_panel, info_panel, key_binds_panel, startup_programs_panel, Panel};
use settings::hyprland_settings::HyprlandSettings;
use settings::monitor::monitor_info_parser::MonitorInfoParser;
use settings::monitor::monitor_configuration::MonitorConfiguration;
use ui::css_styles::CSSStyles;


// Panel names
const GENERAL_PANEL_NAME: &str = "general-panel";
const DISPLAY_PANEL_NAME: &str = "display-panel";
const APPEARANCE_PANEL_NAME: &str = "appearance-panel";
const STARTUP_PROGRAM_PANEL_NAME: &str = "startup-panel";
const KEYBINDS_PANEL_NAME: &str = "keybinds-panel";
const INFO_PANEL_NAME: &str = "info-panel";

fn main() {
    // GTK application initialization
    let application = Application::builder()
        .application_id("de.jinoworks.HyprSettings")
        .build();

    // GTK application start
    application.connect_activate(|app| {
        // Load CSS style files
        load_css_styles();

        // Create the main window application
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hypr Settings")
            .default_width(800)
            .default_height(600)
            .build();

        // The settings container being updated by the individual panels
        let settings = Rc::new(RefCell::new(HyprlandSettings::new()));

        let output = Command::new("wlr-randr")
            .output()
            .expect("Error during wlrandr execution");

        let output_string = String::from_utf8(output.stdout)
            .expect("Failed to parse wlr-randr output");

        let mut monitor_info_parser = MonitorInfoParser::new();
        monitor_info_parser.parse_output(&output_string);
        let monitor_information = monitor_info_parser.get_result();

        let max_monitor_configurations = monitor_information
            .iter()
            .map(|monitor_information| {
                MonitorConfiguration {
                    enabled: true,
                    information: monitor_information.clone(),
                    video_mode: monitor_information.max_video_mode.clone()
                }
            })
            .collect::<Vec<MonitorConfiguration>>();

        settings.borrow_mut().monitor_configurations = max_monitor_configurations;

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

        let settings_clone = settings.clone();
        let save_button_click_callback = move |_: &Button| {
            println!("{:?}", settings_clone.borrow())
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
    });

    // Run the entire GTK application
    application.run();
}

fn load_css_styles() {
    let provider = gtk::CssProvider::new();
    let css_file = File::for_path("res/style.css");
    provider.load_from_file(&css_file);

    let display = Display::default().expect("Could not get default display");
    gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
}

struct CategoryContentHandler {
    category_panels_map: HashMap<String, Box<dyn Panel>>,
    category_boxes_stack: Stack,
}

impl CategoryContentHandler {
    pub fn new(settings: &Rc<RefCell<HyprlandSettings>>) -> Self {
        // The category panels to be individually displayed
        let general_panel = Box::new(general_panel::GeneralPanel::new(settings));
        let display_panel = Box::new(display_panel::DisplayPanel::new(settings));
        let appearance_panel = Box::new(appearance_panel::AppearancePanel::new(settings));
        let startup_programs_panel = Box::new(startup_programs_panel::StartupProgramsPanel::new(settings));
        let keybinds_panel = Box::new(key_binds_panel::KeyBindsPanel::new());
        let info_panel = Box::new(info_panel::InfoPanel::new());

        let mut category_panels_map: HashMap<String, Box<dyn Panel>> = HashMap::new();

        category_panels_map.insert(GENERAL_PANEL_NAME.to_string(), general_panel.clone());
        category_panels_map.insert(DISPLAY_PANEL_NAME.to_string(), display_panel.clone());
        category_panels_map.insert(APPEARANCE_PANEL_NAME.to_string(), appearance_panel.clone());
        category_panels_map.insert(STARTUP_PROGRAM_PANEL_NAME.to_string(), startup_programs_panel.clone());
        category_panels_map.insert(KEYBINDS_PANEL_NAME.to_string(), keybinds_panel.clone());
        category_panels_map.insert(INFO_PANEL_NAME.to_string(), info_panel.clone());

        let category_boxes_stack = Stack::new();
        category_boxes_stack.add_named(general_panel.get_widget(), Some(GENERAL_PANEL_NAME));
        category_boxes_stack.add_named(display_panel.get_widget(), Some(DISPLAY_PANEL_NAME));
        category_boxes_stack.add_named(appearance_panel.get_widget(), Some(APPEARANCE_PANEL_NAME));
        category_boxes_stack.add_named(startup_programs_panel.get_widget(), Some(STARTUP_PROGRAM_PANEL_NAME));
        category_boxes_stack.add_named(keybinds_panel.get_widget(), Some(KEYBINDS_PANEL_NAME));
        category_boxes_stack.add_named(info_panel.get_widget(), Some(INFO_PANEL_NAME));

        Self {
            category_panels_map,
            category_boxes_stack
        }
    }

    pub fn select_panel_active(&self, panel_name: &str, settings: &Rc<RefCell<HyprlandSettings>>) {
        if let Some(panel) = self.category_panels_map.get(panel_name) {
            panel.reload_settings(settings)
        }

        self.category_boxes_stack.set_visible_child_name(panel_name);
    }

    pub fn get_panels_stack(&self) -> &Stack {
        &self.category_boxes_stack
    }
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