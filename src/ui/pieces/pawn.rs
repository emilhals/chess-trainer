use crate::ui::pieces::PieceSize;
use shakmaty::Color;

#[allow(dead_code)]
pub struct Pawn;

impl Pawn {
    #[allow(dead_code)]
    pub fn to_string(size: PieceSize, color: Option<Color>) -> String {
        match size {
            PieceSize::Small => match color {
                Some(Color::White) => "♙".to_string(),
                Some(Color::Black) => "♟".to_string(),
                None => " ".to_string(),
            },
            PieceSize::Compact => "  ▂  \n ▆█▆ \n ▔▔▔ ".to_string(),
            PieceSize::Extended => "     \n ▝█▘ \n ▟█▙ \n ▔▔▔ ".to_string(),
            PieceSize::Large => r#"
 ▄▇▄
 ▜█▛
▄███▄
▔▔▔▔▔
"#
            .to_string(),
        }
    }
}
