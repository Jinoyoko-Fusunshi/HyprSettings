use gtk::{Align, DropDown, Expression, Label, Orientation, StringObject};
use gtk::gio::ListStore;
use gtk::prelude::{BoxExt, Cast, ListModelExt, WidgetExt};
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
    selection_dropdown: DropDown,
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

        let dropdown_model = ListStore::new::<StringObject>();
        let selection_dropdown = DropDown::new(None::<ListStore>, None::<Expression>);
        selection_dropdown.set_width_request(180);
        selection_dropdown.set_model(Some(&dropdown_model));

        selection_box.append(&selection_label);
        selection_box.append(&selection_dropdown);

        let state = SelectionBoxState {
            label_text: "".to_string(),
            selected_option: None,
            options: vec![],
        };

        Self {
            state,
            selection_label,
            selection_box,
            selection_dropdown
        }
    }

    pub fn get_selected_option_as_bool(dropdown: &DropDown) -> bool {
        let selected_text = Self::get_selected_option(dropdown);
        selected_text
            .parse::<bool>()
            .unwrap_or(false)
    }

    pub fn get_selected_option(dropdown: &DropDown) -> String {
        let model = dropdown.model().unwrap();
        if let Some(active_item) = model.item(dropdown.selected()) {
            let active_text = active_item.downcast_ref::<StringObject>().unwrap();
            active_text.string().to_string()
        } else {
            "".to_string()
        }
    }

    pub fn set_selection_change(&self, selection_change: impl Fn(&DropDown) + 'static) {
        self.selection_dropdown.connect_selected_notify(selection_change);
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        if let Some(model) = self.selection_dropdown.model() {
            if let Some(list) = model.downcast_ref::<ListStore>() {
                list.remove_all();

                for item in items {
                    list.append(&StringObject::new(item.as_str()));
                }
            }
        }
    }

    pub fn set_selected_by_name(&mut self, selected_item: String) {
        if let Some(model) = self.selection_dropdown.model() {
            if let Some(list) = model.downcast_ref::<ListStore>() {
                for item_index in 0..list.n_items() {
                    if let Some(item) = list.item(item_index) {
                        if let Some(item_text) = item.downcast_ref::<StringObject>() {
                            if item_text.string().to_string() == selected_item {
                                self.selection_dropdown.set_selected(item_index);
                                return;
                            }
                        }
                    }
                }
            }
        }

        self.selection_dropdown.set_selected(0);
    }
}