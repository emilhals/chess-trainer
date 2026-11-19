use shakmaty::Square;

use crate::{
    app::coord::Coord, settings::Settings, trainer::Trainer, utils::flip_square_if_needed,
};

impl Trainer {
    pub fn go_up(&mut self) {
        self.ui.cursor_up();
    }

    pub fn go_down(&mut self) {
        self.ui.cursor_down();
    }

    pub fn go_left(&mut self) {
        self.ui.cursor_left();
    }

    pub fn go_right(&mut self) {
        self.ui.cursor_right();
    }

    pub fn select_cell(&mut self) {
        let square: Square = self.ui.cursor_coords.into();
        let square: Square = Coord::from(square).into();

        let actual_square = flip_square_if_needed(square, self.board.is_flipped);

        let legal_moves = self.board.get_legal_moves(&actual_square);

        if legal_moves.is_empty() {
            return;
        }

        self.ui.selected_square = Some(square);
        self.ui.old_cursor_coords = Some(self.ui.cursor_coords);
    }

    pub fn handle_already_selected_cell(&mut self, settings: &Settings) {
        let Some(selected_square) = self.ui.selected_square else {
            return;
        };

        let cursor_square = self.ui.cursor_coords.into();

        let actual_selected_coords = flip_square_if_needed(selected_square, self.board.is_flipped);
        let actual_cursor_coords = flip_square_if_needed(cursor_square, self.board.is_flipped);

        if actual_cursor_coords == actual_selected_coords {
            return;
        }

        let legal_moves = self.board.get_legal_moves(&actual_selected_coords);

        if !legal_moves.contains(&actual_cursor_coords) {
            let piece_at_destination = self.board.get_piece_color_at_square(&actual_cursor_coords);

            if piece_at_destination == Some(self.player_turn) {
                self.select_cell();
            } else {
                self.ui.unselect_cell();
            }
            return;
        }

        self.try_move(actual_selected_coords, actual_cursor_coords, settings);

        self.ui.unselect_cell();
    }

    pub fn handle_cell_click(&mut self, settings: &Settings) {
        if self.is_input_blocked() {
            return;
        }

        if self.ui.is_cell_selected() {
            self.handle_already_selected_cell(settings);
        } else {
            self.select_cell();
        }
    }

    pub fn press_square(&mut self, coord: Coord, settings: &Settings) {
        if self.ui.selected_coord() != Some(coord) {
            self.click_square(coord, settings);
        }

        if self.ui.selected_coord() == Some(coord) {
            self.ui.drag_from = Some(coord);
        } else {
            self.ui.drag_from = None;
        }
    }

    pub fn drop_on_square(&mut self, coord: Coord, settings: &Settings) {
        let start = self.ui.drag_from.take();

        if start.is_none() {
            return;
        }

        if start == Some(coord) {
            return;
        }

        self.click_square(coord, settings);
    }

    pub fn cancel_drag(&mut self) {
        self.ui.drag_from = None;
    }

    fn click_square(&mut self, coord: Coord, settings: &Settings) {
        self.ui.cursor_coords = coord;
        self.handle_cell_click(settings);
    }
}
