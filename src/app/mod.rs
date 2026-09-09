pub mod coord;
pub mod menu;

use std::{error, time::Instant};

use crate::{state::ui_state::UIState, trainer::Trainer};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    pub trainer: Trainer,
    pub ui_state: UIState,
}

impl App {
    pub fn new(trainer: Trainer) -> Self {
        Self {
            running: true,
            trainer: trainer,
            ui_state: UIState::default(),
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
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
