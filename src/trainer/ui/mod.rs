pub mod helper;
pub mod render;

use ratatui::layout::Rect;
use shakmaty::Square;

use crate::{
    app::coord::{Coord, MoveDirection},
    trainer::board::Board,
};

// Context for rendering a single cell of the board.
pub struct CellRenderContext<'a> {
    board: &'a Board,
    i: u8,
    j: u8,
    square: Rect,
    actual_square: Option<Square>,
    last_move_from: Option<Square>,
    last_move_to: Option<Square>,
    legal_moves: &'a [Coord],
}

pub struct UI {
    /// The cursor position
    pub cursor_coords: Coord,
    /// The old cursor position when unselecting a cell
    pub old_cursor_coords: Option<Coord>,
    /// The selected square
    pub selected_square: Option<Square>,
    /// The selected piece cursor when we already selected a piece
    pub selected_piece_cursor: i8,
    pub width: u16,
    pub height: u16,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            cursor_coords: Coord::default(),
            selected_square: None,
            selected_piece_cursor: 0,
            old_cursor_coords: None,

            width: 0,
            height: 0,
        }
    }
}

impl UI {
    #[must_use]
    pub fn is_cell_selected(&self) -> bool {
        self.selected_square.is_some()
    }

    pub fn cursor_up(&mut self) {
        self.cursor_coords.move_to(MoveDirection::Up);
    }

    pub fn cursor_down(&mut self) {
        self.cursor_coords.move_to(MoveDirection::Down);
    }

    pub fn cursor_left(&mut self) {
        self.cursor_coords.move_to(MoveDirection::Left);
    }

    pub fn cursor_right(&mut self) {
        self.cursor_coords.move_to(MoveDirection::Right);
    }

    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_square = None;
            self.selected_piece_cursor = 0;
        }
    }

    fn get_last_move_squares(board: &Board) -> (Option<Square>, Option<Square>) {
        if board.move_history.is_empty() {
            return (None, None);
        }

        let Some(last_move) = board.move_history.last() else {
            return (None, None);
        };

        let last_move_from = last_move.from();
        let last_move_to = Some(last_move.to());

        (last_move_from, last_move_to)
    }

    fn get_valid_moves_for_render(
        &self,
        board: &Board,
        actual_square: Option<Square>,
    ) -> Vec<Coord> {
        let Some(square) = actual_square else {
            return vec![];
        };

        if !self.is_cell_selected() {
            return vec![];
        }

        let selected_piece_color = board.get_piece_color_at_square(&square);

        if let Some(color) = selected_piece_color
            && color == board.player_turn
        {
            let mut legal_moves: Vec<Coord> = board
                .get_legal_moves(&square)
                .iter()
                .map(|&s| Coord::from(s))
                .collect();

            if board.is_flipped {
                legal_moves = legal_moves.iter().map(Coord::reverse).collect();
            }
            return legal_moves;
        }
        vec![]
    }
}
