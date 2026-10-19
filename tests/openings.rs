#[cfg(test)]
mod tests {
    use shakmaty::{Chess, Position, uci::UciMove};

    use chess_trainer::trainer::openings::Openings;

    #[test]
    fn test_opening_lines_have_legal_moves() {
        let openings = Openings::from_file("data/openings.json").unwrap();

        for (opening_name, opening) in &openings.get_openings() {
            for line in &opening.lines {
                let mut position = Chess::default();

                for (index, move_str) in line.moves.iter().enumerate() {
                    let uci: UciMove = move_str.parse().unwrap_or_else(|_| {
                        panic!(
                            "Invalid UCI: {opening_name}/{} move {}: {move_str}",
                            line.name,
                            index + 1
                        )
                    });

                    let chess_move = uci.to_move(&position).unwrap_or_else(|_| {
                        panic!(
                            "Illegal move: {opening_name}/{} move {}: {move_str}",
                            line.name,
                            index + 1
                        )
                    });

                    position.play_unchecked(chess_move);
                }
            }
        }
    }
}
