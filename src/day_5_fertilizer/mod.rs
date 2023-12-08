use itertools::Itertools;
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;

use std::isize;
use std::ops::Range;
use std::str::FromStr;
use strum_macros::Display;

pub struct MapRangeList {
    vector: Vec<MapRange>,
}

impl MapRangeList {
    fn from_vec(vector: Vec<MapRange>) -> MapRangeList {
        MapRangeList {
            vector: vector
                .into_iter()
                .sorted_by(|map_range_a, map_range_b| {
                    map_range_a.source.start.cmp(&map_range_b.source.start)
                })
                .collect_vec(),
        }
    }
}
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
pub struct NumberSet {
    vector: Vec<Range<isize>>, // For order
}
fn compare_ranges_start(range_a: &Range<isize>, range_b: &Range<isize>) -> Ordering {
    range_a.start.cmp(&range_b.start)
}
impl NumberSet {
    fn from_vec(vector: Vec<Range<isize>>) -> NumberSet {
        NumberSet::from_iter(vector.into_iter())
    }
    fn from_iter<T>(range_iter: T) -> NumberSet
    where
        T: Iterator<Item = Range<isize>>,
    {
        let cheat = range_iter.collect_vec();
        let mut iter = cheat.into_iter().sorted_by(compare_ranges_start).clone();
        let mut new_bounds = vec![];
        if let Some(last_range) = iter.next() {
            let mut previous_range = last_range.clone();
            for current_range in iter {
                // previous range's start HAS to be equal or smaller than current, since we sorted
                // previous     ->  { }
                // current  ->  < >
                // So its either,  { } < >   or  {< }>
                if current_range.start > previous_range.end {
                    // No overlap, we push them straight up
                    new_bounds.push(previous_range.clone());
                    previous_range = current_range.clone();
                } else if previous_range.end < current_range.end {
                    // They overlap, remove the bound in the middle, take the smallest left and the largest right bound
                    // We sorted by end, however this doesn't guarantee anything
                    previous_range =
                        previous_range.start..max(previous_range.end, current_range.end);
                }
            }

            new_bounds.push(previous_range);
        }
        NumberSet { vector: new_bounds }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Range<isize>> {
        self.vector.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Range<isize>> {
        self.vector.into_iter()
    }
    fn apply_mapping(self, map_list: &MapRangeList) -> NumberSet {
        println!(
            "Trying to map: {:?}, to maps {:?}",
            self.vector, map_list.vector
        );
        let mut mapped_ranges: Vec<Range<isize>> = vec![];
        let mut last_exhausted_map: usize = 0;
        let mut ranges = self.into_iter();
        let mut next_range_to_address = ranges.next();
        // Assumes both the map list and the map vector are ordered
        loop {
            match next_range_to_address {
                None => {
                    println!("Finished, all addressed");
                    return NumberSet::from_vec(mapped_ranges);
                }
                Some(ref range) => {
                    let current_range = range.clone();
                    if last_exhausted_map >= map_list.vector.len() {
                        println!("Exhausted all maps, recycling {:?}", current_range);
                        mapped_ranges.push(current_range);
                        next_range_to_address = ranges.next()
                    } else {
                        for i in last_exhausted_map..map_list.vector.len() {
                            let last_map: &MapRange = &map_list.vector[i];
                            let map_range: &Range<isize> = &last_map.source;
                            println!(
                                "Trying to fractionate {:?} to map {:?}",
                                current_range, map_range
                            );

                            if map_range.start >= current_range.end {
                                println!("Too smol! Recycling");
                                mapped_ranges.push(current_range);
                                next_range_to_address = ranges.next();
                                break;
                            }

                            if current_range.end >= map_range.end {
                                println!("Map exhausted");
                                last_exhausted_map += 1;
                                if current_range.start >= map_range.end {
                                    println!("No overlap");
                                    // Not overlapping, check next map
                                    continue;
                                }
                            }
                            let intersection = max(current_range.start, map_range.start)
                                ..min(current_range.end, map_range.end);
                            println!("Intersection found! {:?}", intersection);
                            mapped_ranges.push(last_map.get_unchecked(&intersection));

                            // In the left side. Its unchecked, gets recycled
                            if map_range.start > current_range.start {
                                let left_complement =
                                    current_range.start..min(current_range.end, map_range.start);
                                println!("Unmapped {:?}", left_complement);
                                mapped_ranges.push(left_complement);
                            }

                            // Determine if there was any left over to the range

                            // In the right side, it needs to be checked in another map
                            if current_range.end > map_range.end {
                                let right_complement = map_range.end..current_range.end;

                                println!("Leftover to retry {:?}", right_complement);
                                next_range_to_address = Some(right_complement);
                                break;
                            } else {
                                println!("Moving on");
                                next_range_to_address = ranges.next();
                                println!("NEXT UP {:?}, ", next_range_to_address);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn from_range(range: &Range<isize>) -> NumberSet {
        NumberSet::from_vec(vec![range.clone()])
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
    fn get(&self, number_set: NumberSet) -> NumberSet {
        let mut last_key = self.base_key_name.clone();
        let mut current_range = number_set;
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
        self.get(number_set)
    }
    pub fn get_seeds_as_ranges(&self) -> NumberSet {
        NumberSet::from_iter(
            self.base_values
                .clone()
                .chunks(2)
                .into_iter()
                .map(|num| {
                    println!("Chunks: {} {}", num[0], num[1]);
                    num
                })
                .map(|slice| (slice[0]..slice[0] + slice[1])),
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
        let answer = self.get(key_ranges);
        answer.vector[0].start
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
    pub fn into_vec(self) -> Vec<MapRange> {
        self.vector
    }
}
impl FromStr for MapRangeList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map_list: Vec<MapRange> = s
            .trim()
            .lines()
            .map(|x| {
                println!("lines {}", x);
                x
            })
            .map(MapRange::from_str)
            .map(|range| range.unwrap())
            .collect_vec();
        Ok(Self::from_vec(map_list))
    }
}

#[derive(Debug)]
pub struct MapRange {
    pub source: Range<isize>,
    pub destination: Range<isize>,
    offset: isize,
}

impl MapRange {
    // Returns the value range based on a key range assumed to be in range
    fn get_unchecked(&self, range: &Range<isize>) -> Range<isize> {
        if (range.start < self.source.start || range.end > self.source.end) {
            println!(
                "BAD MAPPING key: {:?}, source: {:?}, dest: {:?}",
                range, self.source, self.destination,
            );
        }
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
        let map_range = Self {
            source: source_start..(source_start + length),
            destination: destination_start..(destination_start + length),
            offset: (destination_start as isize - source_start as isize),
        };
        Ok(map_range)
    }
}
