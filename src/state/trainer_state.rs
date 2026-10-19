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

impl ToString for HintLevel {
    fn to_string(&self) -> String {
        match self {
            HintLevel::Low => String::from("Hint"),
            HintLevel::Solve => String::from("Solve"),
        }
    }
}

impl ToString for TrainerMode {
    fn to_string(&self) -> String {
        match self {
            TrainerMode::Learn => String::from("Learn"),
            TrainerMode::Drill => String::from("Drill"),
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
            selected_color: self.selected_color.clone(),
            selected_mode: self.selected_mode.clone(),
            hint_level: self.hint_level.clone(),
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
    pub bot_move_delay: u64,

    pub undo_move_at: Option<Instant>,
    pub undo_move_delay: u64,

    pub next_line_at: Option<Instant>,
}

impl Default for TrainerState {
    fn default() -> Self {
        Self {
            mode: TrainerModeState::default(),

            current_opening_name: None,
            current_line_index: 0,
            current_move_index: 0,
            current_instruction_index: 0,

            correct_move: None,
            validate_move_response: None,
            made_wrong_move: false,

            wrong_moves: 0,
            current_streak: 0,

            bot_should_move: false,
            bot_move_at: None,
            bot_move_delay: 700,

            undo_move_at: None,
            undo_move_delay: 700,

            next_line_at: None,
        }
    }
}

impl Clone for TrainerState {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode.clone(),

            current_opening_name: self.current_opening_name.clone(),
            current_line_index: self.current_line_index.clone(),
            current_move_index: self.current_move_index.clone(),
            current_instruction_index: self.current_instruction_index.clone(),

            correct_move: self.correct_move.clone(),
            validate_move_response: self.validate_move_response.clone(),
            made_wrong_move: false,

            wrong_moves: 0,
            current_streak: 0,

            bot_should_move: self.bot_should_move.clone(),
            bot_move_at: self.bot_move_at.clone(),
            bot_move_delay: self.bot_move_delay.clone(),

            undo_move_at: self.undo_move_at.clone(),
            undo_move_delay: self.undo_move_delay.clone(),

            next_line_at: self.next_line_at.clone(),
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
