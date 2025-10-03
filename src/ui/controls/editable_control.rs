use std::marker::PhantomData;
use gtk::{Align, Button};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::Control;
use crate::ui::managed_control::ManagedControl;
use crate::ui::manager::control_manager::ControlManager;
use crate::ui::manager::editable_control_manager::{EditableControlEvent, EditableControlManager};
use crate::ui::state_savable_control::StateSavableControl;
use crate::ui::states::editable_control_state::{EditMode, EditableControlState};
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::{new_rc_mut, RcMut};

pub struct EditableControl<
    ControlType: Control + UpdatableControl<ControlState> + ActivableControl + StateSavableControl
        + 'static,
    ControlState: 'static
> {
    _phantom_control_state: PhantomData<ControlState>,

    editable_control_box: GTKBox,
    control: RcMut<ControlType>,
    mode_toggle_button: Button,
    state: RcMut<EditableControlState>,
}

impl<
    ControlType: Control + UpdatableControl<ControlState> + ActivableControl + StateSavableControl
        + 'static,
    ControlState: 'static> Control for EditableControl<ControlType, ControlState
>  {
    fn get_widget(&self) -> &GTKBox {
        &self.editable_control_box
    }
}

impl<
    ControlType: Control + UpdatableControl<ControlState> + ActivableControl + StateSavableControl
        + 'static,
    ControlState: 'static
>
ManagedControl<EditableControlManager<ControlType, ControlState>> for EditableControl<ControlType, ControlState> {
    fn init_events_by_manager(&self, manager: EditableControlManager<ControlType, ControlState>) {
        self.mode_toggle_button.connect_clicked(move |_| {
            let control = manager.get_editable_control();
            let control_ref = control.borrow();
            let state = control_ref.get_current_state();
            drop(control_ref);

            let new_edit_mode = match state.edit_mode {
                EditMode::Locked => EditMode::Edit,
                EditMode::Edit => EditMode::Locked,
            };

            manager.send_event(
                EditableControlEvent::ChangeEditMode(new_edit_mode)
            );
        });
    }
}

impl<
    ControlType: Control + UpdatableControl<ControlState> + ActivableControl + StateSavableControl
        + 'static,
    ControlState: 'static
> UpdatableControl<EditableControlState> for EditableControl<ControlType, ControlState> {
    fn update_state(&mut self, state: EditableControlState) {
        self.change_mode(state.edit_mode.clone());
        *self.state.borrow_mut() = state;
    }

    fn get_current_state(&self) -> EditableControlState {
        self.state.borrow().clone()
    }
}

impl<
    ControlType: Control + UpdatableControl<ControlState> + ActivableControl + StateSavableControl
        + 'static,
    ControlState: 'static
>
EditableControl<ControlType, ControlState> {
    pub fn new(control: RcMut<ControlType>) -> Self {
        let editable_control_box = BoxBuilder::new("editable-control-element")
            .set_orientation(gtk::Orientation::Horizontal)
            .build();

        let mode_toggle_button = Button::with_label("✅");
        mode_toggle_button.set_vexpand(false);
        mode_toggle_button.set_valign(Align::Center);

        editable_control_box.append(control.borrow().get_widget());
        editable_control_box.append(&mode_toggle_button);

        let state = new_rc_mut(EditableControlState {
            edit_mode: EditMode::Edit
        });

        Self {
            _phantom_control_state: PhantomData,
            editable_control_box,
            control,
            mode_toggle_button,
            state,
        }
    }

    pub fn change_mode(&self, mode: EditMode) {
        match mode {
            EditMode::Edit => {
                self.control.borrow().enable_control();
                self.mode_toggle_button.set_label("✅");
            }
            EditMode::Locked => {
                self.control.borrow().disable_control();
                self.mode_toggle_button.set_label("✏️");
            }
        }
    }

    pub fn get_control(&self) -> RcMut<ControlType> {
        self.control.clone()
    }
}