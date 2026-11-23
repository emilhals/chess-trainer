pub mod board;
pub mod layout;
pub mod popup;
pub mod views;
pub mod widgets;

use ratatui::Frame;

use crate::{
    app::App,
    state::{click_regions::ClickRegions, popup_state::Popups, ui_state::Views},
    ui::{
        popup::{confirmation::render_confirmation_popup, settings::render_settings_menu},
        views::{menu::main_menu::render_menu_ui, trainer::render_trainer_ui},
    },
};
pub fn render(app: &mut App, frame: &mut Frame<'_>, regions: &mut ClickRegions) {
    let area = frame.area();

    match app.ui_state.current_view {
        Views::Menu => render_menu_ui(app, frame, area, regions),
        Views::Trainer => render_trainer_ui(app, frame, area, regions),
    }

    match app.ui_state.popup.active_popup {
        Some(Popups::Settings) => render_settings_menu(app, frame, area),
        Some(Popups::Confirmation(action)) => {
            render_confirmation_popup(
                frame,
                area,
                action.title(),
                action.description(),
                app.ui_state.popup.selected_button,
            );
        }
        _ => {}
    }
}
