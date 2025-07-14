use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Stack, Box, Orientation};
use gtk::gdk::Display;
use gtk::gio::File;

type ButtonCallback = fn(&Button);

fn main() {
    // Panel names
    const GENERAL_PANEL_NAME: &str = "general-panel";
    const APPEARANCE_PANEL_NAME: &str = "appearance-panel";
    const PROGRAMS_PANEL_NAME: &str = "programs-panel";
    const KEYBINDS_PANEL_NAME: &str = "keybinds-panel";
    const INFO_PANEL_NAME: &str = "info-panel";


    // GTK application initialization
    let application = Application::builder()
        .application_id("de.jinoworks.HyprSettings")
        .build();

    // GTK application start
    application.connect_activate(|app| {
        // Load css style files
        load_css_styles();

        // Create the main window application
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hypr Settings")
            .default_width(800)
            .default_height(600)
            .build();

        let category_panels = Stack::new();
        let window_container = Box::new(Orientation::Horizontal, 10);
        window_container.set_margin_start(10);
        window_container.set_margin_end(10);
        window_container.set_margin_top(10);
        window_container.set_margin_bottom(10);

        let category_buttons = Box::new(Orientation::Vertical, 10);
        category_buttons.set_width_request(320);

        let category_content = Box::new(Orientation::Vertical, 10);
        category_content.set_hexpand(true);

        // The navigation buttons to toggle the individual category panel
        let general_button = create_category_button("general", GENERAL_PANEL_NAME, &category_panels);
        let appearance_button = create_category_button("appearance", APPEARANCE_PANEL_NAME, &category_panels);
        let programs_button = create_category_button("programs", PROGRAMS_PANEL_NAME, &category_panels);
        let keybinds_button = create_category_button("keybinds", KEYBINDS_PANEL_NAME, &category_panels);
        let info_button = create_category_button("info", INFO_PANEL_NAME, &category_panels);

        category_buttons.append(&general_button);
        category_buttons.append(&appearance_button);
        category_buttons.append(&programs_button);
        category_buttons.append(&keybinds_button);
        category_buttons.append(&info_button);

        // The category panels to be individually displayed
        let general_panel = create_panel("General Panel");
        let appearance_panel = create_panel("Appearance Panel");
        let programs_panel = create_panel("Programs Panel");
        let keybinds_panel = create_panel("Keybinds Panel");
        let info_panel = create_panel("Info Panel");

        category_panels.add_named(&general_panel, Some(GENERAL_PANEL_NAME));
        category_panels.add_named(&appearance_panel, Some(APPEARANCE_PANEL_NAME));
        category_panels.add_named(&programs_panel, Some(PROGRAMS_PANEL_NAME));
        category_panels.add_named(&keybinds_panel, Some(KEYBINDS_PANEL_NAME));
        category_panels.add_named(&info_panel, Some(INFO_PANEL_NAME));

        // Adding GTK UI controls to the window container
        category_content.append(&category_panels);
        window_container.append(&category_buttons);
        window_container.append(&category_content);
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

fn create_panel(title: &str) -> Box {
    let panel = Box::new(Orientation::Vertical, 0);
    let info_button = create_button::<ButtonCallback>(title, None);
    panel.append(&info_button);
    panel
}

fn create_category_button(title: &str, click_element_name: &'static str, category_panels: &Stack) -> Button {
    let category_panels_closure_clone = category_panels.clone();
    let navigation_button_callback = move |_: &Button| {
        category_panels_closure_clone.set_visible_child_name(&click_element_name);
    };

    let category_button = create_button(title, Some(navigation_button_callback));
    category_button.set_height_request(48);
    category_button
}

fn create_button<F>(title: &str, click_action: Option<F>) -> Button
where F: Fn(&Button) + 'static
{
    let navigation_button = Button::with_label(title);
    if let Some(callback) = click_action{
        navigation_button.connect_clicked(callback);
    }
    navigation_button
}