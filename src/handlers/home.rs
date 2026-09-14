use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, handlers::handler::fallback_key_handler};

// Handles keyboard input in the home view.
pub fn handle_home_view_events(app: &mut App, key_event: KeyEvent) {
    const NUM_OPENINGS: u8 = 2;

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.ui_state.menu_cursor_up(NUM_OPENINGS),
        KeyCode::Down | KeyCode::Char('j') => app.ui_state.menu_cursor_down(NUM_OPENINGS),
        KeyCode::Left | KeyCode::Char('h') => app.ui_state.menu_cursor_up(NUM_OPENINGS),
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => {
            app.ui_state.menu_cursor_down(NUM_OPENINGS)
        }

        KeyCode::Enter | KeyCode::Char(' ') => app.menu_select(),
        _ => fallback_key_handler(app, key_event),
    }
}
