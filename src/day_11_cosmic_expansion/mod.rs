use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ffi::c_void;
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
    pub fn expand(self) -> SpaceImage {
        // Expands the spaceimage
        let mut grid_with_repeated_rows: Vec<Vec<Observation>> = vec![];

        // Check all rows
        for row in &self.grid {
            if (row.iter().all(|ob| ob == &Observation::EmptySpace)) {
                grid_with_repeated_rows.push(row.clone());
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
                    new_row.push(obs.clone());
                }
                new_row.push(obs.clone());
            }
            grid_with_repeated_cols.push(new_row);
        }
        SpaceImage {
            grid: grid_with_repeated_cols,
        }
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

    pub fn find_smallest_distances(&self) -> Vec<usize> {
        let galaxies = self.find_galaxies();
        println!("{} Galaxies: {:?}", galaxies.len(), galaxies);
        let dist = galaxies
            .iter()
            .combinations(2)
            .map(|two_points| taxicab_distance(*two_points[0], *two_points[1]))
            .collect_vec();
        println!("The vector length is {}, and it has {:?}", dist.len(), dist);
        dist
    }
}
