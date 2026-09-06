use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

#[derive(Debug, Default, Setters)]
pub struct Kbd<'a> {
    #[setters(into)]
    content: Text<'a>,
}

impl Widget for Kbd<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);

        let block = Block::new();
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(Style::new().on_blue())
            .centered()
            .block(block)
            .render(area, buf);
    }
}
