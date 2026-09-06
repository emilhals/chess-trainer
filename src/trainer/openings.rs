use std::{collections::HashMap, fs::File, io::BufReader};

use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

use crate::app::AppResult;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Line {
    pub name: String,
    pub moves: Vec<String>,
    pub instructions: Vec<String>,
}

#[allow(dead_code)]
impl Line {
    /// Returns amount of moves in line.
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_completed(&self, move_index: usize) -> bool {
        move_index >= self.moves.len()
    }

    pub fn move_uci(&self, move_index: usize) -> &str {
        let found_move = self.moves.get(move_index);

        match found_move {
            Some(m) => return m,
            None => return "",
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Opening {
    pub lines: Vec<Line>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Openings(HashMap<String, Opening>);

impl Openings {
    pub fn from_file(path: &str) -> AppResult<Openings> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn get_opening(&self, opening_name: &str) -> Option<&Opening> {
        self.0.get(opening_name)
    }

    pub fn get_line(&self, opening_name: &str, line_index: usize) -> Option<&Line> {
        let opening = self.get_opening(opening_name).unwrap();

        opening.lines.get(line_index)
    }

    pub fn get_random_opening(&self) -> Option<(&String, &Opening)> {
        self.0.iter().choose(&mut rand::rng())
    }

    pub fn get_instruction(
        &self,
        opening_name: &str,
        line_index: usize,
        instruction_index: usize,
    ) -> String {
        let Some(line) = self.get_line(opening_name, line_index) else {
            return "".to_string();
        };
        let instruction = line.instructions.get(instruction_index);
        match instruction {
            Some(i) => return i.to_string(),
            None => return "".to_string(),
        }
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
