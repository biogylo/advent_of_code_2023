use crate::day_8_haunted_wasteland::Direction::{Left, Right};
use eqsolver::nalgebra::allocator::SameShapeC;
use itertools::{enumerate, Itertools};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
pub struct GhostKey {
    pub index: usize,
    pub full: String,
    last: char,
    last_index: usize,
}
impl Direction {
    fn from_char(c: char) -> Result<Direction, String> {
        match c {
            'L' => Ok(Left),
            'R' => Ok(Right),
            _ => Err(format!("Unable to parse char ({})", c).to_string()),
        }
    }
}
pub struct GhostMap {
    pub directions: Vec<Direction>,
    pub node_map: Vec<(GhostKey, GhostKey)>,
    pub node_hashmap: HashMap<String, (String, String)>, // Just for fun
    pub sorted_keys: Vec<GhostKey>,
    pub all_endings: Vec<char>,
}

impl GhostMap {
    fn iter_directions(&self) -> impl Iterator<Item = &Direction> {
        self.directions.iter().cycle()
    }

    pub fn walk_across_spacetime(&self, start: usize, end: usize) -> usize {
        let mut next_keys: Vec<&GhostKey> = self.get_all_nodes_ending_with(start);
        let mut step_iterator = self.iter_directions().enumerate();
        loop {
            //
            let (step_count, direction) = step_iterator.next().unwrap();

            if next_keys.iter().all(|gk| gk.last_index == end) {
                return step_count;
            }

            let mut keys_walked = vec![];
            for key in next_keys.into_iter() {
                let (left, right) = &self.node_map[key.index];
                let next_key = match direction {
                    Left => left,
                    Right => right,
                };
                keys_walked.push(next_key);
            }
            next_keys = keys_walked;
        }
    }

    pub fn get_all_nodes_ending_with(&self, index: usize) -> Vec<&GhostKey> {
        self.sorted_keys
            .iter()
            .filter(|&gk| gk.last_index == index)
            .collect_vec()
    }
    pub fn walk(&self, start: usize, end: usize) -> usize {
        let mut next_step_key = start;
        let mut step_iterator = self.iter_directions().enumerate();
        loop {
            //
            let (step_count, direction) = step_iterator.next().unwrap();
            if next_step_key == end {
                return step_count;
            }

            let (left, right) = &self.node_map[next_step_key];
            next_step_key = match direction {
                Left => left.index,
                Right => right.index,
            };
        }
    }
    pub fn walk_until_ending(&self, start: usize, end: usize) -> usize {
        let mut next_step_key = &self.sorted_keys[start];
        let mut step_iterator = self.iter_directions().enumerate();
        loop {
            let (step_count, direction) = step_iterator.next().unwrap();
            if next_step_key.last_index == end {
                return step_count;
            }

            let (left, right) = &self.node_map[next_step_key.index];
            next_step_key = match direction {
                Left => left,
                Right => right,
            };
        }
    }

    pub fn walk_across_spacetime_from_ending(&self, start: char, end: char) -> usize {
        let start_index = self
            .all_endings
            .iter()
            .position(|key| key == &start)
            .expect("You shouldn't do this");
        let end_index = self
            .all_endings
            .iter()
            .position(|key| key == &end)
            .expect("You shouldn't do this");
        self.walk_across_spacetime(start_index, end_index)
    }
    pub fn walk_from_key(&self, start: String, end: String) -> usize {
        let start_index = self
            .sorted_keys
            .iter()
            .position(|key| &key.full == &start)
            .expect("You shouldn't do this");
        let end_index = self
            .sorted_keys
            .iter()
            .position(|key| &key.full == &end)
            .expect("You shouldn't do this");
        self.walk(start_index, end_index)
    }
}
impl FromStr for GhostMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut directions: Vec<Direction> = vec![];
        let mut node_map: Vec<(GhostKey, GhostKey)> = vec![];
        let mut node_hashmap: HashMap<String, (String, String)> = HashMap::new();

        for line in s.trim().lines() {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            };
            if !trimmed_line.contains('=') {
                // Parse directions
                let directions_in_line = trimmed_line
                    .chars()
                    .map(|c| Direction::from_char(c).ok())
                    .collect::<Option<Vec<Direction>>>()
                    .ok_or("Unable to parse all direction chars")?;
                directions.extend(directions_in_line);
            } else {
                // Parse key values
                let (key_token, tuple_token) = trimmed_line
                    .split_once("=")
                    .ok_or("Unable to get key and tuple from line!".to_string())?;

                let trimmed_token = tuple_token.trim().replace(")", "").replace("(", "");
                let (left, right) = trimmed_token
                    .split_once(",")
                    .ok_or("Unable to split the tuple token!".to_string())?;
                node_hashmap.insert(
                    key_token.trim().to_string(),
                    (left.trim().to_string(), right.trim().to_string()),
                );
            };
        }

        let sorted_keys = node_hashmap.keys().sorted().cloned().collect_vec();

        let all_endings = sorted_keys
            .iter()
            .map(|s| s.chars().last().expect("Has to be a key"))
            .unique()
            .sorted()
            .collect_vec();

        let mut string_to_ghostkey: HashMap<String, GhostKey> = HashMap::new();
        for (index, key) in enumerate(&sorted_keys) {
            let last = key
                .chars()
                .last()
                .expect("There has to be a last char in here");

            let last_index = all_endings
                .iter()
                .position(|c| c == &last)
                .expect("Impossible! There has to be one char of the given");

            let gk = GhostKey {
                index,
                full: key.clone(),
                last,
                last_index,
            };
            string_to_ghostkey.insert(key.clone(), gk);
        }

        let mut node_map: Vec<(GhostKey, GhostKey)> = sorted_keys
            .iter()
            .map(|key| node_hashmap[key].clone())
            .map(|(s1, s2)| {
                (
                    string_to_ghostkey[&s1].clone(),
                    string_to_ghostkey[&s2].clone(),
                )
            })
            .collect_vec();

        let sorted_keys = sorted_keys
            .iter()
            .map(|s| string_to_ghostkey[s].clone())
            .collect_vec();
        Ok(GhostMap {
            directions,
            node_map,
            node_hashmap,
            sorted_keys,
            all_endings,
        })
    }
}
