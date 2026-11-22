use crate::{
    app::{App, AppResult},
    handlers::trainer::handle_trainer_mouse,
    input::{MouseInput, MouseInputKind},
    state::{click_regions::ClickTarget, menu_state::MenuFocus, ui_state::Views},
};

pub fn handle_mouse_events(app: &mut App, mouse: MouseInput) -> AppResult<()> {
    if app.ui_state.mouse_disabled {
        return Ok(());
    }

    if app.ui_state.popup.active_popup.is_some() {
        return Ok(());
    }

    let target = app.ui_state.regions.target_at(mouse.column, mouse.row);

    if mouse.kind == MouseInputKind::Move {
        if app.ui_state.hovered != target {
            app.ui_state.hovered = target;
        }
        return Ok(());
    }

    match app.ui_state.current_view {
        Views::Menu => handle_menu_mouse(app, target),
        Views::Trainer => handle_trainer_mouse(app, mouse, target),
    }

    Ok(())
}

fn handle_menu_mouse(app: &mut App, target: Option<ClickTarget>) {
    match target {
        Some(ClickTarget::OpeningCard(i)) => {
            let num_openings = app.trainer.openings.get_openings().count();
            let menu = &mut app.ui_state.menu;

            menu.set_focus(MenuFocus::Openings);
            menu.select_opening(i, num_openings);
            app.start_trainer(i);
        }
        Some(ClickTarget::ModeOption(mode)) => {
            app.ui_state.menu.set_focus(MenuFocus::ModeSelector);
            app.trainer.state.mode.set_mode(mode);
        }
        _ => {}
    }
}
