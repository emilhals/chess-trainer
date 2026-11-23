use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Paragraph},
};
use shakmaty::Color as ShakmatyColor;

use crate::{
    app::App,
    constants::{BLACK_PLAYER_UNICODE, WHITE_PLAYER_UNICODE},
    state::trainer_state::TrainerMode,
    trainer::openings::{Opening, Openings},
};

#[derive(Clone, Copy, Default)]
pub struct CardState {
    pub selected: bool,
    pub focused: bool,
    pub hovered: bool,
}

pub fn render_opening_card(
    frame: &mut Frame<'_>,
    app: &App,
    area: Rect,
    name: &str,
    opening: &Opening,
    state: CardState,
) {
    let [area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7)].as_ref())
        .areas(area);

    let (border_style, text_style) = card_box_style(state.selected, state.focused, state.hovered);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    frame.render_widget(block, area);

    let [_, name_area, info_area, player_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Top padding
            Constraint::Length(1), // Opening name + padding
            Constraint::Length(1), // Completed lines / Highest streak
            Constraint::Length(1), // Player color
        ])
        .areas(area);

    let name_paragraph = Paragraph::new(Openings::format_name(name))
        .centered()
        .style(text_style);
    frame.render_widget(name_paragraph, name_area);

    let info = match app.trainer.state.mode.selected_mode {
        TrainerMode::Learn => format!(
            "{}/{} lines completed",
            app.player_data.completed_lines(name),
            opening.lines.len()
        ),
        TrainerMode::Drill => format!("Best streak: {}", app.player_data.best_streak(name)),
    };
    frame.render_widget(Paragraph::new(info).centered().style(text_style), info_area);

    let player = match opening.player_color() {
        ShakmatyColor::White => format!("White {WHITE_PLAYER_UNICODE}"),
        ShakmatyColor::Black => format!("Black {BLACK_PLAYER_UNICODE}"),
    };
    frame.render_widget(
        Paragraph::new(format!("Playing: {player}"))
            .centered()
            .style(text_style),
        player_area,
    );
}

fn card_box_style(selected: bool, focused: bool, hovered: bool) -> (Style, Style) {
    let border = Style::default();
    let text = Style::default();

    match (selected, focused, hovered) {
        // Card is selected while Openings is focused
        (true, true, _) => (border.magenta(), text.bold()),
        // Selected card, focus is on Mode Picker
        (true, false, true) => (border.white(), text.bold()),
        (true, false, false) => (border.gray(), text.bold()),
        // Not selected
        (false, _, true) => (border.gray(), text.gray()),
        (false, _, false) => (border.dark_gray(), text.dark_gray()),
    }
}
