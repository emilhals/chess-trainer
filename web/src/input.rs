use chess_trainer::input::Key;
use ratzilla::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub struct Unsupported;

pub fn key_from_ratzilla_event(event: KeyEvent) -> Option<Key> {
    match event.code {
        KeyCode::Up => Some(Key::Up),
        KeyCode::Down => Some(Key::Down),
        KeyCode::Left => Some(Key::Left),
        KeyCode::Right => Some(Key::Right),
        KeyCode::Enter => Some(Key::Enter),
        KeyCode::Esc => Some(Key::Esc),
        KeyCode::Backspace => Some(Key::Backspace),
        KeyCode::Char(c) => Some(Key::Char(c)),
        _ => None,
    }
}
