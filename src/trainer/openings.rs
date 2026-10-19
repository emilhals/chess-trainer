use std::{collections::BTreeMap, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::app::AppResult;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Instruction {
    pub learn: String,
    pub play: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Line {
    pub name: String,
    pub moves: Vec<String>,
    pub instructions: Vec<Instruction>,
}

#[allow(dead_code)]
impl Line {
    /// Returns amount of moves in line.
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    /// Returns true if the line is completed.
    pub fn is_completed(&self, move_index: usize) -> bool {
        move_index >= self.moves.len()
    }

    /// Returns a move in UCI-format.
    pub fn move_uci(&self, move_index: usize) -> &str {
        let found_move = self.moves.get(move_index);

        match found_move {
            Some(m) => return m,
            None => return "",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Opening {
    pub player: String,
    pub lines: Vec<Line>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Openings(BTreeMap<String, Opening>);

impl Openings {
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    pub fn bundled() -> serde_json::Result<Self> {
        Self::from_json(include_str!("../../data/openings.json"))
    }

    pub fn from_file(path: &str) -> AppResult<Openings> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn get_openings(&self) -> Vec<(String, Opening)> {
        let openings: Vec<(String, Opening)> = self.0.clone().into_iter().collect();
        openings
    }

    // Returns an Opening from opening name
    pub fn get_opening(&self, opening_name: &str) -> Option<&Opening> {
        self.0.get(opening_name)
    }

    // Returns a Line from opening name and line index
    pub fn get_line(&self, opening_name: &str, line_index: usize) -> Option<&Line> {
        let opening = self.get_opening(opening_name).unwrap();

        opening.lines.get(line_index)
    }

    pub fn get_instruction(
        &self,
        opening_name: &str,
        line_index: usize,
        instruction_index: usize,
    ) -> (String, String) {
        let Some(line) = self.get_line(opening_name, line_index) else {
            return ("".to_string(), "".to_string());
        };
        let Some(instruction) = line.instructions.get(instruction_index) else {
            return ("".to_string(), "".to_string());
        };

        (instruction.learn.clone(), instruction.play.clone())
    }

    pub fn get_move_uci(
        &self,
        opening_name: &str,
        line_index: usize,
        move_index: usize,
    ) -> Option<&String> {
        let line = self.get_line(opening_name, line_index);
        line.unwrap().moves.get(move_index)
    }

    pub fn format_name(name: &str) -> String {
        name.split("-")
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
}
