use std::collections::BTreeMap;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Skin {
    pub light_square: Color,
    pub dark_square: Color,
    pub selected_square: Color,
    pub legal_move: Color,
    pub last_move: Color,
    pub cursor: Color,
}

impl Default for Skin {
    fn default() -> Self {
        Self {
            light_square: Color::Rgb(238, 238, 210),
            dark_square: Color::Rgb(118, 150, 86),
            selected_square: Color::Rgb(246, 246, 105),
            legal_move: Color::Yellow,
            last_move: Color::Yellow,
            cursor: Color::Cyan,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Skins {
    pub skins: BTreeMap<String, Skin>,
}

impl Skins {
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    pub fn bundled() -> serde_json::Result<Self> {
        Self::from_json(include_str!("../data/skins.json"))
    }

    pub fn display_name(name: &str) -> String {
        name.split('-')
            .map(|word| {
                let mut chars = word.chars();

                match chars.next() {
                    Some(first) => first.to_uppercase().to_string() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn get(&self, name: &str) -> Option<&Skin> {
        self.skins.get(name)
    }

    pub fn names(&self) -> Vec<&str> {
        self.skins.keys().map(String::as_str).collect()
    }
}
