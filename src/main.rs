pub mod ui;
pub mod providers;
pub mod models;
pub mod utils;
pub mod persistence;
mod types;
mod math;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CssProvider, STYLE_PROVIDER_PRIORITY_USER};
use gtk::gdk::Display;
use gtk::gio::File;
use crate::ui::controls::Control;
use crate::ui::pages::app::App;

fn main() {
    // GTK application initialization
    let application = Application::builder()
        .application_id("jinoworks.hyprsettings")
        .build();
    
    application.connect_activate(application_activation_setup);
    application.run();
}

fn application_activation_setup(application: &Application) {
    load_css_styles();
    
    let window = ApplicationWindow::builder()
        .application(application)
        .title("HyprSettings")
        .default_width(800)
        .default_height(600)
        .build();
    
    let app = App::new();
    window.set_child(Some(app.get_widget()));
    window.present();
}

fn load_css_styles() {
    const CSS_STYLE_PATH: &str = "/usr/share/hyprsettings/style.css";

    let provider = CssProvider::new();
    let css_file = File::for_path(CSS_STYLE_PATH);
    provider.load_from_file(&css_file);

    let display = Display::default().expect("Could not get default display");
    gtk::style_context_add_provider_for_display(&display, &provider, STYLE_PROVIDER_PRIORITY_USER);
}