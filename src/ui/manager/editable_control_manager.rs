use crate::ui::updatable_control::UpdatableControl;
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::Control;
use crate::ui::controls::editable_control::EditableControl;
use crate::ui::manager::control_manager::ControlManager;
use crate::ui::state_savable_control::StateSavableControl;
use crate::ui::states::editable_control_state::{EditMode, EditableControlState};
use crate::utils::RcMut;

pub enum EditableControlEvent {
    ChangeEditMode(EditMode)
}

pub struct EditableControlManager<
    ControlType: Control + UpdatableControl<ControlState> + ActivableControl + StateSavableControl 
        + 'static, 
    ControlState: 'static
> {
    editable_control: RcMut<EditableControl<ControlType, ControlState>>,
    application_provider: ApplicationProvider
}


impl<
    ControlType: Control + UpdatableControl<ControlState> + ActivableControl + StateSavableControl 
        + 'static, 
    ControlState: 'static
>
ControlManager<ControlType, EditableControlEvent> for EditableControlManager<ControlType, ControlState> {
    fn send_event(&self, event: EditableControlEvent) {
        let application_provider = self.application_provider.clone();
        match event {
            EditableControlEvent::ChangeEditMode(mode) => {
                let mut editable_control_mut = self.editable_control.borrow_mut();
                editable_control_mut.change_mode(mode.clone());
                editable_control_mut.update_state(EditableControlState {
                    edit_mode: mode.clone()
                });

                if let EditMode::Locked = mode {
                    editable_control_mut.get_control().borrow()
                        .save_settings(application_provider);
                }
            }
        }
    }

    fn get_control(&self) -> RcMut<ControlType> {
        let editable_control = self.get_editable_control();
        editable_control.borrow().get_control()
    }
}

impl<
    ControlType: Control + UpdatableControl<ControlState> + ActivableControl + StateSavableControl 
        + 'static, 
    ControlState: 'static
> 
EditableControlManager<ControlType, ControlState> {
    pub fn new(
        editable_control: RcMut<EditableControl<ControlType, ControlState>>,
        application_provider: ApplicationProvider
    ) -> Self {
        Self {
            editable_control,
            application_provider
        }
    }

    pub fn get_editable_control(&self) -> RcMut<EditableControl<ControlType, ControlState>> {
        self.editable_control.clone()
    }
}