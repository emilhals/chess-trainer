use crate::{app::App, state::ui_state::Views};

enum Openings {
    ItalianGame,
    KingsIndianDefense,
}

impl From<u8> for Openings {
    fn from(value: u8) -> Self {
        if let 0 = value {
            Openings::ItalianGame
        } else {
            Openings::KingsIndianDefense
        }
    }
}

impl App {
    pub fn menu_select(&mut self) {
        let field: Openings = Openings::from(self.ui_state.menu_cursor);

        match field {
            Openings::ItalianGame => {
                self.ui_state.current_view = Views::Trainer;
                self.trainer.state.current_opening_name = Some("italian-game".to_string());
            }

            Openings::KingsIndianDefense => {
                self.ui_state.current_view = Views::Trainer;
                self.trainer.state.current_opening_name = Some("kings-indian-defense".to_string());
            }
        }
    }

    pub fn reset_home(&mut self) {
        self.ui_state.go_to_homeview();
    }
}
