use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::Paragraph,
};
use shakmaty::Role;

use crate::{trainer::ui::UI, ui::pieces::PieceSize};

impl UI {
    // Helper function to render a piece paragraph. Returns the paragraph and its line count.
    pub fn render_piece_paragraph(
        &self,
        piece_type: Option<Role>,
        piece_color: Option<shakmaty::Color>,
        square: Rect,
    ) -> (Paragraph<'static>, u16) {
        use crate::ui::pieces::{
            bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
        };

        let piece_size = PieceSize::from_dimensions(square.height);

        let piece_str = match piece_type {
            Some(Role::King) => King::to_string(piece_size, piece_color),
            Some(Role::Queen) => Queen::to_string(piece_size, piece_color),
            Some(Role::Rook) => Rook::to_string(piece_size, piece_color),
            Some(Role::Bishop) => Bishop::to_string(piece_size, piece_color),
            Some(Role::Knight) => Knight::to_string(piece_size, piece_color),
            Some(Role::Pawn) => Pawn::to_string(piece_size, piece_color),
            None => " ".to_string(),
        };

        let piece_color = match piece_color {
            Some(shakmaty::Color::White) => Color::White,
            Some(shakmaty::Color::Black) => Color::Black,
            None => Color::DarkGray,
        };

        let line_count = piece_str.lines().count().max(1) as u16;

        (
            Paragraph::new(piece_str)
                .fg(piece_color)
                .alignment(Alignment::Center),
            line_count,
        )
    }

    // Helper function for creating a centered rect. Returns the centered rect.
    pub fn piece_centered_rect(cell: Rect, content_lines: u16) -> Rect {
        let content_height = content_lines.min(cell.height);
        let top_offset = cell.height.saturating_sub(content_height) / 2;

        Rect {
            x: cell.x,
            y: cell.y + top_offset,
            width: cell.width,
            height: content_height,
        }
    }
}
