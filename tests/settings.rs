#[cfg(test)]
mod tests {
    use chess_trainer::{
        app::{App, AppResult, player::PlayerData},
        handlers::handler::handle_key_events,
        settings::Settings,
        skin::Skins,
        state::popup_state::{
            Button::Cancel, ConfirmationAction::ResetProgress, Popups::Settings as SettingPopup,
        },
        trainer::{Trainer, openings::Openings},
    };

    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::empty(),
        }
    }

    fn send(app: &mut App, keys: &[KeyCode]) -> AppResult<()> {
        for &k in keys {
            handle_key_events(key(k), app)?;
        }
        Ok(())
    }

    fn create_test_app() -> App {
        let openings = Openings::bundled().unwrap();
        let trainer = Trainer::new(openings);

        let player_data = PlayerData::default();
        let settings = Settings::default();

        let skins = Skins::bundled().unwrap();

        App::new(trainer, player_data, settings, skins)
    }

    #[test]
    fn confirmation_cancel_preserves_progress() -> AppResult<()> {
        let mut app = create_test_app();

        app.player_data.highest_streak = 10;

        app.ui_state.popup_state.open_confirmation(ResetProgress);
        app.ui_state.popup_state.selected_button = Cancel;

        send(&mut app, &[KeyCode::Enter])?;

        assert_eq!(app.player_data.highest_streak, 10);
        assert_eq!(app.ui_state.popup_state.active_popup, Some(SettingPopup));

        Ok(())
    }
}
