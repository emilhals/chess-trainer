use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::trainer::openings::Openings;

pub fn render_top(
    frame: &mut Frame,
    area: Rect,
    opening_name: &str,
    current_line: usize,
    current_mode: &str,
) {
    let [left, right, _] = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(5),
        Constraint::Length(1),
    ])
    .areas(area);

    let formatted_name = Openings::format_name(opening_name);

    let line = Line::from(vec![
        Span::styled(" Chess Trainer", Style::default().fg(Color::Magenta)),
        Span::styled("  •  ", Style::default().add_modifier(Modifier::DIM)),
        Span::styled(formatted_name, Style::default()),
        Span::styled("  •  ", Style::default().add_modifier(Modifier::DIM)),
        Span::styled(format!("Line #{current_line}"), Style::default()),
    ]);

    frame.render_widget(line, left);

    frame.render_widget(current_mode, right);
}

pub fn render_bottom(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled(" m ", Style::default().fg(Color::Black).bg(Color::Magenta)),
        Span::styled(
            " toggle mode ",
            Style::default().add_modifier(Modifier::DIM),
        ),
        Span::styled(" c ", Style::default().fg(Color::Black).bg(Color::Magenta)),
        Span::styled(" cycle skin ", Style::default().add_modifier(Modifier::DIM)),
    ]);

    frame.render_widget(line, area);
}
