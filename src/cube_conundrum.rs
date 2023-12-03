use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug, Eq, PartialEq, Hash, EnumIter, Clone)]
pub enum CubeColor {
    Red,
    Blue,
    Green,
}
#[derive(Eq, Debug, PartialEq)]
pub struct CubeCounts(HashMap<CubeColor, usize>);

impl FromIterator<(CubeColor, usize)> for CubeCounts {
    fn from_iter<T: IntoIterator<Item = (CubeColor, usize)>>(iter: T) -> Self {
        let mut the_hashmap = HashMap::from_iter(iter);
        for variant in CubeColor::iter() {
            if !the_hashmap.contains_key(&variant) {
                the_hashmap.insert(variant, 0);
            }
        }
        return CubeCounts { 0: the_hashmap };
    }
}

impl<const N: usize> From<[(CubeColor, usize); N]> for CubeCounts {
    fn from(cubecolor_usize_array: [(CubeColor, usize); N]) -> Self {
        cubecolor_usize_array.into_iter().collect()
    }
}
impl FromStr for CubeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(()),
        }
    }
}

fn get_elements_from_turn_token(token: &str) -> Option<(CubeColor, usize)> {
    let (number_token, color_token) = token.trim().split_once(' ')?;
    let color = CubeColor::from_str(color_token).ok()?;
    let count = usize::from_str(number_token).ok()?;
    Some((color, count))
}

impl CubeCounts {
    pub fn get(&self, color: &CubeColor) -> usize {
        self.0[&color]
    }
    /*
        Whether the given cube count fits the other one
    */
    pub fn fits(&self, bag_other: &Self) -> bool {
        for color in CubeColor::iter() {
            if self.0[&color] > bag_other.0[&color] {
                return false;
            }
        }
        return true;
    }
    /*
        Takes in a line like " 3 blue, 4 red",
        and returns a Turn with the correct fields
    */
    pub fn from_turn_str(turn_string: &str) -> Result<CubeCounts, String> {
        let turn_vector: Vec<(CubeColor, usize)> = turn_string
            .trim()
            .split(',')
            .map(get_elements_from_turn_token)
            .collect::<Option<Vec<(CubeColor, usize)>>>()
            .ok_or(String::from("Unable to parse turn string"))?;
        return Ok(CubeCounts::from_iter(turn_vector.into_iter()));
    }

    pub fn power(&self) -> usize {
        CubeColor::iter().map(|color| self.get(&color)).product()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    pub id: usize,
    pub turns: Vec<CubeCounts>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Conundrum(pub Vec<Game>);
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
            .map(|color| {
                (
                    color.clone(),
                    self.turns
                        .iter()
                        .map(|turn| turn.get(&color))
                        .max()
                        .unwrap(),
                )
            })
            .collect()
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
        let turn_cube_counts: Result<Vec<CubeCounts>, String> = turn_tokens
            .split(";")
            .map(|turn_token| CubeCounts::from_turn_str(turn_token))
            .collect();
        Ok(Self {
            id: game_id,
            turns: turn_cube_counts?,
        })
    }
}
