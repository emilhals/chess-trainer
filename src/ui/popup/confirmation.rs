use ratatui::{
    Frame,
    layout::{Alignment, Constraint, HorizontalAlignment::Center, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::{state::popup_state::Button, ui::components::centered_rect::centered_rect};

pub fn render_confirmation_popup(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    description: &str,
    selected_button: Button,
) {
    let area = centered_rect(45, 30, area);
    frame.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .title(title)
        .title_style(Style::new().bold())
        .title_alignment(Center);

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::vertical([
        Constraint::Length(4), // Description
        Constraint::Min(0),    // Buttons
        Constraint::Length(1), // Footer
    ]);

    let [description_area, buttons_area, footer_area] = layout.areas(inner_area);

    let description_paragraph = Paragraph::new(description)
        .wrap(Wrap { trim: true })
        .centered();

    frame.render_widget(description_paragraph, description_area);

    render_confirmation_buttons(frame, buttons_area, selected_button);

    render_footer(frame, footer_area);
}

fn render_confirmation_buttons(frame: &mut Frame, area: Rect, selected_button: Button) {
    let layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(12),
        Constraint::Length(2),
        Constraint::Length(12),
        Constraint::Fill(1),
    ]);

    let [_, reset_area, _, cancel_area, _] = layout.areas(area);

    let reset_style = if selected_button == Button::Confirm {
        Style::default().fg(Color::Red).bold()
    } else {
        Style::default()
    };

    let cancel_style = if selected_button == Button::Cancel {
        Style::default().fg(Color::Magenta).bold()
    } else {
        Style::default()
    };

    let reset = Paragraph::new("[ Reset ]")
        .alignment(Alignment::Center)
        .style(reset_style);

    let cancel = Paragraph::new("[ Cancel ]")
        .alignment(Alignment::Center)
        .style(cancel_style);

    frame.render_widget(reset, reset_area);
    frame.render_widget(cancel, cancel_area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let key_style = Style::default().fg(Color::Magenta).bold();
    let description_style = Style::default();
    let separator_style = Style::default().add_modifier(Modifier::DIM);

    let mut spans = vec![];
    spans.extend([
        Span::styled(" ← → ", key_style),
        Span::styled("Navigate", description_style),
        Span::styled(" • ", separator_style),
        Span::styled(" Enter ", key_style),
        Span::styled("Confirm", description_style),
        Span::styled(" • ", separator_style),
        Span::styled(" Esc ", key_style),
        Span::styled("Cancel", description_style),
    ]);

    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}
