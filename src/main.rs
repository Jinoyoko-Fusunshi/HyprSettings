pub mod ui;
pub mod providers;
pub mod models;
pub mod utils;
pub mod persistence;
mod types;
mod math;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use crate::ui::controls::Control;
use crate::ui::css_styler::CSSStyler;
use crate::ui::managed_control::ManagedControl;
use crate::ui::manager::css_styler_manager::CSSStylerManager;
use crate::ui::pages::app::App;
use crate::utils::new_rc_mut;

fn main() {
    let application = Application::builder()
        .application_id("jinoworks.hyprsettings")
        .build();
    
    application.connect_activate(application_activation_setup);
    application.run();
}

fn application_activation_setup(application: &Application) {
    let css_styler = new_rc_mut(CSSStyler::new());
    css_styler.borrow().apply_current_style_settings();
    
    let css_styler_manager = CSSStylerManager::new(css_styler.clone());
    css_styler.borrow().init_events_by_manager(css_styler_manager);

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