use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn render_menu_ui(frame: &mut Frame, app: &App, area: Rect) {
    let menu_items: Vec<(&str, &str)> =
        vec![("Play Game", ""), ("Settings", "bla"), ("Quit", "da")];

    // Menu height depends on number of items, each takes 3 lines (item + description + spacing)
    // plus padding
    let menu_height = menu_items.len() as u16 * 3 + 4;

    let main_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 5),         // Title
                Constraint::Length(1),           // Subtitle
                Constraint::Length(menu_height), // Menu (height calculated above)
            ]
            .as_ref(),
        )
        .split(area);

    let title_paragraph = Paragraph::new("Chess Trainer")
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, main_layout_vertical[0]);

    let menu_area = main_layout_vertical[2];
    let mut menu_lines: Vec<Line<'_>> = vec![];

    for (i, (item, description)) in menu_items.iter().enumerate() {
        let is_selected = app.ui_state.menu_cursor == i as u8;

        let item_style = if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let item_text = format!("{}", item);
        menu_lines.push(Line::from(vec![Span::styled(item_text, item_style)]));

        // Add spacing between menu items.
        menu_lines.push(Line::from(""));
    }

    let menu_paragraph = Paragraph::new(menu_lines)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(menu_paragraph, menu_area);
}
