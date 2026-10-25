use crate::state::popup_state::{Button, ConfirmationAction, PopupState, Popups, SettingsTabs};

#[derive(Clone, Copy, PartialEq)]
pub enum Views {
    Home,
    Trainer,
}

pub struct UIState {
    pub current_view: Views,
    pub menu_cursor: u8,

    pub popup_state: PopupState,
    pub confirmation_action: Option<ConfirmationAction>,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            current_view: Views::Home,
            menu_cursor: 0,
            popup_state: PopupState {
                active_popup: None,
                selected_tab: SettingsTabs::General,
                selected_option: 0,
                selected_button: Button::Cancel,
            },
            confirmation_action: None,
        }
    }
}

impl UIState {
    // Returns amount of interactable options for each page in Settings menu.
    pub fn num_options(&mut self) -> usize {
        match self.popup_state.selected_tab {
            SettingsTabs::General => 0,
            SettingsTabs::Appearance => 3,
            SettingsTabs::Trainer => 5,
        }
    }

    pub fn go_to_home_view(&mut self) {
        self.current_view = Views::Home;
        self.menu_cursor = 0;
    }

    pub fn toggle_settings_view(&mut self) {
        if self.popup_state.active_popup == Some(Popups::Settings) {
            self.popup_state.active_popup = None;
        } else {
            self.popup_state.active_popup = Some(Popups::Settings);
        }
    }

    pub fn cycle_tabs(&mut self) {
        match self.popup_state.selected_tab {
            SettingsTabs::General => {
                self.popup_state.selected_tab = SettingsTabs::Appearance;
            }
            SettingsTabs::Appearance => {
                self.popup_state.selected_tab = SettingsTabs::Trainer;
            }
            SettingsTabs::Trainer => {
                self.popup_state.selected_tab = SettingsTabs::General;
            }
        }

        self.popup_state.selected_option = 0;
    }

    pub fn cycle_options(&mut self, num_options: usize, forward: bool) {
        if num_options == 0 {
            return;
        }

        let current_option = self.popup_state.selected_option;

        if forward {
            self.popup_state.selected_option = (current_option + 1) % num_options;
        } else {
            self.popup_state.selected_option = (current_option + num_options - 1) % num_options;
        }
    }

    pub fn menu_cursor_up(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
    }

    pub fn menu_cursor_down(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
    }
}
