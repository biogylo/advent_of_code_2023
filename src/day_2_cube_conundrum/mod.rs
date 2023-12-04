use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use strum_macros::EnumIter;

mod r#impl;

#[derive(Debug, Eq, PartialEq, Hash, EnumIter, Clone)]
pub enum CubeColor {
    Red,
    Blue,
    Green,
}
#[derive(Eq, Debug, PartialEq)]
pub struct CubeCounts(HashMap<CubeColor, usize>);

fn get_elements_from_turn_token(token: &str) -> Option<(CubeColor, usize)> {
    let (number_token, color_token) = token.trim().split_once(' ')?;
    let color = CubeColor::from_str(color_token).ok()?;
    let count = usize::from_str(number_token).ok()?;
    Some((color, count))
}

#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    pub id: usize,
    pub turns: Vec<CubeCounts>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Conundrum(pub Vec<Game>);
