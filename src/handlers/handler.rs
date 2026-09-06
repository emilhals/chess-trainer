use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::{
    app::{App, AppResult},
    handlers::{home::handle_home_view_events, trainer::handle_trainer_view_events},
    state::ui_state::Views,
};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Only process key press events.
    if key_event.kind != KeyEventKind::Press {
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
        KeyCode::Char(' ') | KeyCode::Enter => app.trainer.process_cell_click(),
        KeyCode::Esc => app.trainer.ui.unselect_cell(),
        _ => fallback_key_handler(app, key_event),
    }
}

// Fallback handler for global shortcuts.
pub fn fallback_key_handler(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Backspace => app.ui_state.go_to_previous_view(),
        _ => {}
    }
}
