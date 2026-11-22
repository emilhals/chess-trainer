use crate::{
    app::{App, AppResult},
    handlers::{menu::handle_menu_key, popup::handle_popup_events, trainer::handle_trainer_key},
    input::Key,
    state::ui_state::Views,
};

pub fn handle_key_events(app: &mut App, key: Key) -> AppResult<()> {
    if app.ui_state.popup.active_popup.is_some() {
        handle_popup_events(app, key)?;
        return Ok(());
    }

    handle_view_input(app, key);

    Ok(())
}

pub fn handle_view_input(app: &mut App, key: Key) {
    match &app.ui_state.current_view {
        Views::Menu => handle_menu_key(app, key),
        Views::Trainer => handle_trainer_key(app, key),
    }
}

/// Fallback handler for global shortcuts.
pub fn fallback_key_handler(app: &mut App, key: Key) {
    match key {
        Key::Char('s') => app.ui_state.toggle_settings_view(),
        Key::Char('q') => app.quit(),
        Key::Backspace => app.reset_home(),
        _ => {}
    }
}
