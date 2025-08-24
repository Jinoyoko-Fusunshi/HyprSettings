use gtk::{Align, ComboBoxText, Label, Orientation};
use gtk::prelude::{BoxExt, ComboBoxExt, ComboBoxExtManual, WidgetExt};
use crate::ui::component::Component;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::labeled_component::LabeledComponent;
use crate::ui::statable_component::StatableComponent;
use crate::ui::updatable_component::UpdatableComponent;

#[derive(Clone)]
pub struct SelectionBoxState {
    pub label_text: String,
    pub selected_option: Option<String>,
    pub options: Vec<String>,
}

pub struct SelectionBox {
    state: SelectionBoxState,
    selection_box: gtk::Box,
    selection_label: Label,
    selection_combobox: ComboBoxText,
}

impl Component for SelectionBox {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.selection_box
    }
}

impl LabeledComponent for SelectionBox {
    fn set_text_width(&self, width: u32) {
        self.selection_label.set_width_request(width as i32);
    }
}

impl UpdatableComponent<SelectionBoxState> for SelectionBox {
    fn update_ui(&mut self, state: SelectionBoxState) {
        self.selection_label.set_text(&state.label_text);

        self.set_items(state.options);

        if let Some(selected_option) = state.selected_option {
            self.set_selected_by_name(selected_option)
        }
    }
}

impl StatableComponent<SelectionBoxState> for SelectionBox {
    fn update_state(&mut self, state: SelectionBoxState) {
        self.state = state;
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
        let selection_box = gtk::Box::new(Orientation::Horizontal, 10);

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

    pub fn set_selection_change(&self, selection_change: impl Fn(&ComboBoxText) + 'static) {
        self.selection_combobox.connect_changed(selection_change);
    }

    pub fn set_items(&mut self, items: Vec<String>) {
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