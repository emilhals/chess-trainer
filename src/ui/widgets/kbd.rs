use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Paragraph, Widget},
};

#[derive(Debug, Default, Setters)]
pub struct Kbd {
    key: String,
    content: String,
}

impl Widget for Kbd {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);

        let mut lines = vec![];
        lines.push(Line::from(vec![
            Span::styled("_", Style::default().bg(Color::Gray)),
            Span::styled(
                format!("{:^2}", self.key),
                Style::default().italic().fg(Color::Black).bg(Color::Gray),
            ),
            Span::styled(" ", Style::default()),
            Span::styled(self.content, Style::default().fg(Color::Gray)),
        ]));

        let text = Text::from(lines);

        let block = Block::new();
        Paragraph::new(text).block(block).render(area, buf);
    }
}
