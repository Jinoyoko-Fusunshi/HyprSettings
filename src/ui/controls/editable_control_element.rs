use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::component::Component;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::statable_component::StatableComponent;
use crate::ui::state_savable_component::StateSavableComponent;
use crate::ui::updatable_component::UpdatableComponent;

#[derive(Clone)]
pub enum EditMode {
    Locked,
    Edit
}

pub enum EditableControlElementEvent {
    ChangeEditMode(EditMode)
}

pub struct EditableControlElementManager<Element: ActivableControl + Component + StateSavableComponent + 'static> {
    editable_control_element: Rc<RefCell<EditableControlElement<Element>>>,
    application_provider: ApplicationProvider
}

impl<Element: ActivableControl + Component +  StateSavableComponent + 'static> EditableControlElementManager<Element> {
    pub fn new(
        editable_control_element: Rc<RefCell<EditableControlElement<Element>>>,
        application_provider: ApplicationProvider
    ) -> Self {
        Self {
            editable_control_element,
            application_provider
        }
    }

    pub fn send_event(&self, event: EditableControlElementEvent) {
        let application_provider = self.application_provider.clone();
        match event {
            EditableControlElementEvent::ChangeEditMode(mode) => {
                let mut editable_control_element_mut = self.editable_control_element.borrow_mut();
                editable_control_element_mut.change_mode(mode.clone());
                editable_control_element_mut.update_state(EditableControlElementState {
                    edit_mode: mode.clone()
                });

                if let EditMode::Locked = mode {
                    editable_control_element_mut.editable_control.borrow()
                        .save_settings(application_provider);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct EditableControlElementState {
    pub edit_mode: EditMode
}

pub struct EditableControlElement<Element: ActivableControl + Component + StateSavableComponent> {
    editable_control_element_box: gtk::Box,
    editable_control: Rc<RefCell<Element>>,
    mode_toggle_button: Button,
    state: Rc<RefCell<EditableControlElementState>>,
}

impl<Element: ActivableControl + Component + StateSavableComponent> Component for EditableControlElement<Element> {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.editable_control_element_box
    }
}

impl<Element: ActivableControl + Component + StateSavableComponent>
UpdatableComponent<EditableControlElementState> for EditableControlElement<Element> {
    fn update_ui(&mut self, state: EditableControlElementState) {
        self.change_mode(state.edit_mode);
    }
}

impl<Element: ActivableControl + Component + StateSavableComponent>
StatableComponent<EditableControlElementState> for EditableControlElement<Element> {
    fn update_state(&mut self, state: EditableControlElementState) {
        *self.state.borrow_mut() = state;
    }
}

impl<Element: ActivableControl + Component + StateSavableComponent> EditableControlElement<Element> {
    pub fn new(editable_control: Rc<RefCell<Element>>) -> Self {
        let editable_control_element_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let mode_toggle_button = Button::with_label("✅");
        mode_toggle_button.set_vexpand(false);
        mode_toggle_button.set_valign(Align::Center);

        editable_control_element_box.append(editable_control.borrow().get_widget());
        editable_control_element_box.append(&mode_toggle_button);

        let state = Rc::new(RefCell::new(EditableControlElementState {
            edit_mode: EditMode::Edit
        }));

        Self {
            editable_control_element_box,
            editable_control,
            mode_toggle_button,
            state,
        }
    }

    pub fn init_events(&self, editable_control_element_manager: EditableControlElementManager<Element>) {
        let state = self.state.clone();
        self.mode_toggle_button.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let edit_mode = state_ref.edit_mode.clone();
            drop(state_ref);

            let new_edit_mode = match edit_mode {
                EditMode::Locked => EditMode::Edit,
                EditMode::Edit => EditMode::Locked,
            };

            editable_control_element_manager.send_event(
                EditableControlElementEvent::ChangeEditMode(new_edit_mode)
            );
        });
    }

    pub fn change_mode(&self, mode: EditMode) {
        match mode {
            EditMode::Edit => {
                self.editable_control.borrow().enable_control();
                self.mode_toggle_button.set_label("✅");
            }
            EditMode::Locked => {
                self.editable_control.borrow().disable_control();
                self.mode_toggle_button.set_label("✏️");
            }
        }
    }
}