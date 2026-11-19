use crate::{
    app::App, handlers::handler::fallback_key_handler, input::Key, state::menu_state::MenuFocus,
};

pub fn handle_menu_key(app: &mut App, key: Key) {
    let num_openings = app.trainer.openings.get_openings().count();

    match (app.ui_state.menu.focus, key) {
        (MenuFocus::ModeSelector, Key::Left | Key::Right | Key::Char('l') | Key::Char('h')) => {
            app.trainer.state.mode.toggle_mode();
        }
        (MenuFocus::ModeSelector, Key::Up | Key::Down | Key::Char('k') | Key::Char('j')) => {
            app.ui_state.menu.focus.toggle();
        }

        (MenuFocus::Openings, Key::Up | Key::Down | Key::Char('k') | Key::Char('j')) => {
            app.ui_state.menu.focus.toggle();
        }

        (MenuFocus::Openings, Key::Right | Key::Char('l')) => {
            app.ui_state.menu.select_next_opening(num_openings);
        }

        (MenuFocus::Openings, Key::Left | Key::Char('h')) => {
            app.ui_state.menu.select_previous_opening(num_openings);
        }

        (MenuFocus::Openings, Key::Enter | Key::Char(' ')) => {
            app.start_trainer(app.ui_state.menu.selected_opening_index);
        }

        (_, Key::Char('m')) => {
            app.trainer.state.mode.toggle_mode();
        }
        _ => fallback_key_handler(app, key),
    }
}
