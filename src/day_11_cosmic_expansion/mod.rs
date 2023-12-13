use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
#[derive(Clone, PartialEq)]
pub enum Observation {
    EmptySpace,
    Galaxy,
}

impl Observation {
    fn from_char(c: char) -> Option<Observation> {
        match c {
            '#' => Some(Observation::Galaxy),
            '.' => Some(Observation::EmptySpace),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Observation::EmptySpace => '.',
            Observation::Galaxy => '#',
        }
    }
}

pub fn taxicab_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let x_distance = max(a.0, b.0) - min(a.0, b.0);
    let y_distance = max(a.1, b.1) - min(a.1, b.1);
    x_distance + y_distance
}
pub struct SpaceImage {
    pub grid: Vec<Vec<Observation>>,
}

impl FromStr for SpaceImage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| Observation::from_char(c))
                    .collect::<Option<Vec<Observation>>>()
            })
            .collect::<Option<Vec<Vec<Observation>>>>()
            .ok_or("Unable to parse rows and columns from the given string".to_string())?;
        Ok(SpaceImage { grid })
    }
}

impl Display for SpaceImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for obs in row {
                write!(f, "{}", obs.to_char())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
impl SpaceImage {
    pub fn age_by(self, age: usize) -> SpaceImage {
        // Expands the spaceimage
        let mut grid_with_repeated_rows: Vec<Vec<Observation>> = vec![];

        // Check all rows
        for row in &self.grid {
            if row.iter().all(|ob| ob == &Observation::EmptySpace) {
                for _ in 0..age {
                    grid_with_repeated_rows.push(row.clone());
                }
            }
            grid_with_repeated_rows.push(row.clone());
        }

        // Check if there are columns that need to be repeated
        let n_columns = &self.grid[0].len();
        let columns_to_repeat = (0..*n_columns)
            .filter(|col_n| {
                self.grid
                    .iter()
                    .all(|row| row[*col_n] == Observation::EmptySpace)
            })
            .collect::<HashSet<usize>>();

        let mut grid_with_repeated_cols: Vec<Vec<Observation>> = vec![];

        for row in grid_with_repeated_rows {
            let mut new_row: Vec<Observation> = vec![];
            for (i, obs) in row.iter().enumerate() {
                if columns_to_repeat.contains(&i) {
                    for _ in 0..age {
                        new_row.push(obs.clone());
                    }
                }
                new_row.push(obs.clone());
            }
            grid_with_repeated_cols.push(new_row);
        }
        SpaceImage {
            grid: grid_with_repeated_cols,
        }
    }

    pub fn find_empty_rows(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|ob| ob == &Observation::EmptySpace))
            .map(|(row_n, _)| row_n)
            .collect_vec()
    }

    pub fn find_empty_cols(&self) -> Vec<usize> {
        (0..self.grid[0].len())
            .filter(|col_n| {
                self.grid
                    .iter()
                    .all(|row| row[*col_n] == Observation::EmptySpace)
            })
            .collect_vec()
    }
    pub fn find_galaxies(&self) -> HashSet<(usize, usize)> {
        let mut locations: HashSet<(usize, usize)> = HashSet::new();

        for (row_n, row) in self.grid.iter().enumerate() {
            for (col_n, obs) in row.iter().enumerate() {
                if obs == &Observation::Galaxy {
                    locations.insert((row_n, col_n));
                }
            }
        }
        locations
    }

    pub fn find_smallest_distances(&self, grown_times: usize) -> Vec<usize> {
        let mut distances: Vec<usize> = vec![];
        let galaxies = self.find_galaxies();
        let empty_rows = self.find_empty_rows();
        let empty_cols = self.find_empty_cols();
        for combination in galaxies.iter().combinations(2) {
            let ((row_n, col_n), (row_m, col_m)) = (combination[0], combination[1]);
            // Find how many empty rows are between n and m
            let small_row = min(row_n, row_m);
            let large_row = max(row_n, row_m);

            let empty_rows_inbetween = empty_rows
                .iter()
                .filter(|row_n| large_row > row_n && row_n > &small_row)
                .count();

            let small_col = min(col_n, col_m);
            let large_col = max(col_n, col_m);

            let empty_cols_inbetween = empty_cols
                .iter()
                .filter(|col_n| large_col >= col_n && col_n >= &small_col)
                .count();

            let taxicab_distance = taxicab_distance((*row_n, *col_n), (*row_m, *col_m))
                + (empty_cols_inbetween + empty_rows_inbetween) * (grown_times - 1); // Taxicab distance already takes the row/cols into account once
            distances.push(taxicab_distance);
        }
        distances
    }
}
