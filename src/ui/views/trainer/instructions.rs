use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Paragraph, Wrap},
};

use crate::{
    settings::Settings,
    state::trainer_state::TrainerMode,
    trainer::{Trainer, ui::UI},
    ui::widgets::kbd::Kbd,
};

impl UI {
    pub fn instruction_render(
        &self,
        area: Rect,
        frame: &mut Frame<'_>,
        trainer: &Trainer,
        settings: &Settings,
    ) {
        let [content, kbd] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

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
            frame.render_widget(response_paragraph, content);

            // If auto advance is enabled in Drill mode, don't show any Kbds
            if trainer.state.mode.selected_mode == TrainerMode::Drill && settings.auto_advance {
                return;
            }

            let kbd_next = Kbd::default()
                .key("n".to_string())
                .content("Next line".to_string());
            let kbd_replay = Kbd::default()
                .key("r".to_string())
                .content("Replay".to_string());

            let mut kbd_area_constraints = vec![];
            if trainer.state.mode.selected_mode == TrainerMode::Drill {
                kbd_area_constraints.extend([Constraint::Length(kbd_next.desired_width())]);
            } else {
                kbd_area_constraints.extend([
                    Constraint::Length(kbd_next.desired_width()),
                    Constraint::Length(2),
                    Constraint::Length(kbd_replay.desired_width()),
                ]);
            }

            let kbd_area = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints(kbd_area_constraints)
                .split(kbd);

            frame.render_widget(kbd_next, kbd_area[0]);

            // Only show replay in Learn mode
            if trainer.state.mode.selected_mode == TrainerMode::Learn {
                frame.render_widget(kbd_replay, kbd_area[2]);
            }
        } else {
            // If we don't have a validation response, render the instruction
            if trainer.state.validate_move_response.is_none() {
                // In Drill mode, we don't show the instruction unless a wrong move has been made
                if trainer.state.mode.selected_mode == TrainerMode::Drill
                    && !trainer.state.made_wrong_move
                {
                    let paragraph = Paragraph::new("What's the correct move?");
                    frame.render_widget(paragraph, content);

                    let hint_level = trainer.state.mode.hint_level.to_string();

                    let kbd_hint = Kbd::default()
                        .key("t".to_string())
                        .content(hint_level.to_string());
                    frame.render_widget(kbd_hint, kbd);

                    return;
                }

                let (info, play) = trainer.current_instruction();

                let lines = vec![
                    Line::from(Span::styled(info, Style::new().italic().dark_gray())),
                    Line::from(Span::styled(" ", Style::new())),
                    Line::from(Span::styled(play, Style::new().bold())),
                ];

                let text = Text::from(lines);
                let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
                frame.render_widget(paragraph, content);
            } else {
                // We have a validation response, so we render it
                let response = trainer.state.validate_move_response.clone().unwrap();

                let response_paragraph = Paragraph::new(Line::from(response).bold())
                    .wrap(Wrap { trim: true })
                    .fg(Color::White);
                frame.render_widget(response_paragraph, content);
            }
        }
    }
}
