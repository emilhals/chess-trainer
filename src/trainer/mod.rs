use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use shakmaty::{Color, Square, uci::UciMove};

use crate::{
    app::coord::Coord,
    state::trainer_state::{HintLevel, TrainerState},
    trainer::{
        board::Board,
        openings::{Line, Openings},
        ui::UI,
    },
    utils::{flip_square_if_needed, get_coord_from_square},
};

pub mod board;
pub mod input;
pub mod moves;
pub mod openings;
pub mod ui;

pub struct Trainer {
    pub board: Board,
    pub openings: Openings,
    pub state: TrainerState,
    pub ui: UI,
    pub player_turn: Color,
}

impl Trainer {
    pub fn new(openings: Openings) -> Self {
        Self {
            board: Board::default(),
            openings: openings,
            state: TrainerState::default(),
            ui: UI::default(),
            player_turn: Color::White,
        }
    }

    pub fn current_line(&self) -> Option<&Line> {
        let Some(opening_name) = self.state.current_opening_name.as_deref() else {
            return None;
        };

        let line = self
            .openings
            .get_line(&opening_name, self.state.current_line_index);

        line
    }

    pub fn current_instruction(&self) -> String {
        let Some(opening_name) = self.state.current_opening_name.as_deref() else {
            return String::from("");
        };

        let instruction = self.openings.get_instruction(
            opening_name,
            self.state.current_line_index,
            self.state.current_instruction_index,
        );

        instruction
    }

    /// Reset board state and reset opening state related only to the line
    pub fn next_line(&mut self) {
        self.board.reset();
        self.state.reset_for_next_line();
    }

    pub fn is_completed(&self) -> bool {
        let Some(line) = self.current_line() else {
            return false;
        };

        line.is_completed(self.state.current_move_index)
    }

    fn get_current_move_squares(&mut self) -> (Option<Square>, Option<Square>) {
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

    // Places cursor to the correct next move.
    pub fn show_hint(&mut self) {
        let (Some(from), _) = self.get_current_move_squares() else {
            return;
        };

        let from_coords = get_coord_from_square(from, self.board.is_flipped);
        self.ui.cursor_coords = from_coords;

        self.state.mode.increment_hint_level();
    }

    pub fn apply_solution_move(&mut self) {
        let (Some(from), Some(to)) = self.get_current_move_squares() else {
            return;
        };

        if let Some(_) = self.board.execute_move(from, to) {
            self.state.bot_move_at =
                Some(Instant::now() + Duration::from_millis(self.state.bot_move_delay));

            self.state.current_move_index += 1;

            if !self.is_completed() {
                self.state.bot_should_move = true;
            }
        };

        // Incrementing the hint level here, will set it back to HintLevel::Low
        self.state.mode.increment_hint_level();
    }

    pub fn move_progression(&self) -> usize {
        let Some(line) = self.current_line() else {
            return 0;
        };

        (self.state.current_move_index * 100) / line.len()
    }

    pub fn update_opening_state(&mut self) {
        // If an opening has not been set, assign a random opening
        if self.state.current_opening_name.is_none() {
            if let Some((name, opening)) = self.openings.get_random_opening() {
                self.state.current_opening_name = Some(name.clone());
            }
        }
    }

    pub fn select_cell(&mut self) {
        let square: Square = self.ui.cursor_coords.into();
        let square: Square = Coord::from(square).into();

        let actual_square = flip_square_if_needed(square, self.board.is_flipped);

        let piece_color = self.board.get_piece_color_at_square(&actual_square);

        let legal_moves = self.board.get_legal_moves(self.player_turn, &actual_square);

        if legal_moves.is_empty() {
            return;
        }

        self.ui.selected_square = Some(square);
        self.ui.old_cursor_coords = Some(self.ui.cursor_coords);
    }

    pub fn handle_already_selected_cell(&mut self) {
        let Some(selected_square) = self.ui.selected_square else {
            return;
        };

        let cursor_square = self.ui.cursor_coords.into();

        let actual_selected_coords = flip_square_if_needed(selected_square, self.board.is_flipped);
        let actual_cursor_coords = flip_square_if_needed(cursor_square, self.board.is_flipped);

        if actual_cursor_coords == actual_selected_coords {
            return;
        }

        let legal_moves = self
            .board
            .get_legal_moves(self.player_turn, &actual_selected_coords);

        if !legal_moves.contains(&actual_cursor_coords) {
            let piece_at_destination = self.board.get_piece_color_at_square(&actual_cursor_coords);

            if piece_at_destination == Some(self.player_turn) {
                self.select_cell();
            } else {
                self.ui.unselect_cell();
            }
            return;
        }

        self.try_move(actual_selected_coords, actual_cursor_coords);

        self.ui.unselect_cell();
    }

    pub fn handle_cell_click(&mut self) {
        if self.ui.is_cell_selected() {
            self.handle_already_selected_cell();
        } else {
            self.select_cell();
        }
    }
}
