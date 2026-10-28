use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppResult},
    handlers::handler::fallback_key_handler,
};

pub fn handle_settings_view_events(app: &mut App, key_event: KeyEvent) -> AppResult<()> {
    let num_options = app.ui_state.popup.selected_tab.num_options();

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.ui_state.popup.cycle_options(num_options, false),
        KeyCode::Down | KeyCode::Char('j') => app.ui_state.popup.cycle_options(num_options, true),
        KeyCode::Left | KeyCode::Char('h') => {
            app.adjust_selected_setting(false);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.adjust_selected_setting(true);
        }
        KeyCode::Enter | KeyCode::Char(' ') => app.toggle_selected_setting(),
        KeyCode::Tab => app.ui_state.popup.cycle_tabs(),
        KeyCode::Esc | KeyCode::Char('b') => app.ui_state.toggle_settings_view(),
        _ => fallback_key_handler(app, key_event),
    }

    Ok(())
}
