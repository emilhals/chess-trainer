use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{app::AppResult, skin::Skins};
use std::{
    cmp::{max, min},
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub selected_skin: String,

    pub bot_move_delay: u64,
    pub undo_move_delay: u64,
    pub advance_line_delay: u64,

    pub show_coords: bool,
    pub auto_advance: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            selected_skin: "chessdotcom".to_string(),

            bot_move_delay: 700,
            undo_move_delay: 600,
            advance_line_delay: 1000,

            show_coords: true,
            auto_advance: true,
        }
    }
}

pub fn settings_path() -> io::Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "emilhals", "chess-trainer")
        .ok_or_else(|| io::Error::other("Could not determine application data directory"))?;

    let config_dir = project_dirs.config_dir();

    fs::create_dir_all(config_dir)?;

    Ok(config_dir.join("settings.json"))
}

impl Settings {
    pub fn load_or_create(path: impl AsRef<Path>) -> AppResult<Self> {
        let path = path.as_ref();

        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                Ok(serde_json::from_reader(reader)?)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let settings = Settings::default();
                settings.save(path)?;
                Ok(settings)
            }
            Err(error) => {
                log::warn!("Failed to open settings at {}: {}", path.display(), error);
                Err(error.into())
            }
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) -> AppResult<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;

        Ok(())
    }

    pub fn cycle_skin(&mut self, skins: Skins, forward: bool) {
        let names = skins.names();

        if names.is_empty() {
            return;
        }

        let next_index;
        if let Some(current_index) = names.iter().position(|&skin| skin == self.selected_skin) {
            if forward {
                next_index = (current_index + 1) % names.len();
            } else {
                next_index = (current_index + names.len() - 1) % names.len();
            }

            let Some(next_skin) = names.get(next_index) else {
                return;
            };
            self.selected_skin = next_skin.to_string();
        }
    }

    pub fn toggle_auto_advance(&mut self) {
        self.auto_advance = !self.auto_advance;
    }

    pub fn toggle_show_coords(&mut self) {
        self.show_coords = !self.show_coords;
    }

    pub fn adjust_bot_delay(&mut self, forward: bool) {
        self.bot_move_delay = Self::adjust_delay(self.bot_move_delay, forward)
    }

    pub fn adjust_undo_delay(&mut self, forward: bool) {
        self.undo_move_delay = Self::adjust_delay(self.undo_move_delay, forward)
    }

    pub fn adjust_advance_delay(&mut self, forward: bool) {
        self.advance_line_delay = Self::adjust_delay(self.advance_line_delay, forward)
    }

    fn adjust_delay(delay: u64, forward: bool) -> u64 {
        if forward {
            min(delay.saturating_add(50), 3000)
        } else {
            max(delay.saturating_sub(50), 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::Settings;

    #[test]
    fn load_missing_field() {
        let old_data = r#"
            {
                "undo_move_delay": 700,
                "bot_move_delay": 600
            }
        "#;

        let settings: Settings = serde_json::from_str(old_data).unwrap();

        assert_eq!(settings.undo_move_delay, 700);
        assert_eq!(settings.bot_move_delay, 600);
        assert_eq!(
            settings.advance_line_delay,
            Settings::default().advance_line_delay
        );
    }
}
