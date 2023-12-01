/*
Problem 1:

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

#[cfg(test)]
mod tests {
    use advent_of_code_2023::trebuchet::{get_trebuchet, get_trebuchet_multiple};
    use std::fs;
    #[test]
    fn single_line_example_short_first_last() {
        let example = "1abc2";

        let result = get_trebuchet(example).unwrap();
        assert_eq!(result, 12);
    }
    #[test]
    fn single_line_example_2_digits_middle() {
        let example = "pqr3stu8vwx";

        let result = get_trebuchet(example).unwrap();
        assert_eq!(result, 38);
    }
    #[test]
    fn single_line_example_single_digit() {
        let example = "treb7uchet";

        let result = get_trebuchet(example).unwrap();
        assert_eq!(result, 77);
    }
    #[test]
    fn single_line_example_multiple_digits() {
        let example = "a1b2c3d4e5f";

        let result = get_trebuchet(example).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn multi_line_example_short() {
        let example = fs::read_to_string("./data/trebuchet_input_short.txt").unwrap();
        let num = get_trebuchet_multiple(&example);
        assert_eq!(num, 142);
    }

    #[test]
    fn multi_line_example_long() {
        let example = fs::read_to_string("./data/trebuchet_input_long.txt").unwrap();
        let num = get_trebuchet_multiple(&example);
        assert_eq!(num, 55686);
    }

    #[test]
    fn all_spelled_out() {
        let example = "two1nine";
        let num = get_trebuchet(example).unwrap();
        assert_eq!(num, 29);
    }

    #[test]
    fn spelled_out_should_work() {
        let example = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let num = get_trebuchet_multiple(example);
        assert_eq!(num, 281);
    }
}
