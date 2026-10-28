use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::App,
    constants::{BLACK_PLAYER_UNICODE, WHITE_PLAYER_UNICODE},
    state::trainer_state::TrainerMode,
    trainer::openings::{Opening, Openings},
};

pub fn render_opening_card(
    frame: &mut Frame<'_>,
    app: &App,
    area: Rect,
    opening: (String, Opening),
    is_selected: bool,
    is_focused: bool,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7)].as_ref())
        .split(area);

    let formatted_name = Openings::format_name(&opening.0);

    let border_style = if is_focused && is_selected {
        Style::default().magenta()
    } else {
        Style::default().dark_gray()
    };

    let opening_block = match is_selected {
        true => Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style),
        false => Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style),
    };
    frame.render_widget(opening_block, layout[0]);

    let block_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2), // Top padding
                Constraint::Length(1), // Opening name + padding
                Constraint::Length(1), // Completed lines / Highest streak
                Constraint::Length(1), // Padding
                Constraint::Length(1), // Player color
            ]
            .as_ref(),
        )
        .split(layout[0]);

    let paragraph_style = if is_focused && is_selected {
        Style::default().bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let opening_name_paragraph = Paragraph::new(formatted_name.to_string())
        .centered()
        .style(paragraph_style);
    frame.render_widget(opening_name_paragraph, block_layout[1]);

    let completed_lines = app.player_data.completed_lines(&opening.0);
    let highest_streak = app.player_data.best_streak(&opening.0);

    let info_str = match app.trainer.state.mode.selected_mode {
        TrainerMode::Learn => format!(
            "{}/{} lines completed",
            completed_lines,
            opening.1.lines.len()
        ),
        TrainerMode::Drill => format!("Best streak: {}", highest_streak),
    };

    let line_amount_paragraph = Paragraph::new(info_str).centered().style(paragraph_style);
    frame.render_widget(line_amount_paragraph, block_layout[2]);

    let player_color = match opening.1.player.as_str() {
        "white" => format!("White {}", WHITE_PLAYER_UNICODE),
        "black" => format!("Black {}", BLACK_PLAYER_UNICODE),
        _ => "".to_string(),
    };
    let player_color_paragraph = Paragraph::new(Span::styled(
        format!("Playing: {}", player_color),
        paragraph_style,
    ))
    .centered();
    frame.render_widget(player_color_paragraph, block_layout[3]);
}
