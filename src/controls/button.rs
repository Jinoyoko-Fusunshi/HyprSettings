use gtk::Button;
use gtk::prelude::ButtonExt;

pub fn create_button<F>(title: &str, click_action: Option<F>) -> Button
where F: Fn(&Button) + 'static
{
    let navigation_button = Button::with_label(title);
    if let Some(callback) = click_action{
        navigation_button.connect_clicked(callback);
    }
    navigation_button
}