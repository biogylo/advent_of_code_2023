use crate::day_10_pipe_maze::Pipe::{NorthEast, NorthWest, SouthEast, SouthWest};
use itertools::Itertools;
use std::fmt::{Display, Formatter, Write};
use std::ops::Index;
use std::str::FromStr;

// const SYMBOLS: [char; 6] = ['═', '║', '╔', '╗', '╚', '╝'];
pub fn ugly_pipe_maze_to_cute_pipe_maze(s: &str) -> String {
    s.trim()
        .replace("-", "═")
        .replace("|", "║")
        .replace("F", "╔")
        .replace("7", "╗")
        .replace("L", "╚")
        .replace("J", "╝")
        .to_string()
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug, Copy)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn iter() -> impl IntoIterator<Item = Dir> {
        [Dir::North, Dir::East, Dir::West, Dir::South].into_iter()
    }
    fn complement(&self) -> Dir {
        match &self {
            Dir::East => Dir::West,
            Dir::West => Dir::East,
            Dir::North => Dir::South,
            Dir::South => Dir::North,
        }
    }
    fn to_coordinate_offset(&self) -> (isize, isize) {
        match self {
            Dir::East => (0, 1), // Move 0 rows down, 1 column to the right
            Dir::West => (0, -1),
            Dir::North => (-1, 0), // Move 1 row up, 0 columns
            Dir::South => (1, 0),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Pipe {
    NorthSouth,
    EastWest,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
    Blocked,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match &self {
            Pipe::NorthSouth => '║',
            Pipe::EastWest => '═',
            Pipe::SouthEast => '╔',
            Pipe::SouthWest => '╗',
            Pipe::NorthEast => '╚',
            Pipe::NorthWest => '╝',
            Pipe::Blocked => '.',
        };
        write!(f, "{}", c)
    }
}
impl Pipe {
    pub fn from_char(c: char) -> Pipe {
        match c {
            '═' => Pipe::EastWest,
            '║' => Pipe::NorthSouth,
            '╔' => Pipe::SouthEast,
            '╗' => Pipe::SouthWest,
            '╚' => Pipe::NorthEast,
            '╝' => Pipe::NorthWest,
            _ => Pipe::Blocked,
        }
    }

    pub fn to_dirs(&self) -> Option<(Dir, Dir)> {
        match self {
            Pipe::NorthSouth => Some((Dir::North, Dir::South)),
            Pipe::EastWest => Some((Dir::East, Dir::West)),
            Pipe::SouthEast => Some((Dir::South, Dir::East)),
            Pipe::SouthWest => Some((Dir::South, Dir::West)),
            Pipe::NorthEast => Some((Dir::North, Dir::East)),
            Pipe::NorthWest => Some((Dir::North, Dir::West)),
            Pipe::Blocked => None,
        }
    }

    pub fn from_dirs(dirs: [Dir; 2]) -> Pipe {
        let sorted_dirs: (&Dir, &Dir) = dirs
            .iter()
            .sorted()
            .collect_tuple()
            .expect("Impossible for there to not be two of them, after sorting");

        match sorted_dirs {
            (Dir::North, Dir::South) => Pipe::NorthSouth,
            (Dir::East, Dir::West) => Pipe::EastWest,
            (Dir::South, Dir::East) => Pipe::SouthEast,
            (Dir::South, Dir::West) => Pipe::SouthWest,
            (Dir::North, Dir::East) => Pipe::NorthEast,
            (Dir::North, Dir::West) => Pipe::NorthWest,
            _ => panic!("Impossible to create pipe from dirs: ({:?})", dirs),
        }
    }
    pub fn next(&self, comes_from: &Dir) -> Option<Dir> {
        let directions = self.to_dirs()?;
        if comes_from == &directions.0 {
            Some(directions.1.clone())
        } else if comes_from == &directions.1 {
            Some(directions.0.clone())
        } else {
            None
        }
    }
}
pub struct PipeMaze {
    grid: Vec<Vec<Pipe>>,
    pub start: Option<(usize, usize)>,
    width: usize,
    height: usize,
}

impl PipeMaze {
    pub fn is_inside_loop(&self, row_n: usize, col_n: usize) -> bool {
        let pipes_in_loop: Vec<(usize, usize, &Pipe)> = self
            .get_all_points_in_loop()
            .unwrap()
            .into_iter()
            .filter(|[row_p, col_p]| row_p == &row_n && col_p < &col_n)
            .filter(|[row_p, col_p]| self.grid[*row_p][*col_p] != Pipe::EastWest)
            .map(|[row_p, col_p]| (row_p, col_p, &self.grid[row_p][col_p]))
            .collect_vec(); // Remove horizontal pipes
                            // println!("Points in same row, {:?}", pipes_in_loop);
        let mut edge_count = 0;
        let mut last_pipe = &Pipe::Blocked;
        for (_, _, pipe) in pipes_in_loop {
            match (last_pipe, pipe) {
                (SouthEast, NorthWest) => (), // Dont count edges in this case, since its been counted
                (NorthEast, SouthWest) => (),
                _ => {
                    // print!("({},{},{})->", row, col, pipe);
                    edge_count += 1;
                }
            };
            last_pipe = pipe;
        }
        // println!("edges {}", edge_count);
        edge_count % 2 == 1
    }
    pub fn count_area_inside_loop(&self) -> usize {
        let loop_points = self.get_all_points_in_loop().unwrap();
        (0..self.height)
            .cartesian_product(0..self.width)
            .filter(|(row_n, col_n)| !loop_points.contains(&[*row_n, *col_n]))
            .map(|(row_n, col_n)| {
                println!("{},", self.grid[row_n][col_n]);
                (row_n, col_n)
            })
            .filter(|(row_n, col_n)| self.is_inside_loop(*row_n, *col_n))
            .count()
    }
    pub fn farthest_point_distance(&self) -> Result<usize, String> {
        let loop_length = self.loop_length()?;
        if loop_length % 2 == 0 {
            Ok(loop_length / 2)
        } else {
            Ok((loop_length / 2) + 1)
        }
    }
    pub fn get_all_points_in_loop(&self) -> Result<Vec<[usize; 2]>, String> {
        let (start_row, start_col) = self
            .start
            .expect("Has to be, otherwise why are you calling this");
        // Assumes there IS a loop in start
        // We can start at the first, or second direction of self, lets do first
        let last_pipe = self
            .get(start_row, start_col)
            .expect("There has to be a pipe");
        // Start from whichever
        let mut approach_direction: Dir = last_pipe
            .to_dirs()
            .ok_or("The given start point is not a pipe with a direction".to_string())?
            .0;

        let mut last_row = start_row;
        let mut last_col = start_col;
        let mut points_in_loop = vec![[last_row, last_col]];
        loop {
            // println!("dir={:?}", approach_direction);
            (last_row, last_col, approach_direction) = self
                .next(last_row, last_col, &approach_direction)
                .ok_or_else(|| {
                    format!(
                        "The pipe (row={}, col={}) does not have a valid successor when approaching from {:?}",last_row, last_col,
                         approach_direction
                    )
                })?;

            if last_row == start_row && last_col == start_col {
                return Ok(points_in_loop
                    .iter()
                    .sorted_by(|[_, col_a], [_, col_b]| col_a.cmp(col_b))
                    .cloned()
                    .collect_vec());
            }
            points_in_loop.push([last_row, last_col]);
        }
    }
    pub fn loop_length(&self) -> Result<usize, String> {
        let all_points_in_loop = self.get_all_points_in_loop()?;
        Ok(all_points_in_loop.iter().count() - 1)
    }

    fn next(&self, row_n: usize, col_n: usize, comes_from: &Dir) -> Option<(usize, usize, Dir)> {
        let current = self.get(row_n, col_n)?;
        let next_dir = current.next(comes_from)?;
        let next_coordinates_offset = next_dir.to_coordinate_offset();
        let next_row_n = row_n.checked_add_signed(next_coordinates_offset.0)?;
        let next_col_n = col_n.checked_add_signed(next_coordinates_offset.1)?;
        let approaching_next_from_from = next_dir.complement();

        // row_n, col_n, current, comes_from,next_dir,next_row_n, next_col_n, approaching_next_from_from );
        Some((next_row_n, next_col_n, approaching_next_from_from))
    }

    fn get(&self, row_n: usize, col_n: usize) -> Option<&Pipe> {
        let row_n: usize = row_n.try_into().ok()?;
        let col_n: usize = col_n.try_into().ok()?;
        self.grid.get(row_n)?.get(col_n)
    }

    fn infer_pipe(&self, row_n: usize, col_n: usize) -> Option<Pipe> {
        let mut connected_directions = vec![];
        for dir in Dir::iter() {
            let (row_offset, col_offset) = dir.to_coordinate_offset();
            let next_row = row_n.checked_add_signed(row_offset);
            let next_column = col_n.checked_add_signed(col_offset);

            if let (Some(next_row), Some(next_column)) = (next_row, next_column) {
                let complement = dir.complement();
                if let Some(pipe) = self.get(next_row, next_column) {
                    // If we approach the next pipe from our north, we are approaching it from its south
                    // let see if its actually connected
                    if let Some(_) = pipe.next(&complement) {
                        // Yes! it is actually connected in the given direction (dir)
                        connected_directions.push(dir);
                    }
                }
            }
        }
        if connected_directions.len() == 2 {
            Some(Pipe::from_dirs(
                <[Dir; 2]>::try_from(connected_directions).unwrap(),
            ))
        } else {
            println!(
                "Impossible to infer pipe since connections not equal 2, but {}",
                connected_directions.len()
            );
            None
        }
    }
}
impl Display for PipeMaze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PipeMaze(")?;
        if let Some((row_n, col_n)) = self.start {
            let pipetype_of_start = self.grid.index(row_n).index(col_n);
            write!(
                f,
                "starts=(row={}, col={}, pipe={}),",
                row_n,
                col_n,
                pipetype_of_start.to_string()
            )?;
        }

        write!(f, "grid=(\n",)?;

        for row in self.grid.iter() {
            for pipe in row.iter() {
                f.write_str(&*pipe.to_string())?;
            }
            f.write_char('\n')?;
        }
        f.write_str("))")?;
        Ok(())
        //
    }
}
impl FromStr for PipeMaze {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cute_pipe_maze: String = ugly_pipe_maze_to_cute_pipe_maze(s);
        let mut start = None;
        let mut grid: Vec<Vec<Pipe>> = vec![];
        for (row_n, line) in cute_pipe_maze.trim().lines().enumerate() {
            let mut row = vec![];
            for (col_n, c) in line.trim().chars().enumerate() {
                if c == 'S' {
                    start = Some((row_n, col_n));
                }
                let as_pipe = Pipe::from_char(c);
                row.push(as_pipe);
            }
            grid.push(row);
        }

        if !grid.iter().map(|v| v.len()).all_equal() {
            return Err("Not all the rows have the same amount of characters!!".to_string());
        }
        let height = grid.len();
        let width = grid[0].len();
        let mut pipe_maze = PipeMaze {
            grid,
            start,
            height,
            width,
        };

        match start {
            None => Ok(pipe_maze),
            Some((row_n, col_n)) => {
                let inferred = pipe_maze.infer_pipe(row_n, col_n).expect("Inputs should only have one S, and it should only be inferrable to be a single kind");
                pipe_maze.start = Some((row_n, col_n));
                pipe_maze.grid[row_n][col_n] = inferred;
                Ok(pipe_maze)
            }
        }
    }
}
