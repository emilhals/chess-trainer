use shakmaty::Square;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    row: u8,
    col: u8,
}

impl Default for Coord {
    fn default() -> Self {
        Coord::new(4, 4)
    }
}

impl From<Square> for Coord {
    fn from(square: Square) -> Self {
        let index = square as u8;
        let rank = index / 8;
        let file = index % 8;
        Self {
            row: 7 - rank,
            col: file,
        }
    }
}

impl From<Coord> for Square {
    fn from(value: Coord) -> Self {
        let rank = 7 - value.row;
        let file = value.col;

        let index = (rank * 8) + file;

        Square::new(index.into())
    }
}

impl Coord {
    // This initialization makes the coord always compatible with shakmaty::Square
    pub fn new(row: u8, col: u8) -> Self {
        let row = row.min(7);
        let col = col.min(7);

        Self { row, col }
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn col(&self) -> u8 {
        self.col
    }

    pub fn move_to(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::Up => self.row = self.row.saturating_sub(1),
            MoveDirection::Down => self.row = (self.row + 1).min(7),
            MoveDirection::Left => self.col = self.col.saturating_sub(1),
            MoveDirection::Right => self.col = (self.col + 1).min(7),
        };
    }

    pub fn reverse(&self) -> Self {
        Self {
            row: 7 - self.row,
            col: 7 - self.col,
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.row.cmp(&other.row) {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            other => other,
        }
    }
}
