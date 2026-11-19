#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Tab,
    Esc,
    Backspace,
    Char(char),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseInputKind {
    Press(MouseButton),
    Release(MouseButton),
    Drag(MouseButton),
    Move,
    ScrollUp,
    ScrollDown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MouseInput {
    pub kind: MouseInputKind,
    pub column: u16,
    pub row: u16,
}
