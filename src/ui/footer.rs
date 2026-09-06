use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Paragraph},
};

pub fn render_trainer_footer(frame: &mut Frame, area: Rect) {
    let block = Block::default();

    let paragraph = Paragraph::new("(h)elp / ?")
        .style(Style::new().italic())
        .alignment(Alignment::Right);

    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
