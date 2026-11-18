use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Padding, Paragraph},
};

use crate::state::{
    click_regions::{ClickRegions, ClickTarget},
    trainer_state::TrainerMode,
};

pub fn render_mode_selector(
    frame: &mut Frame,
    area: Rect,
    selected_mode: TrainerMode,
    is_focused: bool,
    hovered_mode: Option<TrainerMode>,
    regions: &mut ClickRegions,
) {
    let [label, learn, drill] = Layout::horizontal([
        Constraint::Length(5),
        Constraint::Length(11),
        Constraint::Length(11),
    ])
    .flex(Flex::Center)
    .spacing(1)
    .areas(area);

    let mode_paragraph = Paragraph::new("MODE")
        .alignment(Alignment::Center)
        .block(Block::new().padding(Padding::top(1)));
    frame.render_widget(mode_paragraph, label);

    for (mode, rect) in [(TrainerMode::Learn, learn), (TrainerMode::Drill, drill)] {
        let selected = mode == selected_mode;
        let hovered = hovered_mode == Some(mode);

        let (border_style, text_style) = mode_box_style(selected, is_focused, hovered);

        let paragraph = Paragraph::new(mode.to_string().to_uppercase())
            .style(text_style)
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(border_style),
            );

        frame.render_widget(paragraph, rect);
        regions.push(rect, ClickTarget::ModeOption(mode));
    }
}

fn mode_box_style(selected: bool, focused: bool, hovered: bool) -> (Style, Style) {
    let bold = Style::default().bold();

    match (selected, focused, hovered) {
        (true, true, _) => (Style::default().magenta(), bold),
        (true, false, true) => (Style::default().white(), bold),
        (true, false, false) => (Style::default().dark_gray(), bold),
        (false, _, true) => (Style::default().gray(), Style::default()),
        (false, _, false) => (Style::default().dark_gray().dim(), Style::default()),
    }
}
