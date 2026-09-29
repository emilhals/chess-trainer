pub mod coord;
pub mod menu;
pub mod player;

use std::{error, time::Instant};

use crate::{
    app::player::PlayerData,
    state::{trainer_state::TrainerMode, ui_state::UIState},
    trainer::Trainer,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    pub trainer: Trainer,
    pub ui_state: UIState,
    pub player_data: PlayerData,
}

impl App {
    pub fn new(trainer: Trainer, player_data: PlayerData) -> Self {
        Self {
            running: true,
            trainer: trainer,
            ui_state: UIState::default(),
            player_data: player_data,
        }
    }

    pub fn next_line(&mut self) {
        let old_streak = self.trainer.state.current_streak;
        let old_wrong_moves = self.trainer.state.wrong_moves;

        self.trainer.next_line(&self.player_data);

        let Some(opening_name) = &self.trainer.state.current_opening_name else {
            return;
        };

        if self.trainer.state.current_streak > old_streak {
            self.player_data
                .set_highest_streak(&opening_name, self.trainer.state.current_streak as u32);
        }

        if self.trainer.state.mode.selected_mode == TrainerMode::Learn && old_wrong_moves == 0 {
            let Some(opening_name) = self.trainer.state.current_opening_name.as_deref() else {
                return;
            };
            let line_name = &self.trainer.current_line().unwrap().name;

            self.player_data.set_completed(opening_name, line_name);
        }
    }

    pub fn tick(&mut self) {
        // Add a delay before moving the bot
        if let Some(bot_move_at) = self.trainer.state.bot_move_at {
            if Instant::now() >= bot_move_at {
                self.trainer.state.bot_move_at = None;

                if self.trainer.is_completed() {
                    return;
                }

                self.trainer.apply_bot_move();
            }
        }

        // Add a delay before moving back after a wrong move
        if let Some(undo_move_at) = self.trainer.state.undo_move_at {
            if Instant::now() >= undo_move_at {
                self.trainer.board.undo_move();

                self.trainer.state.validate_move_response = None;
                self.trainer.state.undo_move_at = None;
            }
        }

        // Add a delay before advancing to next line (Drill mode)
        if let Some(next_line_at) = self.trainer.state.next_line_at {
            if Instant::now() >= next_line_at {
                self.trainer.state.next_line_at = None;

                self.next_line();
            }
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
