use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
};

use crate::{skin::Skin, ui::widgets::big_number::BigNumber};

pub fn streak_render(
    area: Rect,
    frame: &mut Frame,
    current_streak: u32,
    highest_streak: u32,
    skin: &Skin,
) {
    let label = Line::styled(
        format!("Streak · Best:{highest_streak}"),
        Style::default().fg(Color::DarkGray),
    );

    let current_streak = BigNumber::default()
        .value(current_streak)
        .style(Style::default().fg(skin.streak))
        .alignment(Alignment::Center);

    let block_width = label.width().max(current_streak.desired_width() as usize) as u16;
    let block = area.centered_horizontally(Constraint::Length(block_width));

    let [label_area, streak_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(3)])
            .spacing(1)
            .areas(block);

    frame.render_widget(label, label_area);

    frame.render_widget(current_streak, streak_area);
}
