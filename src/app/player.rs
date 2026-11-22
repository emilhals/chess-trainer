use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct PlayerData {
    pub highest_streak: u32,
    pub openings: HashMap<String, OpeningStats>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OpeningStats {
    pub highest_streak: u32,
    pub completed_lines: u32,
    pub lines: HashMap<String, LineStats>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LineStats {
    pub completed: bool,
}

impl PlayerData {
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
        let opening_stats = self.openings.entry(opening_name.to_string()).or_default();
        let line_stats = opening_stats
            .lines
            .entry(line_name.to_string())
            .or_default();

        if line_stats.completed {
            return;
        }

        line_stats.completed = true;
        self.openings
            .entry(opening_name.to_string())
            .or_default()
            .completed_lines += 1;
    }
}
