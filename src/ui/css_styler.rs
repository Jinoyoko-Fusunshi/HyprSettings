use gtk::{CssProvider, InterfaceColorScheme, Settings, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk::gdk::Display;
use crate::ui::managed_control::ManagedControl;
use crate::ui::manager::control_manager::ControlManager;
use crate::ui::manager::css_styler_manager::{CSSStylerManager, CSSStylerManagerEvent};
use crate::utils::{new_rc_mut, RcMut};

pub struct CSSStyler {
    settings: RcMut<Settings>,
}

impl ManagedControl<CSSStylerManager> for CSSStyler {
    fn init_events_by_manager(&self, manager: CSSStylerManager) {
        let manager_clone = manager.clone();
        self.settings.borrow().connect_gtk_interface_color_scheme_notify(move |_: &Settings| {
            manager_clone.send_event(CSSStylerManagerEvent::ThemeChanged);
        });

        let manager_clone = manager.clone();
        self.settings.borrow().connect_gtk_theme_name_notify(move |_: &Settings| {
            manager_clone.send_event(CSSStylerManagerEvent::ThemeChanged);
        });
    }
}

impl CSSStyler{
    const LIGHT_STYLE_FILE: &'static str = "light-style.css";
    const DARK_STYLE_FILE: &'static str = "dark-style.css";

    pub fn new() -> Self {
        let settings = new_rc_mut(Settings::default().expect("Could not get default GTK settings"));
        Self {
            settings,
        }
    }

    fn get_full_style_path(style_file: &str) -> String {
        let shared_application_path = std::path::Path::new("/usr/share/hyprsettings");
        let style_path = shared_application_path.join(style_file);
        if let Some(style_path) = style_path.to_str() {
            style_path.to_string()
        } else {
            "".to_string()
        }
    }

    pub fn apply_current_style_settings(&self) {
        Self::apply_style_settings(&self.settings.borrow());
    }

    pub fn apply_style_settings(settings: &Settings) {
        let style_path = if Self::is_dark_theme_active(settings) {
            Self::get_full_style_path(Self::DARK_STYLE_FILE)
        } else {
            Self::get_full_style_path(Self::LIGHT_STYLE_FILE)
        };

        let provider = CssProvider::new();
        provider.load_from_path(style_path);

        let display = Display::default().expect("Could not get default display");
        gtk::style_context_add_provider_for_display(&display, &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    fn is_dark_theme_active(settings: &Settings) -> bool {
        if Self::has_dark_interface_color_scheme(settings) {
            return true;
        }

        if Self::has_dark_theme(&settings) {
            return true;
        }

        false
    }

    fn has_dark_interface_color_scheme(settings: &Settings) -> bool {
        match settings.gtk_interface_color_scheme() {
            InterfaceColorScheme::Dark => true,
            _ => false,
        }
    }

    fn has_dark_theme(settings: &Settings) -> bool {
        match settings.gtk_theme_name() {
            Some(theme_name) => theme_name.contains("dark"),
            None => false,
        }
    }
}