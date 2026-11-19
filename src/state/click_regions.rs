use ratatui::layout::{Position, Rect};

use crate::{app::coord::Coord, state::trainer_state::TrainerMode};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClickTarget {
    Square(Coord),

    ModeOption(TrainerMode),
    OpeningCard(usize),
    OpeningPrev,
    OpeningNext,
}

#[derive(Default)]
pub struct ClickRegions {
    regions: Vec<(Rect, ClickTarget)>,
}

impl ClickRegions {
    pub fn target_at(&self, col: u16, row: u16) -> Option<ClickTarget> {
        let pos = Position::new(col, row);
        self.regions
            .iter()
            .rev()
            .find(|(area, _)| area.contains(pos))
            .map(|(_, target)| *target)
    }

    pub fn push(&mut self, area: Rect, target: ClickTarget) {
        if !area.is_empty() {
            self.regions.push((area, target));
        }
    }

    pub fn clear(&mut self) {
        self.regions.clear();
    }
}
