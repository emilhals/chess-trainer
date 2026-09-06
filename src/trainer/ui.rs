use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap},
};
use shakmaty::{Role, Square};

use crate::{
    app::coord::{Coord, MoveDirection},
    trainer::{Trainer, board::Board, openings::Openings},
    ui::{components::cell::render_cell, pieces::PieceSize, widgets::kbd::Kbd},
    utils::{flip_square_if_needed, get_coord_from_square, get_square_from_coord},
};

// Context for rendering a single cell of the board.
struct CellRenderContext<'a> {
    board: &'a Board,
    i: u8,
    j: u8,
    square: Rect,
    actual_square: Option<Square>,
    last_move_from: Option<Square>,
    last_move_to: Option<Square>,
    legal_moves: &'a [Coord],
}

pub struct UI {
    /// The cursor position
    pub cursor_coords: Coord,
    /// The old cursor position when unselecting a cell
    pub old_cursor_coords: Option<Coord>,
    /// The selected square
    pub selected_square: Option<Square>,
    /// The selected piece cursor when we already selected a piece
    pub selected_piece_cursor: i8,

    pub width: u16,
    pub height: u16,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            cursor_coords: Coord::default(),
            selected_square: None,
            selected_piece_cursor: 0,
            old_cursor_coords: None,
            width: 0,
            height: 0,
        }
    }
}

impl UI {
    #[must_use]
    pub fn is_cell_selected(&self) -> bool {
        self.selected_square.is_some()
    }

    pub fn cursor_up(&mut self) {
        self.cursor_coords.move_to(MoveDirection::Up);
    }

    pub fn cursor_down(&mut self) {
        self.cursor_coords.move_to(MoveDirection::Down);
    }

    pub fn cursor_left(&mut self) {
        self.cursor_coords.move_to(MoveDirection::Left);
    }

    pub fn cursor_right(&mut self) {
        self.cursor_coords.move_to(MoveDirection::Right);
    }

    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_square = None;
            self.selected_piece_cursor = 0;
        }
    }

    fn piece_centered_rect(cell: Rect, content_lines: u16) -> Rect {
        let content_height = content_lines.min(cell.height);
        let top_offset = cell.height.saturating_sub(content_height) / 2;

        Rect {
            x: cell.x,
            y: cell.y + top_offset,
            width: cell.width,
            height: content_height,
        }
    }

    // Helper function to render a piece paragraph. Returns the paragraph and its line count.
    fn render_piece_paragraph(
        &self,
        piece_type: Option<Role>,
        piece_color: Option<shakmaty::Color>,
        square: Rect,
    ) -> (Paragraph<'static>, u16) {
        use crate::ui::pieces::{
            bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
        };

        let piece_size = PieceSize::from_dimensions(square.height);

        let piece_str = match piece_type {
            Some(Role::King) => King::to_string(piece_size, piece_color),
            Some(Role::Queen) => Queen::to_string(piece_size, piece_color),
            Some(Role::Rook) => Rook::to_string(piece_size, piece_color),
            Some(Role::Bishop) => Bishop::to_string(piece_size, piece_color),
            Some(Role::Knight) => Knight::to_string(piece_size, piece_color),
            Some(Role::Pawn) => Pawn::to_string(piece_size, piece_color),
            None => " ".to_string(),
        };

        let piece_color = match piece_color {
            Some(shakmaty::Color::White) => Color::White,
            Some(shakmaty::Color::Black) => Color::Black,
            None => Color::DarkGray,
        };

        let line_count = piece_str.lines().count().max(1) as u16;

        (
            Paragraph::new(piece_str)
                .fg(piece_color)
                .alignment(Alignment::Center),
            line_count,
        )
    }

    fn get_last_move_squares(board: &Board) -> (Option<Square>, Option<Square>) {
        if board.move_history.is_empty() {
            return (None, None);
        }

        let Some(last_move) = board.move_history.last() else {
            return (None, None);
        };

        let last_move_from = last_move.from();
        let last_move_to = Some(last_move.to());

        (last_move_from, last_move_to)
    }

    fn get_valid_moves_for_render(
        &self,
        board: &Board,
        actual_square: Option<Square>,
    ) -> Vec<Coord> {
        let Some(square) = actual_square else {
            return vec![];
        };

        if !self.is_cell_selected() {
            return vec![];
        }

        let selected_piece_color = board.get_piece_color_at_square(&square);

        if let Some(color) = selected_piece_color
            && color == board.player_turn
        {
            let mut legal_moves: Vec<Coord> = board
                .get_legal_moves(board.player_turn, &square)
                .iter()
                .map(|&s| Coord::from(s))
                .collect();

            if board.is_flipped {
                legal_moves = legal_moves.iter().map(Coord::reverse).collect();
            }
            return legal_moves;
        }
        vec![]
    }

    pub fn info_render(&self, area: Rect, frame: &mut Frame<'_>, trainer: &Trainer) {
        let title = trainer
            .state
            .current_opening_name
            .as_ref()
            .map(|name| name.as_str())
            .unwrap_or("No opening");
        let formatted_title = Openings::format_name(title);

        let trainer_mode = trainer.state.mode.selected_mode.to_string();
        let line_index = trainer.state.current_line_index.to_string();

        let info_block = Block::default()
            .title(Line::from(trainer_mode).left_aligned().bold())
            .title(Line::from(formatted_title).centered())
            .title(Line::from(format!("#{}", line_index)).right_aligned())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        let inner_area = info_block.inner(right_panel_layout[0]);

        let instruction_str = trainer.current_instruction();
        let response_paragraph = Paragraph::new(Line::from(instruction_str).bold())
            .wrap(Wrap { trim: true })
            .fg(Color::White)
            .alignment(Alignment::Center);

        let hint_level = trainer.state.mode.hint_level;

        let kbd_mode = Kbd::default().content("m toggle mode");
        let kbd_hint =
            Kbd::default().content(format!("s {:#}", hint_level.to_string().to_lowercase()));

        let shortcuts_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(8),
                    Constraint::Length(1),
                    Constraint::Length(15),
                    Constraint::Length(4),
                ]
                .as_ref(),
            )
            .split(right_panel_layout[1]);

        frame.render_widget(response_paragraph, inner_area);
        frame.render_widget(info_block, right_panel_layout[0]);
        frame.render_widget(kbd_hint, shortcuts_area[1]);
        frame.render_widget(kbd_mode, shortcuts_area[3]);
    }

    pub fn progress_gauge_render(&self, area: Rect, frame: &mut Frame, trainer: &Trainer) {
        let gauge = Gauge::default()
            .gauge_style(Style::new().light_magenta().on_black())
            .style(Modifier::BOLD)
            .label("")
            .percent(trainer.move_progression() as u16);

        frame.render_widget(gauge, area);
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

    // Renders a single cell of the board.
    fn render_single_cell(&self, frame: &mut Frame<'_>, ctx: &CellRenderContext<'_>) {
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
