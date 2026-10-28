use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Default, Setters)]
pub struct Kbd {
    key: String,
    content: String,
}

impl Kbd {
    pub fn desired_width(&self) -> u16 {
        let key_width = UnicodeWidthStr::width(self.key.as_str());

        let content_width = UnicodeWidthStr::width(self.content.as_str());

        (key_width + content_width + 3) as u16
    }
}

impl Widget for Kbd {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);

        let lines = Line::from(vec![
            Span::styled(" ", Style::default().bg(Color::Gray)),
            Span::styled(
                self.key,
                Style::default().italic().fg(Color::Black).bg(Color::Gray),
            ),
            Span::styled(" ", Style::default().bg(Color::Gray)),
            Span::styled(" ", Style::default()),
            Span::styled(self.content, Style::default().fg(Color::Gray)),
        ]);

        let text = Text::from(lines);

        let block = Block::new();
        Paragraph::new(text).block(block).render(area, buf);
    }
}
