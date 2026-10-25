use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::Style,
    widgets::Block,
};

use crate::{app::App, state::trainer_state::TrainerMode};

pub fn render_trainer_ui(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let main_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),     // Top panel
                Constraint::Ratio(1, 20),  // Top padding
                Constraint::Ratio(18, 20), // Board
                Constraint::Min(0),        // Bottom padding
            ]
            .as_ref(),
        )
        .split(area);

    app.trainer
        .ui
        .top_panel_render(main_rows[0], frame, &app.trainer);

    let content_columns = Layout::default()
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
        .split(main_rows[2]);

    let board_with_labels = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Progress gayge
            Constraint::Min(0),    // Board
            Constraint::Length(1), // File labels
        ])
        .split(content_columns[2]);

    let rank_label_area = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(0),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(content_columns[1]);

    let board_block = Block::default().style(Style::default());

    // Get the inner area of the board (accounting for any block padding)
    let board_inner = board_block.inner(board_with_labels[1]);

    let has_labels = app.settings.show_coords;

    let Some(skin) = app.skins.get(&app.settings.selected_skin) else {
        return;
    };

    app.trainer
        .ui
        .board_render(board_inner, frame, &app.trainer.board, &skin);

    if has_labels {
        app.trainer
            .ui
            .render_rank_labels(frame, rank_label_area[1], app.trainer.board.is_flipped);

        app.trainer.ui.render_file_labels(
            frame,
            board_with_labels[2],
            app.trainer.board.is_flipped,
        );
    }

    app.trainer.ui.progress_gauge_render(
        board_block.inner(board_with_labels[0]),
        frame,
        &app.trainer,
    );

    let trainer_sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Same height as progress gauge
            Constraint::Min(0),    // Align with board
            Constraint::Length(1), // Same height as file labels
        ])
        .split(content_columns[3]);

    let sidebar_content_area = trainer_sidebar[1];
    let board_area = board_with_labels[1];

    let cell_height = board_area.height / 8;
    let board_top_padding = board_area.height / 2 - (4 * cell_height);
    let board_bottom = app.trainer.ui.top_y + app.trainer.ui.height * 8;
    let streak_height = 5;

    let streak_area = Rect::new(
        sidebar_content_area.x,
        board_bottom.saturating_sub(streak_height),
        sidebar_content_area.width.saturating_sub(8),
        streak_height,
    );

    let drill_mode = app.trainer.state.mode.selected_mode == TrainerMode::Drill;
    let Some(opening_name) = &app.trainer.state.current_opening_name else {
        return;
    };

    let instruction_height =
        app.trainer
            .ui
            .instruction_height(sidebar_content_area.width, &app.trainer) as u16;

    let [top, _streak_area, _bottom_padding] = Layout::vertical([
        Constraint::Length(board_top_padding + instruction_height),
        Constraint::Length(5),
        Constraint::Length(1),
    ])
    .flex(Flex::SpaceBetween)
    .areas(sidebar_content_area);

    let [_, instruction_area] = Layout::vertical([
        Constraint::Length(board_top_padding),
        Constraint::Length(instruction_height),
    ])
    .areas(top);

    if drill_mode {
        app.trainer
            .ui
            .instruction_render(instruction_area, frame, &app.trainer);

        let highest_streak = app.player_data.best_streak(&opening_name);

        app.trainer
            .ui
            .streak_render(streak_area, frame, &app.trainer, highest_streak);
    } else {
        app.trainer
            .ui
            .instruction_render(instruction_area, frame, &app.trainer);
    }
}
