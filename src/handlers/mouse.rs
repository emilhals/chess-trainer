use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{
    app::{App, AppResult},
    state::{click_regions::ClickTarget, menu_state::MenuFocus, ui_state::Views},
};

pub fn handle_mouse_events(mouse: MouseEvent, app: &mut App) -> AppResult<()> {
    if app.ui_state.mouse_disabled {
        return Ok(());
    }

    let target = app.ui_state.regions.target_at(mouse.column, mouse.row);

    if mouse.kind == MouseEventKind::Moved {
        if app.ui_state.hovered != target {
            app.ui_state.hovered = target;
        }
        return Ok(());
    }

    match &app.ui_state.current_view {
        Views::Menu => handle_menu_mouse(mouse, app, target),
        Views::Trainer => {}
    }

    Ok(())
}

fn handle_menu_mouse(mouse: MouseEvent, app: &mut App, target: Option<ClickTarget>) {
    let MouseEventKind::Down(MouseButton::Left) = mouse.kind else {
        return;
    };

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
