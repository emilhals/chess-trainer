use ratatui::{Frame, layout::Rect};
use shakmaty::Square;

use crate::{
    app::coord::Coord, skin::Skin, state::click_regions::ClickRegions, trainer::board::Board,
    ui::board::render::render_grid, utils::flip_square_if_needed,
};

pub mod cell;
pub mod helper;
pub mod pieces;
pub mod render;

pub fn render(
    frame: &mut Frame<'_>,
    area: Rect,
    board: &Board,
    skin: &Skin,
    selected_square: Option<Square>,
    cursor_coord: Coord,
    regions: &mut ClickRegions,
) {
    let actual_square = selected_square.map(|s| flip_square_if_needed(s, board.is_flipped));

    render_grid(
        area,
        frame,
        board,
        skin,
        actual_square,
        cursor_coord,
        regions,
    );
}
