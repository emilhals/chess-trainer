use crate::{app::App, state::ui_state::Views};

pub enum MainMenuItems {
    Play,
    SettingsMenu,
    Quit,
}

impl From<u8> for MainMenuItems {
    fn from(value: u8) -> Self {
        match value {
            0 => MainMenuItems::Play,
            1 => MainMenuItems::SettingsMenu,
            2 => MainMenuItems::Quit,
            _ => MainMenuItems::Play,
        }
    }
}
impl App {
    pub fn menu_select(&mut self) {
        let field: MainMenuItems = MainMenuItems::from(self.ui_state.menu_cursor);

        match field {
            MainMenuItems::Play => {
                self.ui_state.current_view = Views::Trainer;
            }
            _ => {}
        }
    }

    pub fn reset_home(&mut self) {
        self.ui_state.go_to_homeview();
    }
}
