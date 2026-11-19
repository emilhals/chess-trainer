use ratatui::crossterm::event::{
    KeyCode as CtKeyCode, KeyEvent as CtKeyEvent, KeyEventKind, MouseButton as CtButton,
    MouseEvent as CtMouseEvent, MouseEventKind as CtKind,
};

use crate::input::{Key, MouseButton, MouseInput, MouseInputKind};

/// The terminal event has no equivalent in the app's input types.
#[derive(Debug)]
pub struct Unsupported;

impl TryFrom<CtKeyEvent> for Key {
    type Error = Unsupported;

    fn try_from(event: CtKeyEvent) -> Result<Self, Self::Error> {
        if event.kind != KeyEventKind::Press {
            return Err(Unsupported);
        }

        match event.code {
            CtKeyCode::Up => Ok(Key::Up),
            CtKeyCode::Down => Ok(Key::Down),
            CtKeyCode::Left => Ok(Key::Left),
            CtKeyCode::Right => Ok(Key::Right),
            CtKeyCode::Enter => Ok(Key::Enter),
            CtKeyCode::Esc => Ok(Key::Esc),
            CtKeyCode::Backspace => Ok(Key::Backspace),
            CtKeyCode::Char(c) => Ok(Key::Char(c)),
            _ => Err(Unsupported),
        }
    }
}

impl TryFrom<CtButton> for MouseButton {
    type Error = Unsupported;

    fn try_from(button: CtButton) -> Result<Self, Self::Error> {
        match button {
            CtButton::Left => Ok(MouseButton::Left),
            CtButton::Right => Ok(MouseButton::Right),
            CtButton::Middle => Err(Unsupported),
        }
    }
}

impl TryFrom<CtMouseEvent> for MouseInput {
    type Error = Unsupported;

    fn try_from(event: CtMouseEvent) -> Result<Self, Self::Error> {
        let kind = match event.kind {
            CtKind::Down(b) => MouseInputKind::Press(b.try_into()?),
            CtKind::Up(b) => MouseInputKind::Release(b.try_into()?),
            CtKind::Drag(b) => MouseInputKind::Drag(b.try_into()?),
            CtKind::Moved => MouseInputKind::Move,
            CtKind::ScrollUp => MouseInputKind::ScrollUp,
            CtKind::ScrollDown => MouseInputKind::ScrollDown,
            _ => return Err(Unsupported),
        };

        Ok(MouseInput {
            kind,
            column: event.column,
            row: event.row,
        })
    }
}
