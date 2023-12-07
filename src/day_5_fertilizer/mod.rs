use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use std::isize;
use std::ops::Range;
use std::slice::Iter;
use std::str::FromStr;

pub struct MapRangeList(pub Vec<MapRange>);

pub struct Almanac {
    base_values: Vec<isize>,
    base_key_name: String,
    b_to_value_map: HashMap<String, MapRangeList>,
    a_to_b_map: HashMap<String, String>,
}
pub trait SetOperate {
    fn intersection(&self, range_b: &Range<isize>) -> Option<Range<isize>>;
}

impl SetOperate for Range<isize> {
    fn intersection(&self, range_b: &Range<isize>) -> Option<Range<isize>> {
        let range_a = self;
        if range_a.start >= range_b.end || range_b.start >= range_a.end {
            return None;
        }
        let left_bound = max(range_a.start, range_b.start);
        let right_bound = min(range_a.end, range_b.end);
        Some(left_bound..right_bound)
    }
}
pub struct NumberSet(HashSet<Range<isize>>);
impl NumberSet {
    fn empty() -> NumberSet {
        NumberSet { 0: HashSet::new() }
    }

    fn contains_range(&self, range: &Range<isize>) -> bool {
        self.0.contains(range)
    }
    fn push(&mut self, number_set: NumberSet) {
        self.0.extend(number_set.iter().cloned());
    }
    fn push_range(&mut self, range: Range<isize>) {
        self.0.insert(range);
    }

    fn from_vec(vector: Vec<Range<isize>>) -> NumberSet {
        NumberSet::from_iter(vector.iter().cloned())
    }

    fn from_iter(iter: impl Iterator<Item = Range<isize>>) -> NumberSet {
        NumberSet {
            0: HashSet::from_iter(iter),
        }
    }

    fn fractionate(
        range_a: &Range<isize>,
        range_b: &Range<isize>,
    ) -> (Option<Range<isize>>, NumberSet /*nonoverlap*/) {
        // Returns the overlapping range, and the non-overlapping range
        if range_a.start >= range_b.end || range_b.start >= range_a.end {
            return (None, NumberSet::from_range(range_a));
        }
        // There is two situations, one set contains the other one, or it contains the side of it
        let left_bound;
        let mut left_over_ranges: Vec<Range<isize>> = vec![];
        if range_b.start < range_a.start {
            left_bound = range_a.start;
        } else if range_a.start < range_b.start {
            left_bound = range_b.start;
            left_over_ranges.push(range_a.start..range_b.start)
        } else {
            left_bound = range_b.start;
        }
        let right_bound;
        if range_a.end == range_a.end {
            right_bound = range_a.end;
        } else if range_a.end > range_b.end {
            right_bound = range_b.end;
            left_over_ranges.push(range_b.end..range_a.end);
        } else {
            right_bound = range_a.end;
        }
        (
            Some(left_bound..right_bound),
            NumberSet::from_vec(left_over_ranges),
        )
    }
    fn ranges_iter(&self) -> impl Iterator<Item = &Range<isize>> {
        self.0.iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Range<isize>> {
        self.ranges_iter()
            .sorted_by(|range, other| range.start.cmp(&other.start))
    }
    fn apply_mapping(&self, map_list: &MapRangeList) -> NumberSet {
        let mut mapped_values: NumberSet = NumberSet::empty();
        let mut overlapping_numbersets: NumberSet = NumberSet::empty();
        let mut all_splits: NumberSet = NumberSet::empty();
        for range in self.iter() {
            for map in map_list.iter() {
                let (overlapping, non_overlapping) = NumberSet::fractionate(&map.source, &range);
                if let Some(intersection) = overlapping {
                    mapped_values.push_range(map.get_unchecked(&intersection));
                    overlapping_numbersets.push_range(intersection);
                }
                all_splits.push(non_overlapping);
            }
        }
        let unused_values = all_splits
            .iter()
            .filter(|&range| !overlapping_numbersets.contains_range(range))
            .cloned();

        mapped_values.push(NumberSet::from_iter(unused_values));
        mapped_values
    }

    fn from_range(range: &Range<isize>) -> NumberSet {
        NumberSet {
            0: HashSet::from([range.clone()]),
        }
    }

    fn from_numbersets(numbersets: Vec<NumberSet>) -> NumberSet {
        let ranges: Vec<Range<isize>> = numbersets
            .into_iter()
            .flat_map(|numberset| numberset.0)
            .collect_vec();
        NumberSet::from_vec(ranges)
    }
}
impl Almanac {
    fn get_numberset_for_range(&self, range: &Range<isize>) -> NumberSet {
        let mut last_key = self.base_key_name.clone();
        let mut current_range = NumberSet::from_range(range);
        loop {
            // First iter
            if let Some(b_key) = self.a_to_b_map.get(&last_key) {
                let range_list = self.b_to_value_map.get(b_key).unwrap();
                current_range = current_range.apply_mapping(&range_list);
                last_key = b_key.to_string();
            } else {
                return current_range;
            }
        }
    }

    fn get_numberset_for_numberset(&self, number_set: NumberSet) -> NumberSet {
        let numberset_vec: Vec<NumberSet> = number_set
            .iter()
            .map(|range| self.get_numberset_for_range(range))
            .collect();
        NumberSet::from_numbersets(numberset_vec)
    }
    pub fn get_seeds_as_ranges(&self) -> NumberSet {
        NumberSet::from_iter(
            self.base_values
                .clone()
                .into_iter()
                .tuples()
                .map(|(start, length)| (start..start + length)),
        )
    }
    pub fn get_seeds_as_individual_ranges(&self) -> NumberSet {
        NumberSet::from_iter(
            self.base_values
                .iter()
                .map(|&value| (value..value + 1))
                .clone(),
        )
    }
    pub fn get_lowest_seed_ranges_locations(&self) -> isize {
        let key_ranges = self.get_seeds_as_ranges();
        self.get_numberset_for_numberset(key_ranges)
            .iter()
            .next()
            .unwrap()
            .start
    }
    pub fn get_lowest_individual_seed_location(&self) -> isize {
        let key_ranges = self.get_seeds_as_individual_ranges();
        self.get_numberset_for_numberset(key_ranges)
            .iter()
            .next()
            .unwrap()
            .start
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First line is the base key
        let (base_key_name, rest_of_tokens) = s
            .split_once("s:")
            .ok_or("Unable to find base key".to_string())?;

        let (base_values_str, mappings) = rest_of_tokens
            .split_once("\n")
            .ok_or("Not enough lines!".to_string())?;

        let base_values: Vec<isize> = base_values_str
            .trim()
            .split_whitespace()
            .map(|value_str| value_str.parse())
            .try_collect()
            .ok()
            .ok_or("Unable to parse base values!".to_string())?;
        println!(
            "Base key is '{}' and the values '{:?}'",
            base_key_name, base_values,
        );
        let mut a_to_b_map: HashMap<String, String> = HashMap::new();
        let mut b_to_value_map: HashMap<String, MapRangeList> = HashMap::new();
        let mut remaining_text = mappings.trim().to_owned() + "\n\n";
        loop {
            let rest = remaining_text;
            let (a_key, rest) = rest.split_once("-to-").ok_or("Error parsing a key!")?;

            let (b_key, rest) = rest.split_once("map:").ok_or("Error Parsing b key")?;

            let (map_lines, rest) = rest.split_once("\n\n").ok_or("Error parsing map range")?;
            let map_range_list: MapRangeList = map_lines.parse()?;

            a_to_b_map.insert(a_key.trim().to_string(), b_key.trim().to_string());
            b_to_value_map.insert(b_key.trim().to_string(), map_range_list);

            if rest.trim().len() == 0 {
                break;
            }
            remaining_text = rest.to_string();
        }
        let base_key_name = base_key_name.trim().to_string();
        Ok(Self {
            base_values,
            base_key_name,
            b_to_value_map,
            a_to_b_map,
        })
    }
}

impl MapRangeList {
    pub fn iter(&self) -> Iter<'_, MapRange> {
        self.0.iter()
    }
}
impl FromStr for MapRangeList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map_list: Vec<MapRange> = s
            .trim()
            .lines()
            .map(MapRange::from_str)
            .flatten()
            .collect_vec();
        Ok(Self { 0: map_list })
    }
}

pub struct MapRange {
    pub source: Range<isize>,
    pub destination: Range<isize>,
    offset: isize,
}

impl MapRange {
    // Returns the value range based on a key range assumed to be in range
    fn get_unchecked(&self, range: &Range<isize>) -> Range<isize> {
        range.start + self.offset..range.end + self.offset
    }
}

impl FromStr for MapRange {
    type Err = String;

    /*
        Takes in a clean line with the three variables
        destination_start source_start length
    */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_start, source_start, length): (isize, isize, isize) = s
            .trim()
            .split_whitespace()
            .map(|substring| substring.trim().parse().ok())
            .while_some()
            .next_tuple()
            .ok_or("Some items couldn't be parsed".to_string())?;
        Ok(Self {
            source: (source_start..source_start + length),
            destination: (destination_start..destination_start + length),
            offset: (destination_start as isize - source_start as isize),
        })
    }
}
