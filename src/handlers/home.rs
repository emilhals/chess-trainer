use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, handlers::handler::fallback_key_handler};

// Handles keyboard input in the home view.
pub fn handle_home_view_events(app: &mut App, key_event: KeyEvent) {
    const MENU_ITEMS: u8 = {
        3 // Play, Settings, Quit
    };

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.ui_state.menu_cursor_up(MENU_ITEMS),
        KeyCode::Down | KeyCode::Char('j') => app.ui_state.menu_cursor_down(MENU_ITEMS),
        KeyCode::Enter | KeyCode::Char(' ') => app.menu_select(),
        _ => fallback_key_handler(app, key_event),
    }
}
