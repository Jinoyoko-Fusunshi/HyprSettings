use gtk::{Adjustment, Align, Label, Orientation, SpinButton as GTKSpinButton};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::ui::component::Component;
use crate::ui::labeled_component::LabeledComponent;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_component::UpdatableComponent;

pub struct SpinButton {
    spin_button_box: gtk::Box,
    spin_button_label: Label,
    spin_button: GTKSpinButton,
}

impl Component for SpinButton {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.spin_button_box
    }
}

impl UpdatableComponent<SpinButtonState> for SpinButton {
    fn update_ui(&mut self, state: SpinButtonState) {
        self.spin_button_label.set_text(state.label_text.as_str());

        let adjustment = Adjustment::new(
            state.current_value,
            state.min_value,
            state.max_value,
            state.increment_value,
            state.page_increment_value,
            state.page_size,
        );
        self.spin_button.set_adjustment(&adjustment);
        self.spin_button.set_climb_rate(state.climb_rate);
        self.spin_button.set_digits(state.digit_count);
        self.spin_button.set_numeric(state.use_integral_numbers);
    }
}

impl LabeledComponent for SpinButton {
    fn set_text_width(&self, width: u32) {
        self.spin_button_label.set_width_request(width as i32);
    }
}

impl SpinButton {
    pub fn new() -> SpinButton {
        let spin_button_box = gtk::Box::new(Orientation::Horizontal, 10);
        let spin_button = GTKSpinButton::new(None::<&Adjustment>, 0.0, 0);
        spin_button.set_wrap(true);

        let spin_button_label = Label::new(None);
        spin_button_label.set_halign(Align::Start);
        spin_button_label.set_xalign(0.0);
        
        spin_button_box.append(&spin_button_label);
        spin_button_box.append(&spin_button);
        
        Self {
            spin_button_box,
            spin_button_label,
            spin_button
        }
    }

    pub fn set_value_change(&self, change_callback: impl Fn(&GTKSpinButton) + 'static) {
        self.spin_button.connect_value_changed(change_callback);
    }
}