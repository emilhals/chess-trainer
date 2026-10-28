use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, handlers::handler::fallback_key_handler, state::menu_state::MenuFocus};

pub fn handle_menu_view_events(app: &mut App, key_event: KeyEvent) {
    let num_openings = app.trainer.openings.get_openings().len();

    match (app.ui_state.menu.focus, key_event.code) {
        (
            MenuFocus::ModeSelector,
            KeyCode::Left | KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('h'),
        ) => {
            app.trainer.state.mode.toggle_mode();
        }
        (
            MenuFocus::ModeSelector,
            KeyCode::Up | KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('j'),
        ) => {
            app.ui_state.menu.focus.toggle();
        }

        (
            MenuFocus::Openings,
            KeyCode::Up | KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('j'),
        ) => {
            app.ui_state.menu.focus.toggle();
        }

        (MenuFocus::Openings, KeyCode::Right | KeyCode::Char('l')) => {
            app.ui_state.menu.select_next_opening(num_openings);
        }

        (MenuFocus::Openings, KeyCode::Left | KeyCode::Char('h')) => {
            app.ui_state.menu.select_previous_opening(num_openings);
        }

        (MenuFocus::Openings, KeyCode::Enter | KeyCode::Char(' ')) => {
            app.menu_select();
        }

        (_, KeyCode::Char('m')) => {
            app.trainer.state.mode.toggle_mode();
        }
        _ => fallback_key_handler(app, key_event),
    }
}
