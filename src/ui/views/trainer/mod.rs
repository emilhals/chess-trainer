pub mod header;
pub mod progress;
pub mod sidebar;
pub mod streak;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    app::App,
    state::{click_regions::ClickRegions, trainer_state::TrainerMode},
    ui::{
        board::{
            self, BoardState,
            render::{render_file_labels, render_rank_labels},
        },
        views::trainer::{
            header::top_panel_render, progress::progress_gauge_render, sidebar::instruction_render,
            streak::streak_render,
        },
    },
};

pub fn render_trainer_ui(
    app: &mut App,
    frame: &mut Frame<'_>,
    area: Rect,
    regions: &mut ClickRegions,
) {
    let Some(opening_name) = &app.trainer.state.current_opening_name else {
        return;
    };

    let Some(skin) = app.skins.get(&app.settings.selected_skin) else {
        return;
    };

    let [top_panel, _reserved_progress, main, _file_labels] = Layout::vertical([
        Constraint::Length(3), // Top panel
        Constraint::Length(2), // (Reserved) Progress gauge
        Constraint::Min(1),    // Board + Instructions + Streak
        Constraint::Length(2), // (Reserved) File labels
    ])
    .areas(area);

    top_panel_render(top_panel, frame, &app.trainer);

    let [_, board, _] = Layout::horizontal([
        Constraint::Length(2),  // (Reserved) Rank labels
        Constraint::Min(1),     // Board
        Constraint::Length(20), // (Reserved) Sidebar
    ])
    .areas(main);

    let board_state = BoardState {
        selected_square: app.trainer.ui.selected_square,
        cursor_coords: app.trainer.ui.cursor_coords,
        move_feedback: app.trainer.state.move_feedback,
    };
    let grid_area = board::render(frame, board, &app.trainer.board, skin, board_state, regions);

    let progress_area = Rect {
        x: grid_area.x,
        y: grid_area.top().saturating_sub(3),
        height: 2,
        width: grid_area.width,
    };
    progress_gauge_render(progress_area, frame, &app.trainer);

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

    let sidebar = Rect {
        x: grid_area.right().saturating_add(1),
        y: grid_area.y,
        height: grid_area.height,
        width: main.right().saturating_sub(grid_area.right()),
    };

    let [instructions, streak] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(5)]).areas(sidebar);

    instruction_render(instructions, frame, &app.trainer, &app.settings);

    let drill_mode = app.trainer.state.mode.selected_mode == TrainerMode::Drill;
    if drill_mode {
        let highest_streak = app.player_data.best_streak(opening_name);

        streak_render(
            streak,
            frame,
            app.trainer.state.current_streak,
            highest_streak,
            skin,
        );
    }
}
