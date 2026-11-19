use crate::app::{App, AppResult};
use crate::native::event::EventHandler;
use crate::state::click_regions::ClickRegions;
use ratatui::Terminal;
use ratatui::backend::Backend;

use crate::ui::main_ui;

#[derive(Debug)]
pub struct Tui<B: Backend> {
    // Interface to the terminal.
    terminal: Terminal<B>,
    // Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn draw(&mut self, app: &mut App) -> AppResult<()>
    where
        <B as Backend>::Error: 'static,
    {
        let mut regions = ClickRegions::default();

        self.terminal
            .draw(|frame| main_ui::render(app, frame, &mut regions))?;
        app.ui_state.regions = regions;

        Ok(())
    }
}
