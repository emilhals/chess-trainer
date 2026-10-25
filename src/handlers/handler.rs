use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::{
    app::{App, AppResult},
    handlers::{
        home::handle_home_view_events, popup::handle_popup_events,
        trainer::handle_trainer_view_events,
    },
    state::ui_state::Views,
};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Only process key press events.
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.ui_state.popup_state.active_popup.is_some() {
        handle_popup_events(app, key_event)?;
        return Ok(());
    }

    handle_view_input(app, key_event);

    Ok(())
}

pub fn handle_view_input(app: &mut App, key_event: KeyEvent) {
    match &app.ui_state.current_view {
        Views::Home => handle_home_view_events(app, key_event),
        Views::Trainer => handle_trainer_view_events(app, key_event),
    }
}

pub fn handle_chess_input(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.trainer.go_up(),
        KeyCode::Down | KeyCode::Char('j') => app.trainer.go_down(),
        KeyCode::Left | KeyCode::Char('h') => app.trainer.go_left(),
        KeyCode::Right | KeyCode::Char('l') => app.trainer.go_right(),
        KeyCode::Char(' ') | KeyCode::Enter => app.trainer.process_cell_click(&app.settings),
        KeyCode::Esc => app.trainer.ui.unselect_cell(),
        _ => fallback_key_handler(app, key_event),
    }
}

// Fallback handler for global shortcuts.
pub fn fallback_key_handler(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('s') => app.ui_state.toggle_settings_view(),
        KeyCode::Char('q') => app.quit(),
        KeyCode::Backspace => app.reset_home(),
        _ => {}
    }
}
