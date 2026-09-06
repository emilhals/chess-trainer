use ratatui::Frame;

use crate::{
    app::App,
    state::ui_state::Views,
    ui::{menu::main_menu::render_menu_ui, trainer_ui::render_trainer_ui},
};

pub fn render(app: &mut App, frame: &mut Frame<'_>) {
    let area = frame.area();

    match app.ui_state.current_view {
        Views::Home => render_menu_ui(frame, app, area),
        Views::Trainer => render_trainer_ui(frame, app, area),
    }
}
