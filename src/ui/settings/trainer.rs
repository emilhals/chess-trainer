use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
};

use crate::{
    app::App,
    settings::Settings,
    ui::settings::{render_adjustable_row, render_checkable_row, render_divider, render_row},
};

pub fn render_trainer_content(frame: &mut Frame, area: Rect, app: &App) {
    let selected = app.ui_state.popup_state.selected_option;

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(4), // Delays
        Constraint::Length(2), // Auto-progress
        Constraint::Length(2), // Divider
        Constraint::Length(1), // Reset progress
    ]);

    let [_padding, delays, auto_progress, divider, reset_progress] = layout.areas(area);

    render_delays_section(frame, delays, &app.settings, selected);

    render_checkable_row(
        frame,
        auto_progress,
        "Auto-advance next line",
        app.settings.auto_advance,
        selected == 3,
    );

    render_divider(frame, divider);

    render_row(
        frame,
        reset_progress,
        "Reset progress",
        Line::from(Span::styled("   Reset", Style::default().red())),
        selected == 4,
    );
}

fn render_delays_section(frame: &mut Frame, area: Rect, settings: &Settings, selected: usize) {
    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ]);

    let [top, middle, bottom] = layout.areas(area);

    render_adjustable_row(
        frame,
        top,
        "Bot move delay",
        settings.bot_move_delay,
        "ms",
        selected == 0,
    );
    render_adjustable_row(
        frame,
        middle,
        "Undo move delay",
        settings.undo_move_delay,
        "ms",
        selected == 1,
    );
    render_adjustable_row(
        frame,
        bottom,
        "Advance line delay",
        settings.advance_line_delay,
        "ms",
        selected == 2,
    );
}
