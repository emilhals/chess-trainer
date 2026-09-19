use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap},
};
use shakmaty::Square;

use crate::{
    app::coord::Coord,
    state::trainer_state::TrainerMode,
    trainer::{Trainer, UI, board::Board, openings::Openings, ui::CellRenderContext},
    ui::{
        components::{cell::render_cell, divider::render_divider},
        widgets::kbd::Kbd,
    },
    utils::{flip_square_if_needed, get_coord_from_square, get_square_from_coord},
};

impl UI {
    pub fn streak_render(&self, area: Rect, frame: &mut Frame, trainer: &Trainer) {
        let streak_paragraph = Paragraph::new(format!("Streak: {}", trainer.state.current_streak));

        let info_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded);

        frame.render_widget(streak_paragraph, area);
        frame.render_widget(info_block, area);
    }

    pub fn progress_gauge_render(&self, area: Rect, frame: &mut Frame, trainer: &Trainer) {
        let gauge = Gauge::default()
            .gauge_style(Style::new().light_magenta().on_dark_gray())
            .style(Modifier::BOLD)
            .label("")
            .percent(trainer.move_progression() as u16);

        frame.render_widget(gauge, area);
    }

    pub fn top_panel_render(&self, area: Rect, frame: &mut Frame<'_>, trainer: &Trainer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Padding
                Constraint::Length(1), // Kbd and paragraphs
                Constraint::Length(1), // Divider
            ])
            .split(area);

        let panel = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),  // Left padding
                Constraint::Length(10), // Back Kbd
                Constraint::Min(0),     // Opening name & current line index
                Constraint::Length(10), // Trainer mode paragraph
                Constraint::Length(1),  // Right padding
            ])
            .split(layout[1]);

        let back_kbd = Kbd::default()
            .key("b".to_string())
            .content("back".to_string());

        frame.render_widget(back_kbd, panel[1]);

        let title = trainer
            .state
            .current_opening_name
            .as_ref()
            .map(|name| name.as_str())
            .unwrap_or("No opening");
        let formatted_title = Openings::format_name(title.to_string());

        let line_index = trainer.state.current_line_index.to_string();

        let mut lines = vec![];
        lines.push(Line::from(vec![
            Span::styled(formatted_title, Style::default().bold()),
            Span::styled(" ", Style::default()),
            Span::styled(format!("#{}", line_index), Style::default()),
        ]));
        let text = Text::from(lines).centered();

        frame.render_widget(text, panel[2]);

        let mode_symbol = match trainer.state.mode.selected_mode {
            TrainerMode::Drill => "⚡",
            TrainerMode::Learn => "📚",
        };

        let mode_str = format!(
            "{} {}",
            mode_symbol,
            trainer.state.mode.selected_mode.to_string()
        );
        let mode_paragraph = Paragraph::new(mode_str).right_aligned();
        frame.render_widget(mode_paragraph, panel[3]);

        render_divider(frame, layout[2]);
    }

    pub fn instruction_render(&self, area: Rect, frame: &mut Frame<'_>, trainer: &Trainer) {
        let info_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        let inner_area = info_block.inner(right_panel_layout[0]);
        frame.render_widget(info_block, right_panel_layout[0]);

        if trainer.is_completed() {
            let lines = vec![
                Line::from(Span::styled("Good job!", Style::default().bold())),
                Line::from(Span::styled("You finished the line.", Style::default())),
            ];
            let text = Text::from(lines);

            let response_paragraph = Paragraph::new(text).alignment(Alignment::Center);
            frame.render_widget(response_paragraph, inner_area);

            let kbd_area = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(right_panel_layout[1]);

            let kbd_next = Kbd::default()
                .key("n".to_string())
                .content("next".to_string());
            frame.render_widget(kbd_next, kbd_area[0]);

            let kbd_restart = Kbd::default()
                .key("r".to_string())
                .content("restart".to_string());
            frame.render_widget(kbd_restart, kbd_area[1]);
        } else {
            let response_str = trainer
                .state
                .validate_move_response
                .clone()
                .unwrap_or(trainer.current_instruction());
            let response_paragraph = Paragraph::new(Line::from(response_str).bold())
                .wrap(Wrap { trim: true })
                .fg(Color::White)
                .alignment(Alignment::Center);
            frame.render_widget(response_paragraph, inner_area);

            let hint_level = trainer.state.mode.hint_level;
            let kbd_hint = Kbd::default()
                .key("s".to_string())
                .content(format!("{:#}", hint_level.to_string().to_lowercase()));
            frame.render_widget(kbd_hint, right_panel_layout[1]);
        }
    }

    // Renders a single cell of the board.
    pub fn render_single_cell(&self, frame: &mut Frame<'_>, ctx: &CellRenderContext<'_>) {
        let CellRenderContext {
            board,
            i,
            j,
            square,
            actual_square,
            last_move_from,
            last_move_to,
            legal_moves,
        } = *ctx;

        let current_rendering_coord = Coord::new(i, j);
        let current_rendering_square =
            get_square_from_coord(current_rendering_coord, board.is_flipped);

        // Safely determine if this specific cell is the one selected by the player
        let is_selected_cell = actual_square
            .is_some_and(|s| get_coord_from_square(s, board.is_flipped) == current_rendering_coord);

        let is_cursor_cell = self.cursor_coords == current_rendering_coord;

        if is_cursor_cell | is_selected_cell {
            render_cell(frame, square, Color::Cyan, None);
        }

        let cell_color: Color = if (i + j).is_multiple_of(2) {
            Color::Gray
        } else {
            Color::DarkGray
        };

        // Where can the piece move?
        let is_cell_in_positions = legal_moves.contains(&current_rendering_coord);

        if is_selected_cell
            || last_move_from == Some(current_rendering_square)
            || last_move_to == Some(current_rendering_square) && !is_cell_in_positions
        {
            let highlight_color = Color::Green;
            render_cell(frame, square, highlight_color, None);
        } else if is_cell_in_positions {
            render_cell(frame, square, Color::LightCyan, None);
        } else {
            let cell = Block::default().bg(cell_color);
            frame.render_widget(cell, square);
        }

        let coord = Coord::new(i, j);
        let square_index = get_square_from_coord(coord, board.is_flipped);
        let piece_role = board.get_role_at_square(&square_index);
        let piece_color = board.get_piece_color_at_square(&square_index);

        if is_cursor_cell {
            let cell = Block::default().bg(Color::Cyan);

            frame.render_widget(cell, square);
        }

        let (paragraph, line_count) = self.render_piece_paragraph(piece_role, piece_color, square);
        let piece_area = Self::piece_centered_rect(square, line_count);
        frame.render_widget(paragraph, piece_area);
    }

    pub fn board_render(&mut self, area: Rect, frame: &mut Frame<'_>, board: &Board) {
        let actual_square = self
            .selected_square
            .map(|s| flip_square_if_needed(s, board.is_flipped));

        self.render_board_grid(area, frame, board, actual_square);
    }

    fn render_board_grid(
        &mut self,
        area: Rect,
        frame: &mut Frame<'_>,
        board: &Board,
        actual_square: Option<Square>,
    ) {
        let width = area.width / 8;
        let height = area.height / 8;

        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);

        self.width = width;
        self.height = height;

        // There's 8 vertical lines.
        let columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(border_height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(border_height),
                ]
                .as_ref(),
            )
            .split(area);

        let (last_move_from, last_move_to) = Self::get_last_move_squares(board);
        let legal_moves = self.get_valid_moves_for_render(board, actual_square);

        for i in 0..8u8 {
            let lines = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(border_width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(border_width),
                    ]
                    .as_ref(),
                )
                .split(columns[i as usize + 1]);

            for j in 0..8u8 {
                let square = lines[j as usize + 1];
                let ctx = CellRenderContext {
                    board,
                    i,
                    j,
                    square,
                    actual_square,
                    last_move_from,
                    last_move_to,
                    legal_moves: &legal_moves,
                };

                self.render_single_cell(frame, &ctx);
            }
        }
    }

    pub fn render_rank_labels(&self, frame: &mut Frame, area: Rect, is_flipped: bool) {
        let ranks = if is_flipped {
            vec!["1", "2", "3", "4", "5", "6", "7", "8"]
        } else {
            vec!["8", "7", "6", "5", "4", "3", "2", "1"]
        };

        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(border_height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(border_height),
            ])
            .split(area);

        for (i, rank) in ranks.iter().enumerate() {
            let rank_area = layout[i + 1];

            let top_padding = if rank_area.height > 1 {
                (rank_area.height - 1) / 2
            } else {
                0
            };
            let bottom_padding = rank_area.height.saturating_sub(1 + top_padding);

            // Create text with emplty lines for vertical centering
            let mut lines: Vec<Line> = Vec::new();

            for _ in 0..top_padding {
                lines.push(Line::from(""));
            }

            lines.push(
                Line::from(*rank)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Gray)),
            );

            for _ in 0..bottom_padding {
                lines.push(Line::from(""));
            }

            let label = Paragraph::new(lines);
            frame.render_widget(label, rank_area);
        }
    }

    pub fn render_file_labels(&self, frame: &mut Frame, area: Rect, is_flipped: bool) {
        let files = if is_flipped {
            vec!["h", "g", "f", "e", "d", "c", "b", "a"]
        } else {
            vec!["a", "b", "c", "d", "e", "f", "g", "h"]
        };

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 8); 8])
            .split(area);

        for (i, file) in files.iter().enumerate() {
            let file_area = layout[i];

            let top_padding = u16::from(file_area.height > 2);
            let bottom_padding = file_area.height.saturating_sub(1 + top_padding);

            // Create text with emplty lines for vertical centering
            let mut lines: Vec<Line> = Vec::new();

            for _ in 0..top_padding {
                lines.push(Line::from(""));
            }
            lines.push(
                Line::from(*file)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Gray)),
            );
            for _ in 0..bottom_padding {
                lines.push(Line::from(""));
            }

            let label = Paragraph::new(lines);
            frame.render_widget(label, file_area);
        }
    }
}
