use crate::day_13_point_of_incidence::Observation::{Ash, Rock};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone)]
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
fn is_reflection_row(
    elements: &Vec<Vec<Observation>>,
    between: usize,
    number_of_smudges: usize,
) -> bool {
    if between == 0 || between == elements.len() {
        return false;
    }
    // At this point we know we are in inbetweens
    // Gotta save a list of up and down
    let up = &elements[0..between];
    let down = &elements[between..];

    // Gotta reverse up, and compare row by row
    let up_rev = up.iter().rev();
    // Instead, we must check how many differences each pair of rows has
    // and it should sum up to our number of smudges
    up_rev.zip(down).map(count_differences).sum::<usize>() == number_of_smudges
}

fn count_differences<A>((v1, v2): (&Vec<A>, &Vec<A>)) -> usize
where
    A: PartialEq,
{
    v1.iter().zip(v2.iter()).filter(|(a, b)| a != b).count()
}
impl Cluster {
    fn find_horizontal_reflection_row(&self, number_of_smudges: usize) -> Option<usize> {
        for i in 0..self.elements.len() {
            if is_reflection_row(&self.elements, i, number_of_smudges) {
                return Some(i);
            }
        }
        None
    }
    fn find_vertical_reflection_col(&self, number_of_smudges: usize) -> Option<usize> {
        // Gotta rotate
        let mut rotated = vec![];
        for i in 0..self.elements[0].len() {
            let row = self
                .elements
                .iter()
                .map(|row| &row[i])
                .cloned()
                .collect_vec();
            rotated.push(row);
        }

        for i in 0..self.elements[0].len() {
            if is_reflection_row(&rotated, i, number_of_smudges) {
                return Some(i);
            }
        }
        None
    }

    fn summarize_pattern(self, number_of_smudges: usize) -> usize {
        let reflection_row = self
            .find_horizontal_reflection_row(number_of_smudges)
            .unwrap_or(0)
            * 100;
        let reflection_col = self
            .find_vertical_reflection_col(number_of_smudges)
            .unwrap_or(0);
        return reflection_col + reflection_row;
    }
}

pub fn summarize_reflections(buffer: &str, number_of_smudges: usize) -> usize {
    let all_clusters = buffer.trim().split("\n\n");
    let mut sum = 0;
    for cluster_str in all_clusters {
        let cluster: Cluster = cluster_str.parse().unwrap();
        sum += cluster.summarize_pattern(number_of_smudges);
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
