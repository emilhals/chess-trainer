use crate::{
    app::{App, player::PlayerData},
    settings::Settings,
    storage::{Storage, StorageError, StorageKey, load_json, save_json},
};

pub(super) fn load_player_data(storage: &dyn Storage) -> (PlayerData, bool) {
    match load_json(storage, StorageKey::PlayerData) {
        Ok(player) => (player, false),
        Err(StorageError::Invalid(err)) => {
            log::warn!("invalid data: {err}");
            (PlayerData::default(), true)
        }
        Err(err) => {
            log::warn!("could not open player data: {err}");

            (PlayerData::default(), false)
        }
    }
}

pub(super) fn load_settings(storage: &dyn Storage) -> Settings {
    match load_json(storage, StorageKey::Settings) {
        Ok(settings) => settings,
        Err(err) => {
            log::warn!("could not open settings: {err}");

            Settings::default()
        }
    }
}

impl App {
    pub fn save_player_data(&mut self) {
        if self.player_data_read_only {
            return;
        }

        if let Err(err) = save_json(
            &mut *self.storage,
            StorageKey::PlayerData,
            &self.player_data,
        ) {
            log::error!("could not save player data: {err}");
        }
    }

    pub fn save_settings(&mut self) {
        if let Err(err) = save_json(&mut *self.storage, StorageKey::Settings, &self.settings) {
            log::error!("could not save settings: {err}");
        }
    }

    pub fn reset_player_data(&mut self) {
        self.player_data_read_only = false;
        self.player_data = PlayerData::default();

        self.save_player_data();
    }
}
