use crate::ui::pieces::PieceSize;
use shakmaty::Color;

#[allow(dead_code)]
pub struct King;

impl King {
    #[allow(dead_code)]
    pub fn to_string(size: PieceSize, color: Option<Color>) -> String {
        match size {
            PieceSize::Small => {
                // Use standard Unicode chess symbols for 1x1
                match color {
                    Some(Color::White) => "♔".to_string(),
                    Some(Color::Black) => "♚".to_string(),
                    None => " ".to_string(),
                }
            }
            PieceSize::Compact => {
                // Simple 2-line design for medium-sized cells
                "▗▂╋▂▖\n ▀█▀ \n ▀▀▀ ".to_string()
            }
            PieceSize::Extended => {
                // Extended 3-4 line design - more solid and consistent
                " ▂╋▂ \n▜███▛\n ▜█▛ \n▝▀▀▀▘".to_string()
            }
            PieceSize::Large => {
                // Current multi-line art
                r#"  ▂▃╋▃▂  
 ▐█████▋ 
  ▜███▛  
   ▟█▙   
  ▀▀▀▀▀  
"#
                .to_string()
            }
        }
    }
}
