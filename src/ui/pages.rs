use gtk::{Orientation, PolicyType, ScrolledWindow};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;

pub mod app;
pub mod user_programs;
pub mod monitors;
pub mod infos;
pub mod keybinds;
pub mod appearance;
pub mod startup_programs;
pub mod overview;
pub mod wallpaper;
pub mod lockscreen;
pub mod input;

pub const OVERVIEW_PAGE: &str = "overview-page";
pub const USER_PROGRAMS_PAGE: &str = "user-programs-page";
pub const MONITORS_PAGE: &str = "monitors-page";
pub const WALLPAPER_PAGE: &str = "wallpaper-page";
pub const LOCKSCREEN_PAGE: &str = "lockscreen-page";
pub const APPEARANCE_PAGE: &str = "appearance-page";
pub const STARTUP_PROGRAMS_PAGE: &str = "startup-page";
pub const INPUT_PAGE: &str = "input-page";
pub const KEYBINDS_PAGE: &str = "keybinds-page";
pub const INFO_PAGE: &str = "info-page";

pub struct Pages;

impl Pages {
    pub fn create_page_layout_box(id_name: &str) -> GTKBox {
        let page_layout_box = BoxBuilder::new(format!("{id_name}_layout").as_str())
            .set_orientation(Orientation::Vertical)
            .set_full_height(true)
            .set_full_width(true)
            .build();

        page_layout_box
    }

    pub fn create_and_attach_page_content_box(id_name: &str, page_layout_box: &GTKBox) -> GTKBox {
        let page_content_box = BoxBuilder::new(id_name)
            .set_orientation(Orientation::Vertical)
            .set_margin(DEFAULT_MARGIN)
            .set_full_height(true)
            .set_full_width(true)
            .build();

        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_widget_name(id_name);
        scrolled_window.set_vexpand(true);
        scrolled_window.set_hexpand(true);
        scrolled_window.set_policy(PolicyType::Automatic, PolicyType::Automatic);
        scrolled_window.set_child(Some(&page_content_box));

        page_layout_box.append(&scrolled_window);
        page_content_box
    }
}