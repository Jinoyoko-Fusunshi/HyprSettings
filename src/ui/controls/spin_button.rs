use gtk::{Adjustment, Align, Label, Orientation, SpinButton as GTKSpinButton};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::Control;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::states::spin_button_state::SpinButtonState;
use crate::ui::updatable_control::UpdatableControl;

pub struct SpinButton {
    state: SpinButtonState,
    spin_button_box: GTKBox,
    spin_button_label: Label,
    spin_button: GTKSpinButton,
}

impl Control for SpinButton {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.spin_button_box
    }
}

impl UpdatableControl<SpinButtonState> for SpinButton {
    fn update_state(&mut self, state: SpinButtonState) {
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

        self.state = state;
    }

    fn get_current_state(&self) -> SpinButtonState {
        self.state.clone()
    }
}

impl ActivableControl for SpinButton {
    fn enable_control(&self) {
        self.spin_button.set_sensitive(true);
    }

    fn disable_control(&self) {
        self.spin_button.set_sensitive(false);
    }
}

impl LabeledControl for SpinButton {
    fn set_text_width(&self, width: u32) {
        self.spin_button_label.set_width_request(width as i32);
    }
}

impl SpinButton {
    pub fn new() -> SpinButton {
        let spin_button_box = BoxBuilder::new("spinbutton")
            .set_orientation(Orientation::Horizontal)
            .build();

        let spin_button = GTKSpinButton::new(None::<&Adjustment>, 0.0, 0);
        spin_button.set_wrap(true);

        let spin_button_label = Label::new(None);
        spin_button_label.set_halign(Align::Start);
        spin_button_label.set_xalign(0.0);
        
        spin_button_box.append(&spin_button_label);
        spin_button_box.append(&spin_button);

        let state = Default::default();

        Self {
            state,
            spin_button_box,
            spin_button_label,
            spin_button
        }
    }

    pub fn set_value_change(&self, change_callback: impl Fn(&GTKSpinButton) + 'static) {
        self.spin_button.connect_value_changed(change_callback);
    }
}