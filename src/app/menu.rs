use shakmaty::Color;

use crate::{app::App, state::ui_state::Views};

enum Openings {
    ItalianGame,
    KingsIndianDefense,
}

impl From<usize> for Openings {
    fn from(value: usize) -> Self {
        if let 0 = value {
            Openings::ItalianGame
        } else {
            Openings::KingsIndianDefense
        }
    }
}

impl App {
    pub fn menu_select(&mut self) {
        let field: Openings = Openings::from(self.ui_state.menu.selected_opening_index);

        match field {
            Openings::ItalianGame => {
                self.ui_state.current_view = Views::Trainer;
                self.trainer.state.current_opening_name = Some("italian-game".to_string());
                self.trainer.board.player_turn = Color::White;
            }

            Openings::KingsIndianDefense => {
                self.ui_state.current_view = Views::Trainer;
                self.trainer.state.current_opening_name = Some("kings-indian-defense".to_string());
                self.trainer
                    .prepare_black_player(self.settings.bot_move_delay);
            }
        }

        self.trainer.find_next_uncompleted_line(&self.player_data);
    }

    pub fn reset_home(&mut self) {
        self.ui_state.go_to_home_view();
        self.trainer.state.reset();
        self.trainer.board.reset();
    }
}
