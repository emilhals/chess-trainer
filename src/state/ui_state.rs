use crate::state::{
    click_regions::{ClickRegions, ClickTarget},
    menu_state::MenuState,
    popup_state::{ConfirmationAction, PopupState, Popups},
};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Views {
    #[default]
    Menu,
    Trainer,
}

#[derive(Default)]
pub struct UIState {
    pub current_view: Views,

    pub menu: MenuState,
    pub popup: PopupState,
    pub confirmation_action: Option<ConfirmationAction>,

    pub regions: ClickRegions,
    pub hovered: Option<ClickTarget>,
    pub mouse_disabled: bool,
}

impl UIState {
    pub fn go_to_home_view(&mut self) {
        self.current_view = Views::Menu;
    }

    pub fn go_to_trainer_view(&mut self) {
        self.current_view = Views::Trainer;
    }

    pub fn toggle_settings_view(&mut self) {
        if self.popup.active_popup == Some(Popups::Settings) {
            self.popup.active_popup = None;
        } else {
            self.popup.active_popup = Some(Popups::Settings);
        }
    }
}
