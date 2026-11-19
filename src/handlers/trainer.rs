use ratatui::crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

use crate::{
    app::App,
    handlers::handler::handle_chess_input,
    state::{click_regions::ClickTarget, trainer_state::HintLevel},
};

pub fn handle_trainer_view_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('b') => {
            // Return to home menu - reset all game state
            app.reset_home();
        }
        KeyCode::Char('n') => {
            if app.trainer.is_completed() {
                app.next_line();
            }
        }

        KeyCode::Char('r') => {
            if app.trainer.is_completed() {
                app.trainer.replay_line();
            }
        }

        KeyCode::Char('t') => match app.trainer.state.mode.hint_level {
            HintLevel::Low => {
                app.trainer.show_hint();
            }
            HintLevel::Solve => {
                app.trainer.apply_solution_move(app.settings.bot_move_delay);
            }
        },

        _ => handle_chess_input(app, key_event),
    }
}

pub fn handle_trainer_mouse(app: &mut App, mouse: MouseEvent, target: Option<ClickTarget>) {
    if mouse.column < app.trainer.ui.top_x || mouse.row < app.trainer.ui.top_y {
        return;
    }

    if app.trainer.ui.width == 0 || app.trainer.ui.height == 0 {
        return;
    }

    let x = (mouse.column - app.trainer.ui.top_x) / app.trainer.ui.width;
    let y = (mouse.row - app.trainer.ui.top_y) / app.trainer.ui.height;

    if x > 7 || y > 7 {
        return;
    }

    let settings = &app.settings;

    match (mouse.kind, target) {
        (MouseEventKind::Down(MouseButton::Left), Some(ClickTarget::Square(coord))) => {
            app.trainer.press_square(coord, settings);
        }
        (MouseEventKind::Up(MouseButton::Left), Some(ClickTarget::Square(coord))) => {
            app.trainer.drop_on_square(coord, settings);
        }
        (MouseEventKind::Up(MouseButton::Left), _) => {
            app.trainer.cancel_drag();
        }
        (MouseEventKind::Down(MouseButton::Right), _) => {
            app.trainer.ui.unselect_cell();
        }
        _ => {}
    }
}
