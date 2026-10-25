use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Default, Setters)]
pub struct Checkbox {
    checked: bool,
}

impl Widget for Checkbox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if self.checked { "☒" } else { "☐" };

        let block = Block::new();
        Paragraph::new(text).block(block).render(area, buf);
    }
}
