mod app;
mod event;
mod handlers;
mod state;
mod trainer;
mod ui;
mod utils;

use app::{App, AppResult};
use event::{Event, EventHandler};
use handlers::handler::handle_key_events;
use log::error;
use ui::tui::Tui;

use crate::trainer::{Trainer, openings::Openings};

fn main() -> AppResult<()> {
    initialize_panic_handler()?;

    // Enable mouse capture
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::EnableMouseCapture
    )?;

    // Load openings
    let openings: Openings = Openings::from_file("data/openings.json")?;

    // Create a trainer
    let trainer = Trainer::new(openings);

    // Create an application
    let mut app = App::new(trainer);

    // Initialize the terminal user interface
    let terminal = ratatui::try_init()?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    app.trainer.update_opening_state();
    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    ratatui::try_restore()?;
    // Free up the mouse
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::DisableMouseCapture
    )?;

    Ok(())
}

pub fn initialize_panic_handler() -> AppResult<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .display_location_section(true)
        .display_env_section(true)
        .into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal before printing panic information.
        if let Err(err) = ratatui::try_restore() {
            error!("Unable to restore terminal: {err:?}");
        }

        // Disable mouse capture as well.
        if let Err(err) = ratatui::crossterm::execute!(
            std::io::stdout(),
            ratatui::crossterm::event::DisableMouseCapture
        ) {
            error!("Unable to disable mouse capture: {err:?}");
        }

        let msg = format!("{}", panic_hook.panic_report(panic_info));

        log::error!("Error: {}", strip_ansi_escapes::strip_str(&msg));

        #[cfg(not(debug_assertions))]
        {
            eprintln!("{msg}");

            use human_panic::{Metadata, handle_dump, print_msg};

            let author = format!("authored by {}", env!("CARGO_PKG_AUTHORS"));
            let support = format!(
                "You can open a support request at {}",
                env!("CARGO_PKG_REPOSITORY")
            );

            let meta = Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                .authors(author)
                .support(support);

            let file_path = handle_dump(&meta, panic_info);

            print_msg(file_path, &meta)
                .expect("human-panic: printing error message to console failed");
        }

        #[cfg(debug_assertions)]
        {
            better_panic::Settings::auto()
                .most_recent_first(false)
                .lineno_suffix(true)
                .verbosity(better_panic::Verbosity::Full)
                .create_panic_handler()(panic_info);
        }
    }));

    Ok(())
}
