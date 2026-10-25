use crate::app::AppResult;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Default)]
pub struct PlayerData {
    pub highest_streak: u32,
    pub openings: HashMap<String, OpeningStats>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OpeningStats {
    pub highest_streak: u32,
    pub completed_lines: u32,
    pub lines: HashMap<String, LineStats>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct LineStats {
    pub completed: bool,
}

pub fn player_data_path() -> io::Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "emilhals", "chess-trainer")
        .ok_or_else(|| io::Error::other("Could not determine application data directory"))?;

    let data_dir = project_dirs.data_dir();

    fs::create_dir_all(data_dir)?;

    Ok(data_dir.join("player.json"))
}

impl PlayerData {
    pub fn from_file(path: impl AsRef<Path>) -> AppResult<PlayerData> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> AppResult<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;

        Ok(())
    }

    pub fn best_streak(&self, opening_name: &str) -> u32 {
        self.openings
            .get(opening_name)
            .map_or(0, |opening| opening.highest_streak)
    }

    pub fn set_best_streak(&mut self, opening_name: &str, current_streak: u32) {
        let opening_stats = self.openings.entry(opening_name.to_string()).or_default();
        opening_stats.highest_streak = opening_stats.highest_streak.max(current_streak);
    }

    pub fn completed_lines(&self, opening_name: &str) -> u32 {
        self.openings
            .get(opening_name)
            .map_or(0, |opening| opening.completed_lines)
    }

    pub fn set_completed(&mut self, opening_name: &str, line_name: &str) {
        let line_stats = self
            .openings
            .entry(opening_name.to_string())
            .or_default()
            .lines
            .entry(line_name.to_string())
            .or_default();

        line_stats.completed = true;
        self.openings
            .entry(opening_name.to_string())
            .or_default()
            .completed_lines += 1;
    }

    pub fn reset_progress(&mut self) -> AppResult<()> {
        let default_player_data = PlayerData::default();

        let path = player_data_path()?;
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &default_player_data)?;

        *self = default_player_data;

        Ok(())
    }
}
