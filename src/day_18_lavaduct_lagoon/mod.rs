use crate::day_18_lavaduct_lagoon::Direction::{East, North, South, West};
use itertools::Itertools;
use std::cmp::{max, min};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(PartialEq, Clone)]
enum Block {
    Air,
    Ground,
}
impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Block::Air => f.write_char('#'),
            Block::Ground => f.write_char('.'),
        }
    }
}
pub struct Terrain {
    grid: Vec<Vec<Block>>,
}

impl Display for Terrain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for block in row {
                write!(f, "{}", block)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Terrain {
    pub fn is_inside_tench(&self, row_n: usize, col_n: usize) -> bool {
        if self.grid[row_n][col_n] == Block::Air {
            return true;
        }

        // Last element in row, its not in trench
        if col_n + 1 == self.grid[0].len() {
            return false;
        }

        let left_over_row = &self.grid[row_n][col_n + 1..];
        // Gotta dedupe consecutive edges
        let mut left_over_row_no_consecutives = vec![];
        let mut last_was_air = false;
        for block in left_over_row {
            if last_was_air && block == &Block::Air {
                // Repeated air, no need to continue
            } else {
                left_over_row_no_consecutives.push(block.clone());
            }
            last_was_air = block == &Block::Air;
        }
        let odd_edges_seen = self.grid[row_n][col_n + 1..]
            .iter()
            .filter(|&block| block == &Block::Air)
            .count()
            % 2
            == 1;
        // By the even-odd rule, we are inside if we saw an odd amount of non_consecutive_edges
        odd_edges_seen
    }
    pub fn culled(self) -> Terrain {
        // For each point in the terrain, check if its inside or outside, if inside paint
        let height = self.grid.len();
        let width = self.grid[0].len();

        let mut new_terrain = self;

        let coordinates_inside_trench: Vec<(usize, usize)> = (0..height)
            .cartesian_product(0..width)
            .filter(|(row_n, col_n)| new_terrain.is_inside_tench(*row_n, *col_n))
            .collect_vec();
        coordinates_inside_trench
            .iter()
            .for_each(|(row_n, col_n)| new_terrain.grid[*row_n][*col_n] = Block::Air);
        new_terrain
    }

    pub fn trench_area(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter().filter(|&block| block == &Block::Air))
            .count()
    }

    pub fn empty(n_rows: usize, n_cols: usize) -> Terrain {
        let grid = vec![vec![Block::Ground; n_cols]; n_rows];
        Terrain { grid }
    }

    pub fn dig(&mut self, row_n: usize, col_n: usize) {
        self.grid[row_n][col_n] = Block::Air;
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().len() != 1 {
            return Err("Unable to parse more than one character for direction");
        }
        match &s.chars().next().unwrap().to_uppercase().next().unwrap() {
            'U' => Ok(North),
            'D' => Ok(South),
            'L' => Ok(West),
            'R' => Ok(East),
            _ => Err("Unable to parse direction from character!"),
        }
    }
}
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RgbColor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Takes in a string that looks like FFFFFF
        if s.len() != 6 && !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Must be a 6 character hex string");
        }
        // There has to be three chunks
        let unable_to_parse_error = |_| "Unable to parse hex";
        let r: u8 = u8::from_str_radix(&s[0..=1], 16).map_err(unable_to_parse_error)?;
        let g: u8 = u8::from_str_radix(&s[2..=3], 16).map_err(unable_to_parse_error)?;
        let b: u8 = u8::from_str_radix(&s[4..=5], 16).map_err(unable_to_parse_error)?;
        Ok(Self { r, g, b })
    }
}
struct Step {
    direction: Direction,
    count: usize,
    color: RgbColor,
}

impl Step {
    pub fn apply(&self, row_n: isize, col_n: isize, only_vertices: bool) -> Vec<(isize, isize)> {
        let coords_to_eval = if only_vertices {
            (self.count..self.count + 1)
        } else {
            (1..self.count + 1)
        };

        coords_to_eval
            .map(|offset| match &self.direction {
                South => (row_n + offset as isize, col_n),
                North => (row_n - offset as isize, col_n),
                East => (row_n, col_n + offset as isize),
                West => (row_n, col_n - offset as isize),
            })
            .collect_vec()
    }
}

impl FromStr for Step {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Takes a row in the form R 6 (#70c710), and parses the tokens
        let tokens_error = || "Unable to get exactly three tokens from step!";
        let tokened_string = s.trim().replace("(#", "").replace(")", "");
        let (direction_token, count_token, color_token): (&str, &str, &str) = tokened_string
            .split_whitespace()
            .collect_tuple()
            .ok_or_else(tokens_error)?;

        let direction = direction_token.parse()?;
        let count = count_token
            .parse()
            .map_err(|_| "Unable to parse step count")?;
        let color = color_token.parse()?;
        Ok(Self {
            direction,
            count,
            color,
        })
    }
}
pub struct DigPlan {
    steps: Vec<Step>,
}

impl FromStr for DigPlan {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s;
        let steps = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Step>, &'static str>>()?;
        Ok(Self { steps })
    }
}

impl DigPlan {
    pub fn apply(self, only_vertices: bool) -> Terrain {
        // Will go through a dig plan creating coordinates for the trench
        let mut min_row = 0;
        let mut min_column = 0;
        let mut max_row = 0;
        let mut max_column = 0;
        let mut current_row = 0;
        let mut current_column = 0;
        let mut coordinates = vec![];

        for step in self.steps {
            for new_cord in step.apply(current_row, current_column, only_vertices) {
                (current_row, current_column) = new_cord;
                coordinates.push((current_row, current_column));
                min_row = min(current_row, min_row);
                min_column = min(current_column, min_column);
                max_row = max(current_row, max_row);
                max_column = max(current_column, max_column);
            }
        }

        let mut normalized_coordinates = vec![];
        for (irow, icol) in coordinates {
            normalized_coordinates.push(((irow - min_row) as usize, (icol - min_column) as usize));
        }

        let num_columns = (max_column - min_column + 1) as usize;
        let num_rows = (max_row - min_row + 1) as usize;

        let mut the_terrain = Terrain::empty(num_rows, num_columns);
        for (row_n, col_n) in normalized_coordinates {
            the_terrain.dig(row_n, col_n);
        }
        the_terrain
    }
}
