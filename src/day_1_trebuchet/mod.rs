const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/*
   Takes in a string slice, and returns the first digit substring
   it finds. The first digit can come either as a digit char, or a word
   (E.g. one, two, three)
*/
fn get_first_digit_substring(the_buffer: &str) -> Option<u32> {
    let lowercase_buffer: String = the_buffer.to_lowercase();
    // First check the first char, to see if it is a digit
    if let Some(digit) = the_buffer.chars().next()?.to_digit(10) {
        return Some(digit);
    }
    DIGIT_WORDS
        .iter()
        .position(|digit_word: &&str| lowercase_buffer.starts_with(digit_word))
        .map(|num| num as u32)
}

/*
    From a string, it returns all the digit substrings
     in the string. Possible digit substrings are "1", "2", "two", "four".
*/
fn get_all_digits(the_buffer: &str) -> Vec<u32> {
    (0..the_buffer.len())
        .map(|i| &the_buffer[i..])
        .map(get_first_digit_substring)
        .flatten()
        .collect()
}

pub fn get_trebuchet(trebuchet: &str) -> Option<u32> {
    let all_digits: Vec<u32> = get_all_digits(trebuchet);

    let (first, last) = (all_digits.first()?, all_digits.last()?);

    Some(first * 10 + last)
}

pub fn get_trebuchet_multiple(trebuchet: &str) -> u32 {
    let list_of_line_sums: Vec<u32> = trebuchet.lines().filter_map(get_trebuchet).collect();
    return list_of_line_sums.iter().sum();
}
