use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::Paragraph,
};

pub fn render_general_content(frame: &mut Frame, area: Rect) {
    let rows = Layout::horizontal([Constraint::Length(2), Constraint::Min(0)]).split(area);

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(5),
        Constraint::Min(0),
    ]);

    let [_padding, global, trainer] = layout.areas(rows[1]);

    render_keybindings(
        frame,
        global,
        "Global",
        &[
            ("m", "Change trainer mode"),
            ("s", "Toggle settings menu"),
            ("q", "Quit application"),
        ],
    );

    render_keybindings(
        frame,
        trainer,
        "Trainer",
        &[
            ("↑ ↓ ← → or k j l h  ", "Move cursor"),
            ("Space", "Select piece"),
            ("Esc", "Deselect piece"),
            ("t", "Show hint"),
            ("r", "Restart line"),
            ("b", "Back to menu"),
        ],
    )
}

fn render_keybindings(frame: &mut Frame, area: Rect, section: &str, keybindings: &[(&str, &str)]) {
    let section_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let [top, content] = section_layout.areas(area);

    let title = Paragraph::new(section.to_string()).bold();
    frame.render_widget(title, top);

    let row_layout = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]);
    let [left, right] = row_layout.areas(content);

    let mut keys = vec![];
    let mut descriptions = vec![];
    for (key, description) in keybindings.iter() {
        descriptions.push(Line::raw(description.to_string()));
        keys.push(Line::styled(key.to_string(), Style::default().bold()));
    }

    let descriptions_text = Text::from(descriptions);
    frame.render_widget(Paragraph::new(descriptions_text), left);

    let keys_text = Text::from(keys);
    frame.render_widget(Paragraph::new(keys_text).alignment(Alignment::Left), right);
}
