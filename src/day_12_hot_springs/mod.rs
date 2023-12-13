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

    pub fn get_arrangements_for_row(&mut self, the_row: &str) -> Result<usize, String> {
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
        let condition_record_char_array = condition_record.chars().collect_vec();

        return Ok(
            self.count_matching_patterns(condition_record_char_array.as_slice(), counts.as_slice())
        );
    }

    pub fn get_arrangements_for_paragraph(&mut self, the_paragraph: &str) -> Result<usize, String> {
        the_paragraph
            .trim()
            .lines()
            .map(|the_row| self.get_arrangements_for_row(the_row))
            .fold_ok(0, Add::add)
    }

    fn count_matching_patterns(&mut self, the_string: &[char], the_vector: &[usize]) -> usize {
        // if the_string.len() == 0 && the_vector.len() == 0 {
        //     return 1;
        // }
        if the_vector.len() == 0 {
            // Ran out of options, all the leftovers must be either unknown or unset
            if the_string
                .iter()
                .all(|&c| c == HotSpringsMemo::UNKNOWN || c == HotSpringsMemo::OPERATIONAL)
            {
                return 1;
            } else {
                return 0;
            }
        }
        let first_count = the_vector[0];
        if first_count > the_string.len() {
            return 0;
        }
        let params = StringAndVectorKey::from_parameters(the_string, the_vector);

        if let Some(memoized_value) = self.memory.get(&params) {
            return *memoized_value;
        }

        let mut matches = 0;
        // Find first match
        for (i, window) in the_string.windows(first_count).enumerate() {
            // We can only loop if we have chunks at least the size of our first count
            // And since there HAS to be a match, since the input is assumed to be valid,
            if window
                .iter()
                .all(|&c| c == HotSpringsMemo::UNKNOWN || c == HotSpringsMemo::DAMAGED)
            {
                // If we exhausted the string, and there is leftover on the vector, this is not a valid arrangement
                let first_char_in_next_window_index = i + first_count;
                let chars_left = the_string.len() - (first_char_in_next_window_index);

                if chars_left == 0 {
                    if the_vector.len() >= 2 {
                        return 0;
                    } else {
                        return 1;
                    }
                }

                if chars_left == 1 {
                    if the_string[first_char_in_next_window_index] == HotSpringsMemo::DAMAGED {
                        // Not a valid arrangement!
                        return 0;
                    } else {
                        return 1;
                    }
                }

                matches += self.count_matching_patterns(
                    &the_string[first_char_in_next_window_index..],
                    &the_vector[1..],
                );
            }
            // We decide whether to continue the loop, if the first one is not damaged
            // because if it is, then it means we cannot feasibly match the first group anymore
            if window[0] == HotSpringsMemo::DAMAGED {
                break;
            }
        }

        self.memory.insert(params, matches);
        return matches;
    }
}
