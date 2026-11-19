use crate::{
    app::{App, AppResult},
    handlers::{handler::fallback_key_handler, settings::handle_settings_view_events},
    input::Key,
    state::popup_state::{Button, ConfirmationAction, Popups},
};

pub fn handle_popup_events(app: &mut App, key: Key) -> AppResult<()> {
    match app.ui_state.popup.active_popup {
        Some(Popups::Settings) => handle_settings_view_events(app, key)?,
        Some(Popups::Confirmation(action)) => handle_confirmation_input(app, key, action)?,
        _ => {}
    }

    Ok(())
}

pub fn handle_confirmation_input(
    app: &mut App,
    key: Key,
    action: ConfirmationAction,
) -> AppResult<()> {
    match key {
        Key::Left | Key::Char('h') => {
            app.ui_state.popup.toggle_button();
        }
        Key::Right | Key::Char('l') => {
            app.ui_state.popup.toggle_button();
        }
        Key::Enter | Key::Char(' ') => {
            if app.ui_state.popup.selected_button == Button::Confirm {
                app.execute_confirmation(action)?
            } else {
                app.ui_state.popup.open_settings();
            }
        }
        Key::Esc | Key::Char('b') => app.ui_state.toggle_settings_view(),
        _ => fallback_key_handler(app, key),
    }

    Ok(())
}
