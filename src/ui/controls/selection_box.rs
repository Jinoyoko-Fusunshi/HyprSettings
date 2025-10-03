use gtk::{Align, ComboBoxText, Label, Orientation};
use gtk::glib::GString;
use gtk::prelude::{BoxExt, ComboBoxExt, ComboBoxExtManual, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::states::selection_box_state::SelectionBoxState;
use crate::ui::updatable_control::UpdatableControl;

pub struct SelectionBox {
    state: SelectionBoxState,
    selection_box: GTKBox,
    selection_label: Label,
    selection_combobox: ComboBoxText,
}

impl Control for SelectionBox {
    fn get_widget(&self) -> &GTKBox {
        &self.selection_box
    }
}

impl LabeledControl for SelectionBox {
    fn set_text_width(&self, width: u32) {
        self.selection_label.set_width_request(width as i32);
    }
}

impl UpdatableControl<SelectionBoxState> for SelectionBox {
    fn update_state(&mut self, state: SelectionBoxState) {
        self.selection_label.set_text(&state.label_text);

        self.set_items(state.options.clone());

        if let Some(selected_option) = state.selected_option.clone() {
            self.set_selected_by_name(selected_option)
        }

        self.state = state;
    }

    fn get_current_state(&self) -> SelectionBoxState {
        self.state.clone()
    }
}

impl ActivableControl for SelectionBox {
    fn enable_control(&self) {
        self.selection_box.set_sensitive(true);
    }

    fn disable_control(&self) {
        self.selection_box.set_sensitive(false);
    }
}

impl SelectionBox {
    pub fn new() -> SelectionBox {
        let selection_box = BoxBuilder::new("selection-box")
            .set_orientation(Orientation::Horizontal)
            .build();

        let selection_label = Label::new(None);
        selection_label.set_halign(Align::Start);
        selection_label.set_xalign(0.0);
        
        let selection_combobox = ComboBoxText::new();
        selection_combobox.set_width_request(180);

        selection_box.append(&selection_label);
        selection_box.append(&selection_combobox);

        let state = SelectionBoxState {
            label_text: "".to_string(),
            selected_option: None,
            options: vec![],
        };

        Self {
            state,
            selection_label,
            selection_box,
            selection_combobox
        }
    }

    pub fn parse_selection_as_bool(selection: Option<GString>) -> bool {
        if let Some(text) = selection {
            text.to_string()
                .parse::<bool>()
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn set_selection_change(&self, selection_change: impl Fn(&ComboBoxText) + 'static) {
        self.selection_combobox.connect_changed(selection_change);
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.selection_combobox.remove_all();
        
        for item in items {
            self.selection_combobox.append_text(item.as_str());
        }
    }

    pub fn set_selected_by_name(&mut self, item: String) {
        let index = self.state.options
            .iter()
            .position(|option| *option == item.clone());

        if let Some(index) = index {
            self.selection_combobox.set_active(Some(index as u32));
        } else {
            self.selection_combobox.set_active(Some(0));
        }
    }
}