use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::{self, line},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::App,
    constants::{BLACK_PLAYER_UNICODE, LOGO, WHITE_PLAYER_UNICODE},
    trainer::openings::{Opening, Openings},
    ui::footer::render_footer,
};

pub fn render_menu_ui(frame: &mut Frame, app: &App, area: Rect) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 10), // Logo
                Constraint::Length(1),    // Subtitle
                Constraint::Length(1),    // Padding
                Constraint::Min(1),       // Openings
                Constraint::Length(2),    // Footer
            ]
            .as_ref(),
        )
        .split(area);

    let (main_layout_vertical, _) = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(26), Constraint::Length(26)].as_ref())
        .split_with_spacers(main_layout_horizontal[3]);

    let logo_paragraph = Paragraph::new(LOGO)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(logo_paragraph, main_layout_horizontal[0]);

    let description_paragraph =
        Paragraph::new("Practice openings to become a chess master!".to_string())
            .centered()
            .bold()
            .block(Block::default());
    frame.render_widget(description_paragraph, main_layout_horizontal[1]);

    let openings = app.trainer.openings.get_openings();
    for i in 0..openings.len() {
        let is_selected = app.ui_state.menu_cursor == i as u8;

        render_opening_block(
            frame,
            main_layout_vertical[i],
            openings[i].clone(),
            is_selected,
        );
    }

    render_footer(frame, app, main_layout_horizontal[4]);
}

fn render_opening_block(
    frame: &mut Frame<'_>,
    area: Rect,
    opening: (String, Opening),
    selected: bool,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20)].as_ref())
        .split(area);

    let formatted_name = Openings::format_name(opening.0);

    let border_style = match selected {
        true => Style::default().fg(Color::Magenta),
        false => Style::default().fg(Color::White),
    };

    let corners_only = symbols::border::Set {
        top_left: line::NORMAL.top_left,
        top_right: line::NORMAL.top_right,
        bottom_left: line::NORMAL.bottom_left,
        bottom_right: line::NORMAL.bottom_right,
        vertical_left: " ",
        vertical_right: " ",
        horizontal_top: " ",
        horizontal_bottom: " ",
    };

    let opening_block = match selected {
        true => Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style),
        false => Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_set(corners_only)
            .border_style(border_style),
    };
    frame.render_widget(opening_block, layout[0]);

    let block_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1), // Top padding
                Constraint::Length(2), // Opening name + padding
                Constraint::Length(1), // Line amount
                Constraint::Length(1), // Padding
                Constraint::Length(1), // Player color
            ]
            .as_ref(),
        )
        .split(layout[0]);

    let opening_name_paragraph = Paragraph::new(formatted_name.to_string()).centered().bold();
    frame.render_widget(opening_name_paragraph, block_layout[1]);

    let line_amount_paragraph =
        Paragraph::new(format!("{} lines total", opening.1.lines.len())).centered();
    frame.render_widget(line_amount_paragraph, block_layout[2]);

    let player_color = match opening.1.player.as_str() {
        "white" => WHITE_PLAYER_UNICODE,
        "black" => BLACK_PLAYER_UNICODE,
        _ => "",
    };
    let player_color_paragraph = Paragraph::new(Span::styled(
        player_color,
        Style::default().fg(Color::White),
    ))
    .centered();
    frame.render_widget(player_color_paragraph, block_layout[3]);
}
