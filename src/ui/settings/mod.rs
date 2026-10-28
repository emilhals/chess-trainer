pub mod appearance;
pub mod board_preview;
pub mod general;
pub mod trainer;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, HorizontalAlignment::Center, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Tabs},
};

use crate::{
    app::App,
    state::popup_state::SettingsTabs,
    ui::{
        components::centered_rect::centered_rect,
        settings::{
            appearance::render_appearence_content, general::render_general_content,
            trainer::render_trainer_content,
        },
    },
};

pub fn render_settings_menu(frame: &mut Frame, app: &App, area: Rect) {
    let area = centered_rect(60, 60, area);
    frame.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .title("Settings")
        .title_style(Style::new().bold())
        .title_alignment(Center);

    let inner_area = block.inner(area);

    frame.render_widget(block, area);

    let layout = Layout::vertical([
        Constraint::Length(1), // Tabs
        Constraint::Min(1),    // Content
        Constraint::Length(1), // Footer
    ]);

    let [tabs, content, footer] = layout.areas(inner_area);

    render_tabs(frame, tabs, app.ui_state.popup.selected_tab as usize);
    render_content(frame, content, app);

    render_footer(
        frame,
        footer,
        app.ui_state.popup.selected_tab,
        app.ui_state.popup.selected_option,
    );
}

pub fn render_tabs(frame: &mut Frame, area: Rect, selected_tab: usize) {
    let tabs = Tabs::new(vec!["General", "Appearance", "Trainer"])
        .style(Style::default())
        .highlight_style(Style::default().magenta().bold().underlined())
        .select(selected_tab)
        .divider(symbols::DOT)
        .padding(" ", " ");
    frame.render_widget(tabs, area);
}

pub fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    match app.ui_state.popup.selected_tab {
        SettingsTabs::General => render_general_content(frame, area),
        SettingsTabs::Appearance => render_appearence_content(frame, area, app),
        SettingsTabs::Trainer => render_trainer_content(frame, area, app),
    }
}

fn render_row(frame: &mut Frame, area: Rect, label: &str, value: Line, is_selected: bool) {
    let style = if is_selected {
        Style::default().bold()
    } else {
        Style::default()
    };

    let indicator_symbol = if is_selected { "›" } else { " " };

    let layout = Layout::horizontal([
        Constraint::Length(2),  // Selection indicator
        Constraint::Fill(1),    // Label
        Constraint::Length(20), // Value
    ]);

    let [indicator, left, right] = layout.areas(area);

    frame.render_widget(Paragraph::new(indicator_symbol).magenta(), indicator);
    frame.render_widget(Paragraph::new(label).style(style), left);
    frame.render_widget(Paragraph::new(value).style(style), right);
}

fn render_checkable_row(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    checked: bool,
    is_selected: bool,
) {
    let value = if checked {
        Line::from(vec![
            Span::raw("   "),
            Span::styled(" On ", Style::default()),
        ])
    } else {
        Line::from(vec![
            Span::raw("   "),
            Span::styled("Off", Style::default()),
        ])
    };

    render_row(frame, area, label, value, is_selected);
}

fn render_adjustable_row(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: u64,
    unit: &str,
    is_selected: bool,
) {
    let formatted_value = format!("{:>4}", value);
    let line = Line::from(vec![
        Span::styled("‹ ", Style::default().add_modifier(Modifier::DIM)),
        Span::styled(format!("{} {}", formatted_value, unit), Style::default()),
        Span::styled(" ›", Style::default().add_modifier(Modifier::DIM)),
    ]);

    render_row(frame, area, label, line, is_selected);
}

fn render_footer(
    frame: &mut Frame,
    area: Rect,
    selected_tab: SettingsTabs,
    selected_option: usize,
) {
    let key_style = Style::default().fg(Color::Magenta).bold();
    let description_style = Style::default();
    let separator_style = Style::default().add_modifier(Modifier::DIM);

    let mut spans = vec![];
    spans.extend([
        Span::styled(" ↑ ↓ ", key_style),
        Span::styled("Navigate", description_style),
    ]);

    let shortcuts = selected_tab.shortcuts(selected_option);

    for &(key, description) in shortcuts {
        spans.push(Span::styled(" • ", separator_style));
        spans.push(Span::styled(format!(" {} ", key), key_style));
        spans.push(Span::styled(description, description_style));
    }

    spans.extend([
        Span::styled(" • ", separator_style),
        Span::styled(" Tab ", key_style),
        Span::styled("Switch", description_style),
        Span::styled(" • ", separator_style),
        Span::styled(" Esc ", key_style),
        Span::styled("Close", description_style),
    ]);

    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}

pub fn render_divider(frame: &mut Frame, area: Rect) {
    let width = area.width.saturating_sub(4) as usize;
    let divider = format!("  {}", "─".repeat(width));

    frame.render_widget(
        Paragraph::new(divider).style(Style::default().add_modifier(Modifier::DIM)),
        area,
    );
}
