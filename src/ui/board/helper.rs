use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};
use shakmaty::{Role, Square};

use crate::{
    app::coord::Coord,
    state::trainer_state::TrainerMode,
    trainer::{Trainer, board::Board},
    ui::board::pieces::PieceSize,
};

// Helper function to render a piece paragraph. Returns the paragraph and its line count.
pub fn render_piece_paragraph(
    piece_type: Option<Role>,
    piece_color: Option<shakmaty::Color>,
    square: Rect,
) -> (Paragraph<'static>, u16) {
    use crate::ui::board::pieces::{
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

pub fn instruction_height(width: u16, trainer: &Trainer) -> usize {
    let inner_width = width.saturating_sub(2);

    if trainer.is_completed() {
        return 5;
    }

    let text = if trainer.state.validate_move_response.is_some() {
        let response = trainer
            .state
            .validate_move_response
            .clone()
            .unwrap_or_else(|| String::from(""));

        vec![Line::from(response)]
    } else {
        if trainer.state.mode.selected_mode == TrainerMode::Drill && !trainer.state.made_wrong_move
        {
            return 4;
        }

        let (info, play) = trainer.current_instruction();

        vec![
            Line::from(Span::styled(info, Style::new())),
            Line::from(Span::styled(play, Style::new().bold())),
        ]
    };

    let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });

    paragraph.line_count(inner_width) + 3
}

pub fn legal_moves(board: &Board, actual_square: Option<Square>) -> Vec<Coord> {
    let Some(square) = actual_square else {
        return vec![];
    };

    let selected_piece_color = board.get_piece_color_at_square(&square);

    if let Some(color) = selected_piece_color
        && color == board.player_turn
    {
        let mut legal_moves: Vec<Coord> = board
            .get_legal_moves(&square)
            .iter()
            .map(|&s| Coord::from(s))
            .collect();

        if board.is_flipped {
            legal_moves = legal_moves.iter().map(Coord::reverse).collect();
        }
        return legal_moves;
    }
    vec![]
}
