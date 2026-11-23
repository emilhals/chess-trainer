use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
};

// Renders a divider used for footer and top panels
pub fn render_divider(frame: &mut Frame, area: Rect) {
    let divider_paragraph = Paragraph::new(Line::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(Color::DarkGray),
    ));

    frame.render_widget(divider_paragraph, area);
}
