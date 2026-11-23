use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::{
    state::trainer_state::TrainerMode,
    trainer::{Trainer, openings::Openings},
    ui::widgets::{divider::render_divider, kbd::Kbd},
};

pub fn top_panel_render(area: Rect, frame: &mut Frame<'_>, trainer: &Trainer) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Padding
            Constraint::Length(1), // Kbd and paragraphs
            Constraint::Length(1), // Divider
        ])
        .split(area);

    let panel = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(1),  // Left padding
            Constraint::Length(10), // Back Kbd
            Constraint::Min(0),     // Opening name & current line index
            Constraint::Length(10), // Trainer mode paragraph
            Constraint::Length(1),  // Right padding
        ])
        .split(layout[1]);

    let back_kbd = Kbd::default()
        .key("b".to_string())
        .content("Back".to_string());

    frame.render_widget(back_kbd, panel[1]);

    let title = trainer
        .state
        .current_opening_name
        .as_deref()
        .unwrap_or("No opening");
    let formatted_title = Openings::format_name(title);

    let line_index = trainer.state.current_line_index.to_string();

    let mut lines = vec![];
    lines.push(Line::from(vec![
        Span::styled(formatted_title, Style::default().bold()),
        Span::styled(" ", Style::default()),
        Span::styled(format!("⚬ Line {}", line_index), Style::default()),
    ]));
    let text = Text::from(lines).centered();

    frame.render_widget(text, panel[2]);

    let mode_symbol = match trainer.state.mode.selected_mode {
        TrainerMode::Drill => "⚡",
        TrainerMode::Learn => "📚",
    };

    let mode_str = format!("{} {}", mode_symbol, trainer.state.mode.selected_mode);
    let mode_paragraph = Paragraph::new(mode_str).right_aligned();
    frame.render_widget(mode_paragraph, panel[3]);

    render_divider(frame, layout[2]);
}
