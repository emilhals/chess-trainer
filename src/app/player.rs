use std::{any::Any, collections::HashMap, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::app::AppResult;

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

impl PlayerData {
    pub fn from_file(path: &str) -> AppResult<PlayerData> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn save(&self, path: &str) -> AppResult<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }

    pub fn best_streak(&self, opening_name: &str) -> u32 {
        self.openings
            .get(&opening_name.to_string())
            .map_or(0, |opening| opening.highest_streak)
    }

    pub fn set_best_streak(&mut self, opening_name: &str, current_streak: u32) {
        let opening_stats = self.openings.entry(opening_name.to_string()).or_default();
        opening_stats.highest_streak = opening_stats.highest_streak.max(current_streak);
    }

    pub fn completed_lines(&self, opening_name: &str) -> u32 {
        self.openings
            .get(&opening_name.to_string())
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
}
