use std::io;

use ratzilla::ratatui::Terminal;

use ratzilla::{WebGl2Backend, WebRenderer};

pub mod input;

use std::{cell::RefCell, rc::Rc};

use chess_trainer::{
    app::App,
    handlers::handler::handle_key_events,
    skin::Skins,
    state::click_regions::ClickRegions,
    storage::{MemoryStorage, Storage},
    trainer::{Trainer, openings::Openings},
    ui::main_ui,
};
use ratzilla::{FontAtlasConfig, backend::webgl2::WebGl2BackendOptions};

use crate::input::key_from_ratzilla_event;

fn main() -> io::Result<()> {
    let options = WebGl2BackendOptions::new()
        .font_atlas_config(FontAtlasConfig::dynamic(
            &["JetBrains Mono", "Noto Color Emoji", "monospace"],
            16.0,
        ))
        .fallback_glyph("?");

    let backend = WebGl2Backend::new_with_options(options)?;
    let mut terminal = Terminal::new(backend)?;

    let openings: Openings = Openings::bundled()?;
    let skins: Skins = Skins::bundled()?;

    let storage: Box<dyn Storage> = Box::new(MemoryStorage::default());

    let trainer = Trainer::new(openings);

    let app = Rc::new(RefCell::new(App::new(trainer, skins, storage)));

    terminal.on_key_event({
        let app_cloned = app.clone();
        move |key_event| {
            if let Some(key) = key_from_ratzilla_event(key_event) {
                let mut app = app_cloned.borrow_mut();

                if let Err(err) = handle_key_events(key, &mut app) {
                    log::warn!("{err}");
                }
            };
        }
    })?;

    terminal.draw_web(move |f| {
        app.borrow_mut().tick();

        let mut regions = ClickRegions::default();
        main_ui::render(&mut app.borrow_mut(), f, &mut regions);
    });

    Ok(())
}
