use crate::{
    app::{App, AppResult},
    state::popup_state::{
        ConfirmationAction::{self, ResetProgress},
        Popups, SettingsTabs,
    },
};

impl App {
    pub fn execute_confirmation(&mut self, action: ConfirmationAction) -> AppResult<()> {
        match action {
            ConfirmationAction::ResetProgress => {
                self.player_data.reset_progress()?;
                self.trainer.state.reset();

                self.ui_state.popup_state.open_settings();
            }
        }

        Ok(())
    }

    pub fn adjust_selected_setting(&mut self, forward: bool) {
        let selected_tab = self.ui_state.popup_state.selected_tab;
        let selected_option = self.ui_state.popup_state.selected_option;

        match selected_tab {
            SettingsTabs::General => {}
            SettingsTabs::Appearance => match selected_option {
                0 => self.settings.toggle_show_coords(),
                1 => self.settings.cycle_skin(self.skins.clone(), forward),
                _ => {}
            },
            SettingsTabs::Trainer => match selected_option {
                0 => self.settings.adjust_bot_delay(forward),
                1 => self.settings.adjust_undo_delay(forward),
                2 => self.settings.adjust_advance_delay(forward),
                _ => {}
            },
        }
    }

    pub fn toggle_selected_setting(&mut self) {
        let selected_tab = self.ui_state.popup_state.selected_tab;
        let selected_option = self.ui_state.popup_state.selected_option;

        match selected_tab {
            SettingsTabs::General => {}
            SettingsTabs::Appearance => {
                if selected_option == 0 {
                    self.settings.toggle_show_coords();
                }
            }
            SettingsTabs::Trainer => match selected_option {
                3 => self.settings.toggle_auto_advance(),
                4 => {
                    self.ui_state.popup_state.active_popup =
                        Some(Popups::Confirmation(ResetProgress))
                }
                _ => {}
            },
        }
    }
}
