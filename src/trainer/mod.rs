use std::time::{Duration, Instant};

use shakmaty::Color;

use crate::{
    app::player::PlayerData,
    state::trainer_state::{
        TrainerMode::{self},
        TrainerState,
    },
    trainer::{
        board::Board,
        openings::{Line, Openings},
        ui::UI,
    },
};

pub mod board;
pub mod input;
pub mod moves;
pub mod openings;
pub mod ui;

pub struct Trainer {
    pub board: Board,
    pub openings: Openings,
    pub state: TrainerState,
    pub ui: UI,
    pub player_turn: Color,
}

impl Trainer {
    pub fn new(openings: Openings) -> Self {
        Self {
            board: Board::default(),
            openings,
            state: TrainerState::default(),
            ui: UI::default(),
            player_turn: Color::White,
        }
    }

    // Updates board and trainer state for when the player is black
    pub fn prepare_black_player(&mut self, bot_move_delay: u64) {
        self.board.player_turn = Color::Black;
        self.board.is_flipped = true;
        self.state.bot_should_move = true;

        self.state.bot_move_at = Some(Instant::now() + Duration::from_millis(bot_move_delay));
    }

    pub fn current_line(&self) -> Option<&Line> {
        let opening_name = self.state.current_opening_name.as_deref()?;

        self.openings
            .get_line(opening_name, self.state.current_line_index)
    }

    pub fn current_instruction(&self) -> (String, String) {
        let Some(opening_name) = self.state.current_opening_name.as_deref() else {
            return (String::from(""), String::from(""));
        };

        let instruction = self.openings.get_instruction(
            opening_name,
            self.state.current_line_index,
            self.state.current_instruction_index,
        );

        (instruction.0, instruction.1)
    }

    pub fn find_next_uncompleted_line(&mut self, player_data: &PlayerData) {
        let Some(opening_name) = &self.state.current_opening_name else {
            return;
        };
        let Some(opening) = self.openings.get_opening(opening_name) else {
            return;
        };

        let next = opening.lines.iter().enumerate().find(|(_, line)| {
            let is_completed = player_data
                .openings
                .get(opening_name)
                .and_then(|opening_stats| opening_stats.lines.get(&line.name))
                .is_some_and(|line_stats| line_stats.completed);

            !is_completed
        });

        if let Some((index, _line)) = next {
            self.state.current_line_index = index;
        }
    }

    /// Reset board state and reset opening state related only to the line
    pub fn next_line(&mut self, player_data: &PlayerData) {
        self.board.reset();

        match self.state.mode.selected_mode {
            TrainerMode::Drill => {
                // Set a random next line
                let opening = self
                    .openings
                    .get_opening(self.state.current_opening_name.clone().unwrap().as_str());
                let line_count: usize = opening.unwrap().lines.len();

                // If no mistakes were made, increment current streak
                if !self.state.made_wrong_move {
                    self.state.current_streak += 1;
                } else {
                    self.state.current_streak = 0;
                }

                self.state.random_line(line_count);
                self.state.reset_line_progress();
            }
            TrainerMode::Learn => {
                self.find_next_uncompleted_line(player_data);
                self.state.reset_line_progress();
            }
        }

        self.state.made_wrong_move = false;
        self.state.wrong_moves = 0;
    }

    pub fn replay_line(&mut self) {
        self.state.made_wrong_move = false;
        self.board.reset();
        self.state.reset_line_progress();
    }

    pub fn is_completed(&self) -> bool {
        let Some(line) = self.current_line() else {
            return false;
        };

        line.is_completed(self.state.current_move_index)
    }

    pub fn is_input_blocked(&self) -> bool {
        self.state.bot_move_at.is_some()
            || self.state.undo_move_at.is_some()
            || self.state.next_line_at.is_some()
            || self.is_completed()
    }

    pub fn move_progression(&self) -> usize {
        let Some(line) = self.current_line() else {
            return 0;
        };

        (self.state.current_move_index * 100) / line.len()
    }
}
