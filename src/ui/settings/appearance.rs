use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
};

use crate::{
    app::App,
    skin::Skins,
    ui::settings::{board_preview::render_board_preview, render_checkable_row, render_row},
};

pub fn render_appearence_content(frame: &mut Frame, area: Rect, app: &App) {
    let selected = app.ui_state.popup.selected_option;

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1), // Coordinates
        Constraint::Length(1), // Padding
        Constraint::Length(1), // Skin selector
        Constraint::Length(1), // Padding
        Constraint::Min(0),    // Board preview
    ]);

    let [_, coords_area, _, selector_area, _, board_area] = layout.areas(area);

    render_checkable_row(
        frame,
        coords_area,
        "Show coordinates",
        app.settings.show_coords,
        selected == 0,
    );

    let skin_name = Skins::display_name(&app.settings.selected_skin);
    render_selector_row(frame, selector_area, &skin_name, selected == 1);

    let Some(skin) = app.skins.get(&app.settings.selected_skin) else {
        return;
    };
    render_board_preview(frame, board_area, skin);
}

fn render_selector_row(frame: &mut Frame, area: Rect, skin_name: &str, selected: bool) {
    let line = Line::from(vec![
        Span::styled("‹ ", Style::default()),
        Span::styled(skin_name, Style::default()),
        Span::styled(" ›", Style::default()),
    ]);

    render_row(frame, area, "Board skin", line, selected);
}
