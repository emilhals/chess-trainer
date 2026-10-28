use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{Block, Paragraph},
};

use crate::{
    app::App,
    constants::LOGO,
    state::{menu_state::MenuFocus, trainer_state::TrainerMode},
    ui::{
        components::divider::render_divider,
        menu::{mode_selector::render_mode_selector, opening_card::render_opening_card},
        widgets::kbd::Kbd,
    },
};

pub fn render_menu_ui(frame: &mut Frame, app: &App, area: Rect) {
    let layout = Layout::default().constraints([
        Constraint::Length(4), // Title
        Constraint::Length(1), // Spacing
        Constraint::Length(3), // Mode picker
        Constraint::Length(2), // Mode description
        Constraint::Length(1), // Divider
        Constraint::Length(2), // Openings label
        Constraint::Min(1),    // Openings
        Constraint::Length(3), // Footer
    ]);

    let [
        title,
        _,
        mode_picker,
        mode_description,
        divider,
        openings_label,
        openings,
        footer,
    ] = layout.areas(area);

    let block_width = 30;

    let title_paragraph = Paragraph::new(LOGO)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, title);

    render_mode_selector(
        frame,
        mode_picker,
        app.trainer.state.mode.selected_mode,
        app.ui_state.menu.focus == MenuFocus::ModeSelector,
    );

    let mode_description_str = if app.trainer.state.mode.selected_mode == TrainerMode::Learn {
        "Learn opening lines one move at a time"
    } else {
        "Challenge yourself to complete opening lines"
    };

    frame.render_widget(
        Paragraph::new(mode_description_str)
            .alignment(Alignment::Center)
            .dark_gray()
            .italic(),
        mode_description,
    );

    render_divider(frame, divider);

    let openings_paragraph = Paragraph::new("SELECT OPENING")
        .block(Block::default())
        .alignment(Alignment::Center);
    frame.render_widget(openings_paragraph, openings_label);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(block_width),
            Constraint::Length(block_width),
        ])
        .split(openings.centered_horizontally(Constraint::Length(60)));

    let openings = app.trainer.openings.get_openings();
    for i in 0..openings.len() {
        let is_selected = app.ui_state.menu.selected_opening_index == i;

        render_opening_card(
            frame,
            app,
            main_layout_vertical[i],
            openings[i].clone(),
            is_selected,
            app.ui_state.menu.focus == MenuFocus::Openings,
        );
    }

    render_footer(frame, footer, app.ui_state.menu.focus.keybinds());
}

pub fn render_footer(frame: &mut Frame<'_>, area: Rect, keybindings: &[(&str, &str)]) {
    const GLOBAL_KEYBINDINGS: &[(&str, &str)] = &[("s", "Settings"), ("q", "Quit")];

    let layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Divider
            Constraint::Length(1), // Footer
        ])
        .split(area);

    render_divider(frame, layout_horizontal[0]);

    let all_keybindings: Vec<(&str, &str)> = keybindings
        .iter()
        .chain(GLOBAL_KEYBINDINGS.iter())
        .cloned()
        .collect();

    let mut constraints = vec![];
    let mut kbds: Vec<Kbd> = vec![];

    for (i, &(key, description)) in all_keybindings.iter().enumerate() {
        if i == keybindings.len() {
            constraints.push(Constraint::Min(0));
        }

        let kbd = Kbd::default()
            .key(key.to_string())
            .content(description.to_string());

        constraints.push(Constraint::Length(kbd.desired_width()));
        kbds.push(kbd);
    }

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .spacing(1)
        .split(layout_horizontal[1]);

    for (i, kbd) in kbds.into_iter().enumerate() {
        let mut area_index = i;

        if i >= keybindings.len() {
            area_index = area_index.saturating_add(1);
        }

        frame.render_widget(kbd, layout[area_index]);
    }
}
