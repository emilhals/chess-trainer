use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::Block,
};

use crate::{
    app::coord::Coord,
    skin::Skin,
    state::trainer_state::MoveFeedback,
    trainer::ui::CellRenderContext,
    ui::board::{
        BoardState,
        helper::{piece_centered_rect, render_piece_paragraph},
    },
    utils::{get_coord_from_square, get_square_from_coord},
};

// Renders a single cell of the board.
pub fn render_single_cell(
    frame: &mut Frame<'_>,
    ctx: &CellRenderContext<'_>,
    skin: &Skin,
    board_state: BoardState,
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

    let is_cursor_cell = board_state.cursor_coords == current_rendering_coord;

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

    if last_move_to == Some(current_rendering_square)
        && let Some(marker) = board_state.move_feedback
    {
        render_marker(frame, square, marker, skin);
    }
}

// Renders a single board cell as a colored 'Block' with optional style modifier
fn render_cell(frame: &mut Frame, square: Rect, color: Color, modifier: Option<Modifier>) {
    let mut cell = Block::default().bg(color);
    if let Some(modifier) = modifier {
        cell = cell.add_modifier(modifier);
    }
    frame.render_widget(cell, square);
}

fn render_marker(frame: &mut Frame, area: Rect, feedback: MoveFeedback, skin: &Skin) {
    const BADGE_WIDTH: u16 = 3;
    const PADDING: u16 = 1;

    if area.width < BADGE_WIDTH + PADDING || area.height == 0 {
        return;
    }

    let (symbol, background) = match feedback {
        MoveFeedback::Correct => ("✓", skin.feedback_correct),
        MoveFeedback::Incorrect => ("✕", skin.feedback_incorrect),
    };

    let badge = Rect {
        x: area.right() - BADGE_WIDTH - PADDING,
        y: area.y,
        width: BADGE_WIDTH,
        height: 1,
    };

    let style = Style::default().fg(Color::White).bg(background);

    frame.render_widget(Span::styled(format!(" {symbol} "), style), badge);
}
