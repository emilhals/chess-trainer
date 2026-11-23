use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
};
use shakmaty::Square;

use crate::{
    app::coord::Coord,
    skin::Skin,
    state::click_regions::{ClickRegions, ClickTarget},
    trainer::{board::Board, ui::CellRenderContext},
    ui::board::{cell::render_single_cell, helper::legal_moves},
    utils::flip_square_if_needed,
};

pub fn board_render(
    frame: &mut Frame<'_>,
    area: Rect,
    board: &Board,
    skin: &Skin,
    selected_square: Option<Square>,
    cursor_coord: Coord,
    regions: &mut ClickRegions,
) {
    let actual_square = selected_square.map(|s| flip_square_if_needed(s, board.is_flipped));

    render_board_grid(
        area,
        frame,
        board,
        skin,
        actual_square,
        cursor_coord,
        regions,
    );
}

fn render_board_grid(
    area: Rect,
    frame: &mut Frame<'_>,
    board: &Board,
    skin: &Skin,
    actual_square: Option<Square>,
    cursor_coord: Coord,
    regions: &mut ClickRegions,
) {
    let width = area.width / 8;
    let height = area.height / 8;

    let border_height = area.height / 2 - (4 * height);
    let border_width = area.width / 2 - (4 * width);

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

    let (last_move_from, last_move_to) = board.last_move();
    let legal_moves = legal_moves(board, actual_square);

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
            render_single_cell(frame, &ctx, skin, cursor_coord);
        }
    }
}

pub fn render_rank_labels(frame: &mut Frame, area: Rect, is_flipped: bool) {
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

pub fn render_file_labels(frame: &mut Frame, area: Rect, is_flipped: bool) {
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
