use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use shakmaty::{Square, uci::UciMove};

use crate::{
    settings::Settings, state::trainer_state::TrainerMode, trainer::Trainer,
    utils::get_coord_from_square,
};

impl Trainer {
    pub fn try_move(&mut self, from: Square, to: Square, settings: &Settings) {
        if let Some(executed_move) = self.board.execute_move(from, to) {
            let move_uci = executed_move
                .to_uci(shakmaty::CastlingMode::Standard)
                .to_string();

            let (correct_move, response) = self.validate_move(move_uci);

            if correct_move {
                self.state.bot_move_at =
                    Some(Instant::now() + Duration::from_millis(settings.bot_move_delay));

                self.state.current_move_index += 1;

                if !self.is_completed() {
                    self.state.bot_should_move = true;
                }

                // In Drill mode, we go automatically to the next line
                if self.is_completed() && self.state.mode.selected_mode == TrainerMode::Drill {
                    self.state.next_line_at =
                        Some(Instant::now() + Duration::from_millis(settings.bot_move_delay));
                }
            } else {
                self.state.bot_should_move = false;
                self.state.made_wrong_move = true;

                self.state.validate_move_response = response;

                self.state.undo_move_at =
                    Some(Instant::now() + Duration::from_millis(settings.undo_move_delay));
            }
        }
    }

    pub fn apply_bot_move(&mut self, settings: &Settings) -> bool {
        let bot_move_uci = self
            .openings
            .get_move_uci(
                self.state.current_opening_name.as_ref().unwrap(),
                self.state.current_line_index,
                self.state.current_move_index,
            )
            .unwrap();

        let current_position = self.board.position_ref().clone();

        let Some(uci_move) = UciMove::from_str(bot_move_uci).ok() else {
            return false;
        };

        let bot_actual_move = uci_move.to_move(&current_position).unwrap();

        let Some(_executed_move) = self
            .board
            .execute_move(bot_actual_move.from().unwrap(), bot_actual_move.to())
        else {
            return false;
        };

        if self.board.move_history.len() > 1 {
            self.state.current_instruction_index += 1;
        }

        self.state.current_move_index += 1;

        if self.is_completed() && self.state.mode.selected_mode == TrainerMode::Drill {
            self.state.next_line_at =
                Some(Instant::now() + Duration::from_millis(settings.bot_move_delay));
        }

        true
    }

    pub fn validate_move(&self, move_uci: String) -> (bool, Option<String>) {
        let Some(line) = self.current_line() else {
            return (false, None);
        };

        let correct_move = line.move_uci(self.state.current_move_index);

        if correct_move == move_uci {
            (true, None)
        } else {
            (false, Some("Wrong move!".to_string()))
        }
    }

    /// Places cursor to the correct next move
    pub fn show_hint(&mut self) {
        if self.is_input_blocked() {
            return;
        }

        let (Some(from), _) = self.get_current_move_squares() else {
            return;
        };

        let from_coords = get_coord_from_square(from, self.board.is_flipped);
        self.ui.cursor_coords = from_coords;

        self.state.mode.increment_hint_level();

        // Showing a hint counts as a wrong move
        self.state.made_wrong_move = true;
    }

    pub fn apply_solution_move(&mut self, bot_move_delay: u64) {
        if self.is_input_blocked() {
            return;
        }

        let (Some(from), Some(to)) = self.get_current_move_squares() else {
            return;
        };

        if self.board.execute_move(from, to).is_some() {
            self.state.bot_move_at = Some(Instant::now() + Duration::from_millis(bot_move_delay));

            self.state.current_move_index += 1;

            if !self.is_completed() {
                self.state.bot_should_move = true;
            }
        };

        // Incrementing the hint level here, will set it back to HintLevel::Low
        self.state.mode.increment_hint_level();
    }

    fn get_current_move_squares(&self) -> (Option<Square>, Option<Square>) {
        let Some(line) = self.current_line() else {
            return (None, None);
        };

        let Some(move_uci_str) = line.moves.get(self.state.current_move_index) else {
            return (None, None);
        };

        let Some(uci_move) = UciMove::from_str(move_uci_str).ok() else {
            return (None, None);
        };

        (uci_move.from(), uci_move.to())
    }
}
