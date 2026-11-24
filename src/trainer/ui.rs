use ratatui::layout::Rect;
use shakmaty::Square;

use crate::{
    app::coord::{Coord, MoveDirection},
    trainer::board::Board,
};

// Context for rendering a single cell of the board.
pub struct CellRenderContext<'a> {
    pub(crate) board: &'a Board,
    pub(crate) i: u8,
    pub(crate) j: u8,
    pub(crate) square: Rect,
    pub(crate) actual_square: Option<Square>,
    pub(crate) last_move_from: Option<Square>,
    pub(crate) last_move_to: Option<Square>,
    pub(crate) legal_moves: &'a [Coord],
}

#[derive(Default)]
pub struct UI {
    /// The cursor position
    pub cursor_coords: Coord,
    /// The old cursor position when unselecting a cell
    pub old_cursor_coords: Option<Coord>,
    /// The selected square
    pub selected_square: Option<Square>,
    /// The selected piece cursor when we already selected a piece
    pub selected_piece_cursor: i8,
    /// The start position if a piece is dragged.
    pub drag_from: Option<Coord>,
}

impl UI {
    pub fn is_cell_selected(&self) -> bool {
        self.selected_square.is_some()
    }

    pub fn selected_coord(&self) -> Option<Coord> {
        self.selected_square.map(Coord::from)
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

    pub fn undo_cursor(&mut self) {
        if let Some(coords) = self.old_cursor_coords {
            self.cursor_coords = coords;
        }
    }

    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_square = None;
            self.selected_piece_cursor = 0;
            self.drag_from = None;
        }
    }
}
