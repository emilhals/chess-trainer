use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use ratatui::crossterm::event::{self, Event as CrosstermEvent};

use crate::{
    app::AppResult,
    input::{Key, MouseInput},
};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Fired at the tick rate. Used for timed actions like bot moves and undo.
    Tick,
    /// Key press.
    Key(Key),
    /// Mouse input in terminal cells.
    Mouse(MouseInput),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    // Event receiver channel.
    receiver: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("unable to poll for event") {
                    let app_event = match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) => Key::try_from(e).map(Event::Key).ok(),
                        CrosstermEvent::Mouse(e) => MouseInput::try_from(e).map(Event::Mouse).ok(),
                        _ => None,
                    };

                    if let Some(event) = app_event {
                        sender.send(event).expect("failed to send terminal event");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        });
        Self { receiver }
    }

    /// Receive the next event from the handler thread.
    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
