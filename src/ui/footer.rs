use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::ui::{components::divider::render_divider, widgets::kbd::Kbd};

pub fn render_footer(frame: &mut Frame<'_>, area: Rect) {
    let layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(1),  // Padding
            Constraint::Length(20), // Trainer mode Kbd
            Constraint::Min(1),     // Fill space to place the rest on the right side
            Constraint::Length(13), // Settings Kbd
            Constraint::Length(10), // Quit Kbd
        ])
        .split(layout_horizontal[1]);

    let mode_kbd = Kbd::default()
        .key("m".to_string())
        .content("Change mode".to_string());
    frame.render_widget(mode_kbd, layout[1]);

    let settings_kbd = Kbd::default()
        .key("s".to_string())
        .content("Settings".to_string());
    frame.render_widget(settings_kbd, layout[3]);

    let quit_kbd = Kbd::default()
        .key("q".to_string())
        .content("Quit".to_string());
    frame.render_widget(quit_kbd, layout[4]);

    render_divider(frame, layout_horizontal[0]);
}
