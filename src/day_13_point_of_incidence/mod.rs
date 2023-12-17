use crate::day_13_point_of_incidence::Observation::{Ash, Rock};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
enum Observation {
    Ash,
    Rock,
}
impl Observation {
    fn from_char(c: char) -> Result<Observation, String> {
        match c {
            '#' => Ok(Ash),
            '.' => Ok(Rock),
            _ => Err(format!("Unable to parse char to Observation ({:?})", c).to_string()),
        }
    }
}
struct Cluster {
    elements: Vec<Vec<Observation>>,
}
fn is_reflection_row(elements: &Vec<Vec<Observation>>, between: usize) -> bool {
    if between == 0 || between == elements.len() {
        return false;
    }
    // At this point we know we are in inbetweens
    // Gotta save a list of up and down
    let up = &elements[0..between];
    let down = &elements[between..];

    // Gotta reverse up, and compare row by row
    let up_rev = up.iter().rev();

    up_rev.zip(down).all(|(up, down)| up == down)
}

impl Cluster {
    fn find_horizontal_reflection_row(self) -> Option<usize> {
        for i in (0..self.elements.len()) {
            if is_reflection_row(&self.elements, i) {
                return Some(i);
            }
        }
        None
    }
    fn find_vertical_reflection_col(self) -> Option<usize> {
        // Gotta rotate
        let mut rotated = vec![];
        for i in (0..self.elements[0].len()) {
            let row = self.elements.iter().map(|row| row[i]).collect_vec();
            rotated.push(row);
        }

        for i in (0..self.elements[0].len()) {
            if is_reflection_row(&rotated, i) {
                return Some(i);
            }
        }
        None
    }

    fn summarize_pattern(self) -> usize {
        let reflection_row = self.find_horizontal_reflection_row().unwrap_or(0) * 100;
        let reflection_col = self.find_vertical_reflection_col().unwrap_or(0);
        return reflection_col + reflection_row;
    }
}

fn summarize_reflections(buffer: &str) -> usize {
    let all_clusters = buffer.trim().split("\n\n");
    let mut sum = 0;
    for cluster_str in all_clusters {
        let cluster: Cluster = cluster_str.parse().unwrap();
        sum += cluster.summarize_pattern();
    }
    sum
}
impl FromStr for Cluster {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = vec![];
        for line in s.trim().lines() {
            let row = line
                .chars()
                .map(|c| Observation::from_char(c))
                .collect::<Result<Vec<Observation>, String>>()?;
            elements.push(row);
        }
        Ok(Self { elements })
    }
}
