use std::str::FromStr;

pub enum CubeColor {
    Red,
    Blue,
    Green,
}

#[derive(PartialEq, Debug)]
pub struct Turn {
    pub red_balls_shown: usize,
    pub green_balls_shown: usize,
    pub blue_balls_shown: usize,
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
impl FromStr for Turn {
    type Err = ();

    /*
        Takes in a line like " 3 blue, 4 red",
        and returns a Turn with the correct fields
    */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut the_turn = Self {
            red_balls_shown: 0,
            green_balls_shown: 0,
            blue_balls_shown: 0,
        };
        let elements = s.trim().split(',').map(get_elements_from_turn_token);
        for element in elements {
            match element {
                Some((CubeColor::Red, count)) => the_turn.red_balls_shown = count,
                Some((CubeColor::Green, count)) => the_turn.green_balls_shown = count,
                Some((CubeColor::Blue, count)) => the_turn.blue_balls_shown = count,
                None => return Err(()),
            }
        }
        return Ok(the_turn);
    }
}

pub struct GameRecord {
    all_turns: Vec<Turn>,
}
pub struct Bag {
    red_cubes: usize,
    blue_cubes: usize,
    green_cubes: usize,
}
