use std::str::FromStr;

use shakmaty::{Square, uci::UciMove};

use crate::app::coord::Coord;

/// Converts a [`Coord`] to a [`Square`], mirroring it when the board is flipped.
#[must_use]
pub fn get_square_from_coord(coord: Coord, is_flipped: bool) -> Square {
    if is_flipped {
        coord.reverse().into()
    } else {
        coord.into()
    }
}

/// Converts a [`Square`] to a [`Coord`], mirroring it when the board is flipped.
#[must_use]
pub fn get_coord_from_square(square: Square, is_flipped: bool) -> Coord {
    if is_flipped {
        Coord::from(square).reverse()
    } else {
        Coord::from(square)
    }
}

/// Returns the mirror square when the board is flipped, otherwise the original square.
#[must_use]
pub fn flip_square_if_needed(square: Square, is_flipped: bool) -> Square {
    if is_flipped {
        Coord::from(square).reverse().into()
    } else {
        square
    }
}
