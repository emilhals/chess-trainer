use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppResult},
    handlers::{handler::fallback_key_handler, settings::handle_settings_view_events},
    state::popup_state::{Button, ConfirmationAction, Popups},
};

pub fn handle_popup_events(app: &mut App, key_event: KeyEvent) -> AppResult<()> {
    match app.ui_state.popup_state.active_popup {
        Some(Popups::Settings) => handle_settings_view_events(app, key_event)?,
        Some(Popups::Confirmation(action)) => handle_confirmation_input(app, key_event, action)?,
        _ => {}
    }

    Ok(())
}

pub fn handle_confirmation_input(
    app: &mut App,
    key_event: KeyEvent,
    action: ConfirmationAction,
) -> AppResult<()> {
    match key_event.code {
        KeyCode::Left | KeyCode::Char('h') => {
            app.ui_state.popup_state.toggle_button();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.ui_state.popup_state.toggle_button();
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if app.ui_state.popup_state.selected_button == Button::Confirm {
                app.execute_confirmation(action)?
            } else {
                app.ui_state.popup_state.open_settings();
            }
        }
        KeyCode::Esc | KeyCode::Char('b') => app.ui_state.toggle_settings_view(),
        _ => fallback_key_handler(app, key_event),
    }

    Ok(())
}
