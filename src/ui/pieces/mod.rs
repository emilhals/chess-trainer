use shakmaty::{Color, Role};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceSize {
    /// Single character (1x1) - use standard Unicode chess symbols
    Small,
    /// Simple 2-line design - reliable fallback for medium-sized cells
    Compact,
    /// Extended 3-4 line design - intermediate between compact and large
    Extended,
    /// Large multi-line art (current default)
    Large,
}

impl PieceSize {
    pub fn from_dimensions(height: u16) -> Self {
        // If height is less than 3, use small (1x1)
        if height < 3 {
            return PieceSize::Small;
        }
        // If height is less than 4, use compact (simple 2-line)
        if height < 4 {
            return PieceSize::Compact;
        }
        // If height is less than 5, use extended (3-4 lines)
        if height < 5 {
            return PieceSize::Extended;
        }
        // If height is less than 7, use large multi-line art
        if height < 7 {
            return PieceSize::Large;
        }
        // Otherwise use large multi-line art
        PieceSize::Large
    }
}

/// Convert piece type to UTF-8 character
#[allow(dead_code)]
pub fn role_to_utf_enum(role: &Role, color: Option<Color>) -> &'static str {
    match color {
        Some(Color::White) => match role {
            Role::King => "♔",
            Role::Queen => "♕",
            Role::Rook => "♖",
            Role::Bishop => "♗",
            Role::Knight => "♘",
            Role::Pawn => "♙",
        },
        Some(Color::Black) => match role {
            Role::King => "♚",
            Role::Queen => "♛",
            Role::Rook => "♜",
            Role::Bishop => "♝",
            Role::Knight => "♞",
            Role::Pawn => "♟",
        },
        None => " ",
    }
}
