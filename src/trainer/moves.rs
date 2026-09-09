use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use shakmaty::{Square, uci::UciMove};

use crate::trainer::Trainer;

impl Trainer {
    pub fn try_move(&mut self, from: Square, to: Square) {
        // Check if moving a piece and get piece type
        let Some(role_from) = self.board.get_role_at_square(&from) else {
            return;
        };

        let role_to = self.board.get_role_at_square(&to);

        if let Some(executed_move) = self.board.execute_move(from, to) {
            let move_uci = executed_move
                .to_uci(shakmaty::CastlingMode::Standard)
                .to_string();

            let (correct_move, response) = self.validate_move(move_uci);

            if correct_move {
                self.state.bot_move_at =
                    Some(Instant::now() + Duration::from_millis(self.state.bot_move_delay));

                self.state.current_move_index += 1;

                if !self.is_completed() {
                    self.state.bot_should_move = true;
                }
            } else {
                self.state.bot_should_move = false;
                //self.wrong_move_played = true;

                self.state.validate_move_response = response;

                self.state.undo_move_at =
                    Some(Instant::now() + Duration::from_millis(self.state.undo_move_delay));
            }
        }
    }

    // Apply bot move, returns true if success
    pub fn apply_bot_move(&mut self) -> bool {
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

        let Some(executed_move) = self
            .board
            .execute_move(bot_actual_move.from().unwrap(), bot_actual_move.to())
        else {
            return false;
        };

        self.state.current_move_index += 1;
        self.state.current_instruction_index += 1;

        true
    }

    // Returns true if move is valid. Returns false with message if not valid.
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
}
