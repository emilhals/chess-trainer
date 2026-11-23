use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
};
use shakmaty::Square;

use crate::{
    app::coord::Coord,
    skin::Skin,
    state::click_regions::{ClickRegions, ClickTarget},
    trainer::{
        board::Board,
        ui::{CellRenderContext, UI},
    },
    ui::board::cell::render_cell,
    utils::{flip_square_if_needed, get_coord_from_square, get_square_from_coord},
};

impl UI {
    // Renders a single cell of the board.
    pub fn render_single_cell(
        &self,
        frame: &mut Frame<'_>,
        ctx: &CellRenderContext<'_>,
        skin: &Skin,
    ) {
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
            skin.light_square
        } else {
            skin.dark_square
        };

        // Where can the piece move?
        let is_cell_in_positions = legal_moves.contains(&current_rendering_coord);

        if is_selected_cell
            || last_move_from == Some(current_rendering_square)
            || last_move_to == Some(current_rendering_square) && !is_cell_in_positions
        {
            render_cell(frame, square, skin.last_move, None);
        } else if is_cell_in_positions {
            render_cell(frame, square, skin.legal_move, None);
        } else {
            let cell = Block::default().bg(cell_color);
            frame.render_widget(cell, square);
        }

        let coord = Coord::new(i, j);
        let square_index = get_square_from_coord(coord, board.is_flipped);
        let piece_role = board.get_role_at_square(&square_index);
        let piece_color = board.get_piece_color_at_square(&square_index);

        if is_cursor_cell {
            let cell = Block::default().bg(skin.cursor);

            frame.render_widget(cell, square);
        }

        let (paragraph, line_count) = self.render_piece_paragraph(piece_role, piece_color, square);
        let piece_area = Self::piece_centered_rect(square, line_count);
        frame.render_widget(paragraph, piece_area);
    }

    pub fn board_render(
        &mut self,
        area: Rect,
        frame: &mut Frame<'_>,
        board: &Board,
        skin: &Skin,
        regions: &mut ClickRegions,
    ) {
        let actual_square = self
            .selected_square
            .map(|s| flip_square_if_needed(s, board.is_flipped));

        self.render_board_grid(area, frame, board, skin, actual_square, regions);
    }

    fn render_board_grid(
        &mut self,
        area: Rect,
        frame: &mut Frame<'_>,
        board: &Board,
        skin: &Skin,
        actual_square: Option<Square>,
        regions: &mut ClickRegions,
    ) {
        let width = area.width / 8;
        let height = area.height / 8;

        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);

        self.width = width;
        self.height = height;
        self.top_x = area.x + border_width;
        self.top_y = area.y + border_height;

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

                regions.push(square, ClickTarget::Square(Coord::new(i, j)));
                self.render_single_cell(frame, &ctx, skin);
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
