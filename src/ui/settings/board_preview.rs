use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::Block,
};

use crate::skin::Skin;

pub fn render_board_preview(frame: &mut Frame, area: Rect, skin: &Skin) {
    let width = area.width / 10;
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

            let cell_color: Color = if (i + j).is_multiple_of(2) {
                skin.light_square
            } else {
                skin.dark_square
            };

            render_single_cell(frame, square, cell_color);
        }
    }
    /*for rank in 0..7 {
        for file in 0..7 {
            let square_area = Rect {
                x: area.x + file * square_width,
                y: area.y + rank * square_height,
                width: square_width,
                height: square_height,
            };

            let cell_color: Color = if (rank + file).is_multiple_of(2) {
                skin.light_square
            } else {
                skin.dark_square
            };

            render_cell(frame, square_area, cell_color);
        }
    }*/
}

pub fn render_single_cell(frame: &mut Frame, area: Rect, cell_color: Color) {
    let cell = Block::default().bg(cell_color);
    frame.render_widget(cell, area);
}
