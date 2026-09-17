use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::Block,
};

use crate::app::App;

pub fn render_trainer_ui(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),     // Top panel
                Constraint::Ratio(1, 20),  // Top padding
                Constraint::Ratio(18, 20), // Board
                Constraint::Min(0),        // Bottom padding
            ]
            .as_ref(),
        )
        .split(area);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 18),  // Left padding
                Constraint::Ratio(1, 18),  // Rank labels
                Constraint::Ratio(11, 18), // Board
                Constraint::Ratio(4, 18),  // Instruction block
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[2]);

    let board_with_labels = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Progress gayge
            Constraint::Min(0),    // Board
            Constraint::Length(1), // File labels
        ])
        .split(main_layout_vertical[2]);

    let rank_label_area = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(0),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(main_layout_vertical[1]);

    let board_block = Block::default().style(Style::default());

    let right_box_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2), // Padding for aligning with board
                Constraint::Length(6), // Info box
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(main_layout_vertical[3]);

    // Get the inner area of the board (accounting for any block padding)
    let board_inner = board_block.inner(board_with_labels[1]);
    app.trainer
        .ui
        .board_render(board_inner, frame, &app.trainer.board);

    app.trainer
        .ui
        .render_rank_labels(frame, rank_label_area[1], app.trainer.board.is_flipped);

    app.trainer
        .ui
        .render_file_labels(frame, board_with_labels[2], app.trainer.board.is_flipped);

    app.trainer.ui.progress_gauge_render(
        board_block.inner(board_with_labels[0]),
        frame,
        &app.trainer,
    );

    app.trainer
        .ui
        .instruction_render(right_box_layout[1], frame, &app.trainer);

    app.trainer
        .ui
        .panel_render(main_layout_horizontal[0], frame, &app.trainer);
}
