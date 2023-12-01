fn get_first_digit_substring(the_buffer: &str) -> Option<usize> {
    const NUM_WORDS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let first_char = the_buffer.chars().next()?;
    if first_char.is_digit(10) {
        return Some(first_char.to_digit(10).unwrap() as usize);
    }
    NUM_WORDS
        .iter()
        .position(|&word| the_buffer.starts_with(word)) // The digit value is the index of the one we find in NUM_WORDS first
}

// Returns a tuple with the first match, and the index where the match was found
fn get_all_digits(the_buffer: &str) -> Vec<usize> {
    let mut nums: Vec<usize> = vec![];
    // First check first value, if it is straight up a digit
    for i in 0..the_buffer.len() {
        if let Some(digit) = get_first_digit_substring(&the_buffer[i..]) {
            nums.push(digit)
        }
    }
    return nums;
}

pub fn get_trebuchet(trebuchet: &str) -> Option<usize> {
    let all_digits: Vec<usize> = get_all_digits(trebuchet);

    let (first, last) = (all_digits.first()?, all_digits.last()?);

    Some(first * 10 + last)
}

pub fn get_trebuchet_multiple(trebuchet: &str) -> usize {
    let list_of_line_sums: Vec<usize> = trebuchet.lines().filter_map(get_trebuchet).collect();
    return list_of_line_sums.iter().sum();
}
