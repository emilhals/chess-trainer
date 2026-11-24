use ratatui::{Frame, layout::Rect};
use shakmaty::Square;

use crate::{
    app::coord::Coord,
    skin::Skin,
    state::{click_regions::ClickRegions, trainer_state::MoveFeedback},
    trainer::board::Board,
    ui::board::render::render_grid,
    utils::flip_square_if_needed,
};

pub mod cell;
pub mod helper;
pub mod pieces;
pub mod render;

#[derive(Debug, Copy, Clone, Default)]
pub struct BoardState {
    pub selected_square: Option<Square>,
    pub cursor_coords: Coord,
    pub move_feedback: Option<MoveFeedback>,
}

pub fn render(
    frame: &mut Frame<'_>,
    area: Rect,
    board: &Board,
    skin: &Skin,
    state: BoardState,
    regions: &mut ClickRegions,
) {
    let actual_square = state
        .selected_square
        .map(|s| flip_square_if_needed(s, board.is_flipped));

    render_grid(area, frame, board, skin, state, actual_square, regions);
}
