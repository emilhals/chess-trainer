use crate::{
    app::App,
    handlers::handler::fallback_key_handler,
    input::{Key, MouseButton, MouseInput, MouseInputKind},
    state::{click_regions::ClickTarget, trainer_state::HintLevel},
};

pub fn handle_trainer_key(app: &mut App, key: Key) {
    match key {
        Key::Char('b') => {
            // Return to home menu - reset all game state
            app.reset_home();
        }
        Key::Char('n') => {
            if app.trainer.is_completed() {
                app.next_line();
            }
        }
        Key::Char('r') => {
            if app.trainer.is_completed() {
                app.trainer.replay_line();
            }
        }
        Key::Char('t') => match app.trainer.state.mode.hint_level {
            HintLevel::Low => {
                app.trainer.show_hint();
            }
            HintLevel::Solve => {
                app.trainer.apply_solution_move(app.settings.bot_move_delay);
            }
        },

        _ => handle_chess_input(app, key),
    }
}

pub fn handle_trainer_mouse(app: &mut App, mouse: MouseInput, target: Option<ClickTarget>) {
    let settings = &app.settings;
    match (mouse.kind, target) {
        (MouseInputKind::Release(MouseButton::Left), Some(ClickTarget::Square(coord))) => {
            app.trainer.drop_on_square(coord, settings);
        }
        (MouseInputKind::Press(MouseButton::Left), Some(ClickTarget::Square(coord))) => {
            app.trainer.press_square(coord, settings);
        }
        (MouseInputKind::Drag(MouseButton::Left), None) => {
            app.trainer.cancel_drag();
        }
        (MouseInputKind::Press(MouseButton::Right), _) => {
            app.trainer.ui.unselect_cell();
        }
        _ => {}
    }
}

fn handle_chess_input(app: &mut App, key: Key) {
    match key {
        Key::Up | Key::Char('k') => app.trainer.go_up(),
        Key::Down | Key::Char('j') => app.trainer.go_down(),
        Key::Left | Key::Char('h') => app.trainer.go_left(),
        Key::Right | Key::Char('l') => app.trainer.go_right(),
        Key::Char(' ') | Key::Enter => app.trainer.handle_cell_click(&app.settings),
        Key::Esc => app.trainer.ui.unselect_cell(),
        _ => fallback_key_handler(app, key),
    }
}
