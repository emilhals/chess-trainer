use crate::state::{
    menu_state::MenuState,
    popup_state::{ConfirmationAction, PopupState, Popups},
};

#[derive(Clone, Copy, PartialEq)]
pub enum Views {
    Menu,
    Trainer,
}

pub struct UIState {
    pub current_view: Views,

    pub menu: MenuState,
    pub popup: PopupState,
    pub confirmation_action: Option<ConfirmationAction>,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            current_view: Views::Menu,
            menu: MenuState::default(),
            popup: PopupState::default(),
            confirmation_action: None,
        }
    }
}

impl UIState {
    pub fn go_to_home_view(&mut self) {
        self.current_view = Views::Menu;
    }

    pub fn toggle_settings_view(&mut self) {
        if self.popup.active_popup == Some(Popups::Settings) {
            self.popup.active_popup = None;
        } else {
            self.popup.active_popup = Some(Popups::Settings);
        }
    }
}
