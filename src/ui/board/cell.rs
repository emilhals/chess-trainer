use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Stylize},
    widgets::Block,
};

use crate::{
    app::coord::Coord,
    skin::Skin,
    trainer::ui::CellRenderContext,
    ui::board::helper::{piece_centered_rect, render_piece_paragraph},
    utils::{get_coord_from_square, get_square_from_coord},
};

// Renders a single cell of the board.
pub fn render_single_cell(
    frame: &mut Frame<'_>,
    ctx: &CellRenderContext<'_>,
    skin: &Skin,
    cursor_coord: Coord,
) {
    let CellRenderContext {
        board,
        i,
        j,
        square,
        actual_square,
        last_move_from,
        last_move_to,
        legal_moves,
    } = *ctx;

    let current_rendering_coord = Coord::new(i, j);
    let current_rendering_square = get_square_from_coord(current_rendering_coord, board.is_flipped);

    // Safely determine if this specific cell is the one selected by the player
    let is_selected_cell = actual_square
        .is_some_and(|s| get_coord_from_square(s, board.is_flipped) == current_rendering_coord);

    let is_cursor_cell = cursor_coord == current_rendering_coord;

    if is_cursor_cell | is_selected_cell {
        render_cell(frame, square, Color::Cyan, None);
    }

    let cell_color: Color = if (i + j).is_multiple_of(2) {
        skin.light_square
    } else {
        skin.dark_square
    };

    // Where can the piece move?
    let is_cell_in_positions = legal_moves.contains(&current_rendering_coord);

    if is_selected_cell
        || last_move_from == Some(current_rendering_square)
        || last_move_to == Some(current_rendering_square) && !is_cell_in_positions
    {
        render_cell(frame, square, skin.last_move, None);
    } else if is_cell_in_positions {
        render_cell(frame, square, skin.legal_move, None);
    } else {
        let cell = Block::default().bg(cell_color);
        frame.render_widget(cell, square);
    }

    let coord = Coord::new(i, j);
    let square_index = get_square_from_coord(coord, board.is_flipped);
    let piece_role = board.get_role_at_square(&square_index);
    let piece_color = board.get_piece_color_at_square(&square_index);

    if is_cursor_cell {
        let cell = Block::default().bg(skin.cursor);

        frame.render_widget(cell, square);
    }

    let (paragraph, line_count) = render_piece_paragraph(piece_role, piece_color, square);
    let piece_area = piece_centered_rect(square, line_count);
    frame.render_widget(paragraph, piece_area);
}

// Renders a single board cell as a colored 'Block' with optional style modifier
fn render_cell(frame: &mut Frame, square: Rect, color: Color, modifier: Option<Modifier>) {
    let mut cell = Block::default().bg(color);
    if let Some(modifier) = modifier {
        cell = cell.add_modifier(modifier);
    }
    frame.render_widget(cell, square);
}
