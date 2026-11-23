use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::trainer::{Trainer, ui::UI};

impl UI {
    pub fn streak_render(
        &self,
        area: Rect,
        frame: &mut Frame,
        trainer: &Trainer,
        highest_streak: u32,
    ) {
        let info_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .title_top(format!("Streak: {}", trainer.state.current_streak))
            .title_alignment(Alignment::Center);

        let inner_block = info_block.inner(area);
        frame.render_widget(info_block, area);

        let content_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(inner_block);

        let best_streak =
            Paragraph::new(format!("Best: {}", highest_streak)).alignment(Alignment::Center);
        frame.render_widget(best_streak, content_area[1]);
    }
}
