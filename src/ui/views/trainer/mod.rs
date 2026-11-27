pub mod panel;
pub mod progress;
pub mod sidebar;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Spacing::Overlap},
    symbols::merge::MergeStrategy,
    widgets::Block,
};

use crate::{
    app::App,
    state::click_regions::ClickRegions,
    ui::board::{
        self, BoardState,
        render::{render_file_labels, render_rank_labels},
    },
};

pub fn render_trainer_ui(
    app: &mut App,
    frame: &mut Frame<'_>,
    area: Rect,
    regions: &mut ClickRegions,
) {
    let Some(skin) = app.skins.get(&app.settings.selected_skin) else {
        return;
    };

    let Some(opening_name) = &app.trainer.state.current_opening_name else {
        return;
    };

    let [top, main, bottom] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .spacing(Overlap(1))
    .areas(area);

    let [left, right] = Layout::horizontal([Constraint::Length(40), Constraint::Min(1)])
        .spacing(Overlap(1))
        .areas(main);

    let top_block = Block::bordered().merge_borders(MergeStrategy::Exact);
    let bottom_block = Block::bordered().merge_borders(MergeStrategy::Exact);

    panel::render_top(
        frame,
        top_block.inner(top),
        opening_name,
        app.trainer.state.current_line_index,
        &app.trainer.state.mode.selected_mode.to_string(),
    );
    panel::render_bottom(frame, bottom_block.inner(bottom));

    let board_block = Block::bordered().merge_borders(MergeStrategy::Exact);
    let sidebar_block = Block::bordered().merge_borders(MergeStrategy::Exact);

    let inner_board = board_block.inner(right);
    let [_reserved_progress, main, _file_labels] = Layout::vertical([
        Constraint::Length(2), // (Reserved) Progress gauge
        Constraint::Min(1),    // Board + Instructions + Streak
        Constraint::Length(2), // (Reserved) File labels
    ])
    .areas(inner_board);

    let board_state = BoardState {
        selected_square: app.trainer.ui.selected_square,
        cursor_coords: app.trainer.ui.cursor_coords,
        move_feedback: app.trainer.state.move_feedback,
    };
    let grid_area = board::render(frame, main, &app.trainer.board, skin, board_state, regions);

    if app.settings.show_coords {
        let rank_area = Rect {
            x: grid_area.left().saturating_sub(3),
            y: grid_area.y,
            height: grid_area.height,
            width: 2,
        };

        let file_area = Rect {
            x: grid_area.x,
            y: grid_area.bottom().saturating_add(1),
            height: 2,
            width: grid_area.width,
        };

        render_rank_labels(frame, rank_area, app.trainer.board.is_flipped);
        render_file_labels(frame, file_area, app.trainer.board.is_flipped);
    }

    sidebar::render(frame, sidebar_block.inner(left), &app.trainer, skin);

    frame.render_widget(top_block, top);
    frame.render_widget(sidebar_block, left);
    frame.render_widget(board_block, right);
    frame.render_widget(bottom_block, bottom);
}
