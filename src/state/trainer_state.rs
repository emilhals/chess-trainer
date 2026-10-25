use std::fmt;
use std::time::Instant;

use shakmaty::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum TrainerMode {
    Learn,
    Drill,
}

#[derive(Clone, Copy, PartialEq)]
pub enum HintLevel {
    Low,
    Solve,
}

impl fmt::Display for HintLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HintLevel::Low => write!(f, "Hint"),
            HintLevel::Solve => write!(f, "Solve"),
        }
    }
}

impl fmt::Display for TrainerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrainerMode::Learn => write!(f, "Learn"),
            TrainerMode::Drill => write!(f, "Drill"),
        }
    }
}

#[derive(PartialEq)]
pub struct TrainerModeState {
    pub selected_color: Color,
    pub selected_mode: TrainerMode,
    pub hint_level: HintLevel,
}

impl Default for TrainerModeState {
    fn default() -> Self {
        Self {
            selected_color: Color::White,
            selected_mode: TrainerMode::Learn,
            hint_level: HintLevel::Low,
        }
    }
}

impl Clone for TrainerModeState {
    fn clone(&self) -> Self {
        Self {
            selected_color: self.selected_color,
            selected_mode: self.selected_mode,
            hint_level: self.hint_level,
        }
    }
}

impl TrainerModeState {
    pub fn increment_hint_level(&mut self) {
        match self.hint_level {
            HintLevel::Low => self.hint_level = HintLevel::Solve,
            HintLevel::Solve => self.hint_level = HintLevel::Low,
        }
    }

    pub fn toggle_mode(&mut self) {
        match self.selected_mode {
            TrainerMode::Learn => self.selected_mode = TrainerMode::Drill,
            TrainerMode::Drill => self.selected_mode = TrainerMode::Learn,
        }
    }
}

#[derive(Default)]
pub struct TrainerState {
    pub mode: TrainerModeState,

    pub current_opening_name: Option<String>,
    pub current_line_index: usize,
    pub current_move_index: usize,
    pub current_instruction_index: usize,

    pub correct_move: Option<bool>,
    pub validate_move_response: Option<String>,
    pub made_wrong_move: bool,

    pub wrong_moves: usize,
    pub current_streak: usize,

    pub bot_should_move: bool,
    pub bot_move_at: Option<Instant>,

    pub undo_move_at: Option<Instant>,

    pub next_line_at: Option<Instant>,
}

impl Clone for TrainerState {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode.clone(),

            current_opening_name: self.current_opening_name.clone(),
            current_line_index: self.current_line_index,
            current_move_index: self.current_move_index,
            current_instruction_index: self.current_instruction_index,

            correct_move: self.correct_move,
            validate_move_response: self.validate_move_response.clone(),
            made_wrong_move: false,

            wrong_moves: 0,
            current_streak: 0,

            bot_should_move: self.bot_should_move,

            bot_move_at: self.bot_move_at,
            undo_move_at: self.undo_move_at,
            next_line_at: self.next_line_at,
        }
    }
}

impl TrainerState {
    pub fn reset(&mut self) {
        self.current_line_index = 0;
        self.current_instruction_index = 0;
        self.current_move_index = 0;
        self.bot_should_move = false;
    }

    pub fn reset_for_current_line(&mut self) {
        self.current_instruction_index = 0;
        self.current_move_index = 0;
        self.bot_should_move = false;
    }

    pub fn reset_for_next_line(&mut self) {
        self.current_instruction_index = 0;
        self.current_move_index = 0;
        self.bot_should_move = false;
    }

    pub fn reset_for_random_line(&mut self, line_length: usize) {
        // Set a random line index but exclude the current one
        let n = rand::random_range(0..line_length);
        let new_line_index = if n == self.current_line_index {
            n + 1
        } else {
            n
        };

        self.current_line_index = new_line_index;

        self.current_instruction_index = 0;
        self.current_move_index = 0;
        self.bot_should_move = false;
    }
}
