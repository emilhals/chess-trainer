use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::Gauge,
};

use crate::trainer::Trainer;

pub fn progress_gauge_render(area: Rect, frame: &mut Frame, trainer: &Trainer) {
    let gauge = Gauge::default()
        .gauge_style(Style::new().light_magenta().on_dark_gray())
        .style(Modifier::BOLD)
        .label("")
        .percent(trainer.move_progression() as u16);

    frame.render_widget(gauge, area);
}
