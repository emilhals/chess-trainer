use std::cmp;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Wrap},
};

use crate::{
    skin::Skin,
    state::trainer_state::{MoveFeedback, TrainerMode},
    trainer::Trainer,
    ui::widgets::big_number::BigNumber,
};

pub fn render(frame: &mut Frame, area: Rect, trainer: &Trainer, skin: &Skin) {
    let [guide, _, stats] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Fill(1),
        Constraint::Length(7),
    ])
    .areas(area);

    render_feedback(frame, guide, trainer);

    render_stats(
        frame,
        stats,
        trainer.state.mode.selected_mode,
        trainer.state.current_streak,
        0,
        skin,
    );
}

fn render_feedback(frame: &mut Frame, area: Rect, trainer: &Trainer) {
    if trainer.is_completed() {
        let lines = vec![
            Line::from(Span::styled(
                "Line complete!",
                Style::default().bold().light_green(),
            )),
            Line::from(Span::styled("Ready for the next line?", Style::default())),
        ];
        let text = Text::from(lines);

        let response_paragraph = Paragraph::new(text).alignment(Alignment::Center);
        render_centered(frame, area, response_paragraph.centered());
    }

    match (
        trainer.state.mode.selected_mode,
        trainer.state.move_feedback,
    ) {
        (TrainerMode::Learn, None) => {
            let (info, play) = trainer.current_instruction();

            let lines = vec![
                Line::from(Span::styled(info, Style::default().italic().dark_gray())),
                Line::from(Span::styled(" ", Style::default())),
                Line::from(Span::styled(play, Style::default().bold())),
            ];

            let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true }).centered();

            render_centered(frame, area, paragraph.centered());
        }

        (TrainerMode::Drill, Some(MoveFeedback::Incorrect)) => {
            let paragraph = Paragraph::new("Wrong move").centered();
            frame.render_widget(paragraph, area);
        }
        (_, _) => {}
    }
}

fn render_stats(
    frame: &mut Frame,
    area: Rect,
    current_mode: TrainerMode,
    current_streak: u32,
    highest_streak: u32,
    skin: &Skin,
) {
    match current_mode {
        TrainerMode::Learn => {}
        TrainerMode::Drill => {
            let label = Line::styled("streak", Style::default().fg(Color::DarkGray));
            let best_streak = cmp::max(current_streak, highest_streak);

            let current_streak = BigNumber::default()
                .value(current_streak)
                .style(Style::default().fg(skin.streak))
                .alignment(Alignment::Center);

            let block_width = label.width().max(current_streak.desired_width() as usize) as u16;
            let block = area.centered_horizontally(Constraint::Length(block_width));

            let [label_area, streak_area, best_area] = Layout::vertical([
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .areas(block);

            frame.render_widget(label, label_area);
            frame.render_widget(current_streak, streak_area);
            frame.render_widget(
                Paragraph::new(format!("best {best_streak}")).centered(),
                best_area,
            );
        }
    }
}
fn render_centered(frame: &mut Frame, area: Rect, paragraph: Paragraph) {
    let height = (paragraph.line_count(area.width) as u16).min(area.height);
    let area = area.centered_vertically(Constraint::Length(height));
    frame.render_widget(paragraph, area);
}
