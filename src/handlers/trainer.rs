use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, handlers::handler::handle_chess_input, state::trainer_state::HintLevel};

pub fn handle_trainer_view_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('b') => {
            // Return to home menu - reset all game state
            app.reset_home();
        }
        KeyCode::Char('n') => {
            if app.trainer.is_completed() {
                app.trainer.next_line();
            }
        }
        KeyCode::Char('s') => match app.trainer.state.mode.hint_level {
            HintLevel::Low => {
                app.trainer.show_hint();
            }
            HintLevel::Solve => {
                app.trainer.apply_solution_move();
            }
        },

        _ => handle_chess_input(app, key_event),
    }
}
