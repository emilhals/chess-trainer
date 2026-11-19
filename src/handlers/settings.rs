use crate::{
    app::{App, AppResult},
    handlers::handler::fallback_key_handler,
    input::Key,
};

pub fn handle_settings_view_events(app: &mut App, key: Key) -> AppResult<()> {
    let num_options = app.ui_state.popup.selected_tab.num_options();

    match key {
        Key::Up | Key::Char('k') => app.ui_state.popup.cycle_options(num_options, false),
        Key::Down | Key::Char('j') => app.ui_state.popup.cycle_options(num_options, true),
        Key::Left | Key::Char('h') => {
            app.adjust_selected_setting(false);
        }
        Key::Right | Key::Char('l') => {
            app.adjust_selected_setting(true);
        }
        Key::Enter | Key::Char(' ') => app.toggle_selected_setting(),
        Key::Tab => app.ui_state.popup.cycle_tabs(),
        Key::Esc | Key::Char('b') => app.ui_state.toggle_settings_view(),
        _ => fallback_key_handler(app, key),
    }

    Ok(())
}
