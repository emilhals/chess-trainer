use ratatui::Frame;

use crate::{
    app::App,
    state::{popup_state::Popups, ui_state::Views},
    ui::{
        menu::main_menu::render_menu_ui, popup::confirmation::render_confirmation_popup,
        settings::render_settings_menu, trainer_ui::render_trainer_ui,
    },
};

pub fn render(app: &mut App, frame: &mut Frame<'_>) {
    let area = frame.area();

    match app.ui_state.current_view {
        Views::Menu => render_menu_ui(frame, app, area),
        Views::Trainer => render_trainer_ui(frame, app, area),
    }

    match app.ui_state.popup.active_popup {
        Some(Popups::Settings) => render_settings_menu(frame, app, area),
        Some(Popups::Confirmation(action)) => {
            render_confirmation_popup(
                frame,
                area,
                action.title(),
                action.description(),
                app.ui_state.popup.selected_button,
            );
        }
        _ => {}
    }
}
