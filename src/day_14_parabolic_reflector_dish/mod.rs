use crate::day_14_parabolic_reflector_dish::Observation::{CubeRock, Empty, RoundRock};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Clone, PartialEq)]
enum Observation {
    RoundRock,
    CubeRock,
    Empty,
}

impl Observation {
    fn from_char(c: char) -> Result<Observation, &'static str> {
        match c {
            'O' => Ok(RoundRock),
            '#' => Ok(CubeRock),
            '.' => Ok(Empty),
            _ => Err("Unable to parse!"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            RoundRock => 'O',
            CubeRock => '#',
            Empty => '.',
        }
    }
}
pub struct Platform {
    elements: Vec<Vec<Observation>>,
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.elements {
            for element in row {
                f.write_char(element.to_char())?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')?;
        Ok(())
    }
}
impl Platform {
    pub fn tilted(self) -> Platform {
        let mut elements = self.elements;
        let n_cols = elements[0].len();
        for _ in (1..elements.len()).rev() {
            for row_n in (1..elements.len()).rev() {
                // Swap elements
                for col_n in 0..n_cols {
                    if elements[row_n][col_n] == RoundRock && elements[row_n - 1][col_n] == Empty {
                        elements[row_n - 1][col_n] = RoundRock;
                        elements[row_n][col_n] = Empty;
                    }
                }
            }
        }
        Platform { elements }
    }

    pub fn total_load(&self) -> usize {
        // Get load for every row, by counting the amount of rocks and multiplying by the weight
        let n_rows = self.elements.len();
        let weights = (1..=n_rows).rev();
        self.elements
            .iter()
            .zip(weights)
            .map(|(row, weight)| row.iter().filter(|&o| o == &RoundRock).count() * weight)
            .sum()
    }
}
impl FromStr for Platform {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(Observation::from_char)
                    .collect::<Result<Vec<Observation>, &'static str>>()
            })
            .collect::<Result<Vec<Vec<Observation>>, &'static str>>()?;
        Ok(Platform { elements })
    }
}
