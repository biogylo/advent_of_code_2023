use crate::day_2_cube_conundrum;
use crate::day_2_cube_conundrum::{Conundrum, CubeColor, CubeCounts, Game};
use std::string::String;

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;

impl FromIterator<(CubeColor, usize)> for CubeCounts {
    fn from_iter<T: IntoIterator<Item = (CubeColor, usize)>>(iter: T) -> Self {
        let the_hashmap: HashMap<CubeColor, usize> = CubeColor::iter()
            .map(|variant| (variant, 0)) // Set default values to 0
            .chain(iter) // Combine with the given
            .collect();
        return CubeCounts { 0: the_hashmap };
    }
}

impl<const N: usize> From<[(CubeColor, usize); N]> for CubeCounts {
    fn from(cube_color_usize_array: [(CubeColor, usize); N]) -> Self {
        cube_color_usize_array.into_iter().collect()
    }
}

impl fmt::Display for CubeColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CubeColor::Red => write!(f, "Red"),
            CubeColor::Green => write!(f, "Green"),
            CubeColor::Blue => write!(f, "Blue"),
        }
    }
}

impl FromStr for CubeColor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR_STR: &str = "Unable to obtain a color from the given string";
        let clean_input = s.trim().to_lowercase();
        let mapping: HashMap<String, Self> = Self::iter()
            .map(|variant| (variant.to_string().to_lowercase(), variant))
            .collect();
        mapping.get(clean_input.as_str()).cloned().ok_or(ERROR_STR)
    }
}

impl CubeCounts {
    pub fn get(&self, color: &CubeColor) -> usize {
        self.0.get(&color).unwrap().clone()
    }
    /*
        Whether the given cube count fits the other one
    */
    pub fn fits(&self, bag_other: &Self) -> bool {
        CubeColor::iter().all(|color| self.0[&color] <= bag_other.0[&color])
    }
    /*
        Takes in a line like " 3 blue, 4 red",
        and returns a Turn with the correct fields
    */
    pub fn from_turn_str(turn_string: &str) -> Result<CubeCounts, String> {
        let turn_vector: Vec<(CubeColor, usize)> = turn_string
            .trim()
            .split(',')
            .map(day_2_cube_conundrum::get_elements_from_turn_token)
            .collect::<Option<Vec<(CubeColor, usize)>>>()
            .ok_or(String::from("Unable to parse turn string"))?;
        return Ok(CubeCounts::from_iter(turn_vector.into_iter()));
    }

    pub fn power(&self) -> usize {
        CubeColor::iter().map(|color| self.get(&color)).product()
    }
}

impl FromStr for Conundrum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            0: s.trim()
                .split("\n")
                .map(Game::from_str)
                .collect::<Result<Vec<Game>, _>>()?,
        })
    }
}

impl Conundrum {
    pub fn possible_games(&self, bag: &CubeCounts) -> Vec<usize> {
        self.0
            .iter()
            .filter(|game| game.is_possible(bag))
            .map(|game| game.id)
            .collect()
    }

    pub fn sum_of_possible_game_ids(&self, bag: &CubeCounts) -> usize {
        self.possible_games(bag).iter().sum()
    }

    pub fn smallest_bags_needed(&self) -> Vec<CubeCounts> {
        self.0.iter().map(Game::smallest_bag).collect()
    }

    pub fn powers(&self) -> Vec<usize> {
        self.smallest_bags_needed()
            .iter()
            .map(CubeCounts::power)
            .collect()
    }
}

impl Game {
    /*
        Tells you whether the game can be done with the
        given bag of cube counts.
    */
    pub fn is_possible(&self, bag: &CubeCounts) -> bool {
        self.turns.iter().all(|turn| turn.fits(&bag))
    }

    pub fn smallest_bag(&self) -> CubeCounts {
        CubeColor::iter()
            .map(|color| (color.clone(), self.highest_count(&color)))
            .collect()
    }

    /*
        Returns the highest count in a draw of a given color variant in the game
    */
    fn highest_count(&self, color: &CubeColor) -> usize {
        self.turns
            .iter()
            .map(|turn| turn.get(&color))
            .max()
            .unwrap()
    }
}

impl FromStr for Game {
    type Err = String;
    fn from_str(game_string: &str) -> Result<Self, String> {
        let right_string = game_string.trim().replace("Game", "");
        let (game_id_token, turn_tokens) = right_string
            .trim()
            .split_once(':')
            .ok_or("Unable to parse game str due to missing colon".to_string())?;
        let game_id: usize = game_id_token
            .parse()
            .or(Err("Unable to parse game id".to_string()))?;
        let turn_cube_counts: Vec<CubeCounts> = turn_tokens
            .split(";")
            .map(|turn_token| CubeCounts::from_turn_str(turn_token))
            .collect::<Result<Vec<CubeCounts>, String>>()?;
        Ok(Self {
            id: game_id,
            turns: turn_cube_counts,
        })
    }
}
