use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph},
};

use crate::state::trainer_state::TrainerMode;

pub fn render_mode_selector(
    frame: &mut Frame,
    area: Rect,
    selected_mode: TrainerMode,
    is_focused: bool,
) {
    let layout = Layout::horizontal([
        Constraint::Length(5),
        Constraint::Length(13),
        Constraint::Length(9),
    ])
    .flex(Flex::Center);

    let [label, selected, unselected] = layout.areas(area);

    let mode_paragraph = Paragraph::new("MODE")
        .alignment(Alignment::Center)
        .block(Block::new().padding(Padding::top(1)));
    frame.render_widget(mode_paragraph, label);

    let border_style = if is_focused {
        Style::default().magenta()
    } else {
        Style::default().dark_gray()
    };

    let selected_line = if is_focused {
        Line::from(vec![
            Span::styled("‹ ", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(
                selected_mode.to_string().to_uppercase(),
                Style::default().bold(),
            ),
            Span::styled(" ›", Style::default().add_modifier(Modifier::DIM)),
        ])
    } else {
        Line::from(vec![Span::styled(
            selected_mode.to_string().to_uppercase(),
            Style::default().bold(),
        )])
    };

    let selected_paragraph = Paragraph::new(selected_line)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .style(border_style),
        )
        .alignment(Alignment::Center);
    frame.render_widget(selected_paragraph, selected);

    let unselected_string = if selected_mode == TrainerMode::Learn {
        "DRILL".to_string()
    } else {
        "LEARN".to_string()
    };

    let unselected_paragraph = Paragraph::new(unselected_string)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .add_modifier(Modifier::DIM),
        )
        .alignment(Alignment::Center);
    frame.render_widget(unselected_paragraph, unselected);
}
