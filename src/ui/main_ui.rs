use ratatui::{
    Frame,
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Paragraph},
};

use crate::{
    app::App,
    constants::LOGO,
    state::ui_state::Views,
    ui::{menu::main_menu::render_menu_ui, trainer_ui::render_trainer_ui},
};

pub fn render(app: &mut App, frame: &mut Frame<'_>) {
    let area = frame.area();

    match app.ui_state.current_view {
        Views::Home => render_setup_ui(frame, app, area),
        Views::Trainer => render_trainer_ui(frame, app, area),
    }
}

pub fn render_setup_ui(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let area = frame.area();

    // Grid med åpninger
    //
    // Statistikk?
    //
    //

    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 10),  // Logo
                Constraint::Length(1),     // Description
                Constraint::Ratio(18, 20), // Openings
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
                Constraint::Ratio(1, 18),  // Right padding
                Constraint::Ratio(4, 18),  // Sidebar
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

    let logo_paragraph = Paragraph::new(LOGO)
        .alignment(Alignment::Center)
        .block(Block::default());
    let description_paragraph = Paragraph::new("Practice openings!".to_string())
        .centered()
        .block(Block::default());

    frame.render_widget(logo_paragraph, main_layout_horizontal[0]);
    frame.render_widget(description_paragraph, main_layout_horizontal[1]);
}
