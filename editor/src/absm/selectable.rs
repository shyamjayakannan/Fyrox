//! A mixin that provides selection functionality for a widget.

use fyrox::{
    core::pool::Handle,
    gui::message::{MessageDirection, MouseButton, UiMessage},
    gui::widget::WidgetMessage,
    gui::{define_constructor, UiNode, UserInterface},
};

#[derive(Debug, Clone, PartialEq)]
pub enum SelectableMessage {
    Select(bool),
}

impl SelectableMessage {
    define_constructor!(SelectableMessage:Select => fn select(bool), layout: false);
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Selectable {
    selected: bool,
}

impl Selectable {
    pub fn handle_routed_message(
        &mut self,
        self_handle: Handle<UiNode>,
        ui: &mut UserInterface,
        message: &mut UiMessage,
    ) {
        if let Some(msg) = message.data::<WidgetMessage>() {
            match msg {
                WidgetMessage::MouseDown { button, .. } => {
                    if *button == MouseButton::Left || *button == MouseButton::Right {
                        if !self.selected {
                            message.set_handled(true);

                            ui.send_message(SelectableMessage::select(
                                self_handle,
                                MessageDirection::ToWidget,
                                true,
                            ));

                            ui.capture_mouse(self_handle);
                        }
                    }
                }
                WidgetMessage::MouseUp { button, .. } => {
                    if *button == MouseButton::Left || *button == MouseButton::Right {
                        ui.release_mouse_capture();
                    }
                }
                _ => {}
            }
        } else if let Some(SelectableMessage::Select(selected)) = message.data() {
            if message.destination() == self_handle {
                if message.direction() == MessageDirection::ToWidget {
                    if self.selected != *selected {
                        self.selected = *selected;
                        ui.send_message(message.reverse());
                    }
                }
            }
        }
    }
}