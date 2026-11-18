use shakmaty::Color;

use crate::app::App;

impl App {
    pub fn start_trainer(&mut self, opening_index: usize) {
        let Some((name, opening)) = self.trainer.openings.get_opening_by_index(opening_index)
        else {
            return;
        };

        self.trainer.state.reset();
        self.trainer.board.reset();

        let color = opening.player_color();

        self.trainer.state.current_opening_name = Some(name.to_string());

        match color {
            Color::White => {
                self.trainer.board.player_turn = Color::White;
            }
            Color::Black => {
                self.trainer
                    .prepare_black_player(self.settings.bot_move_delay);
            }
        }

        self.trainer.find_next_uncompleted_line(&self.player_data);
        self.ui_state.go_to_trainer_view();
    }

    pub fn reset_home(&mut self) {
        self.ui_state.go_to_home_view();
        self.trainer.state.reset();
        self.trainer.board.reset();
    }
}
