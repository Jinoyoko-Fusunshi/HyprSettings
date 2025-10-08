use crate::ui::css_styler::CSSStyler;
use crate::ui::manager::control_manager::ControlManager;
use crate::utils::RcMut;

#[derive(Clone)]
pub struct CSSStylerManager {
    css_styler: RcMut<CSSStyler>,
}

pub enum CSSStylerManagerEvent {
    ThemeChanged,
}

impl ControlManager<CSSStyler, CSSStylerManagerEvent> for CSSStylerManager {
    fn send_event(&self, event: CSSStylerManagerEvent) {
        match event {
            CSSStylerManagerEvent::ThemeChanged => {
                self.css_styler.borrow().apply_current_style_settings();
            }
        }
    }

    fn get_control(&self) -> RcMut<CSSStyler> {
        self.css_styler.clone()
    }
}

impl CSSStylerManager {
    pub fn new(css_styler: RcMut<CSSStyler>) -> Self {
        Self {
            css_styler
        }
    }
}