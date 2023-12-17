use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Eq, PartialEq, Hash)]
struct StringAndVectorKey {
    the_string: String,
    the_vector: Vec<usize>,
}

impl StringAndVectorKey {
    fn from_parameters(the_string: &[char], the_vector: &[usize]) -> StringAndVectorKey {
        let the_string: String = the_string.iter().collect();
        let the_vector: Vec<usize> = the_vector.iter().cloned().collect_vec();
        StringAndVectorKey {
            the_string,
            the_vector,
        }
    }
}
pub struct HotSpringsMemo {
    memory: HashMap<StringAndVectorKey, usize>,
}

impl HotSpringsMemo {
    const UNKNOWN: char = '?';
    const OPERATIONAL: char = '.';
    const DAMAGED: char = '#';
    pub fn new() -> HotSpringsMemo {
        HotSpringsMemo {
            memory: HashMap::new(),
        }
    }

    pub fn get_arrangements_for_row(
        &mut self,
        the_row: &str,
        unfold_n: usize,
    ) -> Result<usize, String> {
        let (condition_record, count_tokens) = the_row
            .trim()
            .split_once(" ")
            .ok_or("Unable to parse row".to_string())?;

        if !condition_record.chars().all(|c| {
            c == HotSpringsMemo::UNKNOWN
                || c == HotSpringsMemo::OPERATIONAL
                || c == HotSpringsMemo::DAMAGED
        }) {
            return Err(
                "Not all the chars in a condition record were correct (. # ?)!".to_string(),
            );
        }

        let counts: Vec<usize> = count_tokens
            .split(",")
            .map(|token| token.parse())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| "Unable to parse all number tokens from a row")?;
        let record_array = condition_record.chars().collect_vec();

        let mut unfolded_record_array = record_array.clone();
        let mut unfolded_counts = counts.clone();

        for i in (1..unfold_n) {
            unfolded_record_array.push(HotSpringsMemo::UNKNOWN);
            unfolded_record_array.extend(record_array.clone());
            unfolded_counts.extend(counts.clone());
        }

        return Ok(HotSpringsMemo::count_matching_patterns(
            unfolded_record_array.as_slice(),
            unfolded_counts.as_slice(),
        ));
    }

    pub fn get_arrangements_for_paragraph(
        &mut self,
        the_paragraph: &str,
        unfold_n: usize,
    ) -> Result<usize, String> {
        the_paragraph
            .trim()
            .lines()
            .map(|the_row| self.get_arrangements_for_row(the_row, unfold_n))
            .fold_ok(0, Add::add)
    }

    fn cached_count_matching_patters(
        &mut self,
        the_string: &[char],
        the_vector: &[usize],
    ) -> usize {
        let key = StringAndVectorKey::from_parameters(the_string, the_vector);
        if let Some(cache) = self.memory.get(&key) {
            return *cache;
        }
        let result = HotSpringsMemo::count_matching_patterns(the_string, the_vector);
        self.memory.insert(key, result);
        result
    }
    fn count_matching_patterns(the_string: &[char], the_vector: &[usize]) -> usize {
        if the_vector.len() == 0 {
            if the_string.len() == 0 {
                // Defo a match
                return 1;
            } else if the_string.iter().all(|c| c != &HotSpringsMemo::DAMAGED) {
                return 1;
            } else {
                return 0;
            }
        }
        let mut matches = 0;
        let match_count = the_vector[0];
        let rest_vec = &the_vector[1..];

        for (i, _) in the_string.windows(match_count).enumerate() {
            if the_string[i..i + match_count]
                .iter()
                .all(|c| c != &HotSpringsMemo::OPERATIONAL)
            {
                // A possible match: three scenatios
                if let Some(next_char) = the_string.get(i + match_count) {
                    if next_char != &HotSpringsMemo::DAMAGED {
                        // This could very well be a match, but only if the rest is a match
                        matches += HotSpringsMemo::count_matching_patterns(
                            &the_string[i + match_count + 1..],
                            rest_vec,
                        );
                    } else {
                        // Not a match... We need to iterate further
                    }
                } else {
                    // Exhausted the string
                    if rest_vec.len() == 0 {
                        matches += 1;
                    }
                    return matches;
                }
            }
            if the_string[i] == HotSpringsMemo::DAMAGED {
                // Wont be matches in the next loops
                return matches;
            }
        }
        return matches;
    }
}
