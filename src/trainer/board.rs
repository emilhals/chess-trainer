use shakmaty::{Chess, Color, Move, Piece, Position, Role, Square, san::San};

pub struct Board {
    // History of player moves
    pub move_history: Vec<Move>,
    /// Historic of the past gameboards states.
    /// The last position is the current position.
    pub position_history: Vec<Chess>,
    // Stores captured pieces
    pub taken_pieces: Vec<Piece>,
    // Track if the board is currently flipped
    pub is_flipped: bool,
    /// Current position index in history when navigating. None means viewing the latest position.
    pub history_position_index: Option<usize>,
    pub player_turn: Color,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            move_history: Vec::new(),
            position_history: vec![Chess::default()],
            taken_pieces: Vec::new(),
            is_flipped: false,
            history_position_index: None,
            player_turn: Color::White,
        }
    }
}

#[allow(dead_code)]
impl Board {
    pub fn reset(&mut self) {
        self.position_history = vec![Chess::default()];
        self.move_history = Vec::new();
        self.is_flipped = false;
        self.history_position_index = None;
    }

    pub fn position_ref(&self) -> &Chess {
        self.current_position().unwrap_or_else(|| {
            panic!("Position history is empty: board was not initialized correctly")
        })
    }

    pub fn execute_move(&mut self, from: Square, to: Square) -> Option<Move> {
        let chess = self.position_ref().clone();

        // Find matching legal move
        let legal_moves = chess.legal_moves();

        let matching_move = legal_moves
            .iter()
            .find(|m| m.from() == Some(from) && m.to() == to);

        if let Some(shakmaty_move) = matching_move {
            match *shakmaty_move {
                Move::Normal { .. } => {
                    if let Some(captured_piece) = chess.board().piece_at(shakmaty_move.to()) {
                        self.taken_pieces.push(captured_piece);
                    }
                }
                Move::EnPassant { .. } => {}
                Move::Castle { .. } => {}
                Move::Put { .. } => {}
            }
            // Execute move
            match chess.play(*shakmaty_move) {
                Ok(new_chess) => {
                    self.position_history.push(new_chess);
                    self.move_history.push(*shakmaty_move);
                    self.history_position_index = None;
                    Some(shakmaty_move.clone())
                }

                Err(e) => {
                    // fix logg
                    None
                }
            }
        } else {
            // move not found
            None
        }
    }

    /// Gets a read-only reference to the current position, or None if history is empty
    /// If navigating history, returns the position at history_position_index.
    pub fn current_position(&self) -> Option<&Chess> {
        if let Some(index) = self.history_position_index {
            self.position_history.get(index)
        } else {
            self.position_history.last()
        }
    }

    /// Get piece type at a coordinate (handles flipped board)
    pub fn get_role_at_square(&self, square: &Square) -> Option<Role> {
        self.position_ref()
            .board()
            .piece_at(*square)
            .map(|p| p.role)
    }

    pub fn get_square_from_uci(&self, uci: &str) {}

    /// Get the color of a piece on a given square
    pub fn get_piece_color_at_square(&self, square: &Square) -> Option<Color> {
        let piece = self.position_ref().board().piece_at(*square);
        piece.map(|p| p.color)
    }

    pub fn get_legal_moves(&self, player_turn: Color, square: &Square) -> Vec<Square> {
        self.position_ref()
            .clone()
            .legal_moves()
            .iter()
            .filter(|m| m.from() == Some(*square))
            .map(|m| m.to())
            .collect()
    }

    pub fn move_to_san(&self, move_index: usize) -> String {
        if move_index >= self.move_history.len() || move_index >= self.position_history.len() {
            return String::new();
        }

        // Get the position before this move was made
        let position = &self.position_history[move_index];
        let chess_move = &self.move_history[move_index];

        // Convert to SAN using shakmaty
        let san = San::from_move(position, *chess_move);
        san.to_string()
    }
}
