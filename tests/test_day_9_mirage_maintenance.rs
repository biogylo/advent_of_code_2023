#[cfg(test)]
mod test_challenge_parsing {
    use advent_of_code_2023::day_9_mirage_maintenance::{parse_report, History};
    use itertools::Itertools;
    use std::fs;

    #[test]
    fn parse_oasis_line_correctly() {
        let buffer = "0 3 6 9 12 15";
        let history: History = buffer.parse().unwrap();
        assert_eq!(history.values, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn parse_oasis_line_correctly_and_do_extrapolation() {
        let buffer = "0 3 6 9 12 15";
        let history: History = buffer.parse().unwrap();
        assert_eq!(history.naive_extrapolate(), 18);
    }

    #[test]
    fn parse_oasis_line_correctly_and_do_extrapolation_txt() {
        let buffer = fs::read_to_string("./data/mirage_maintenance_input_short.txt").unwrap();
        let histories: Vec<History> = parse_report(&buffer).unwrap();
        let ans_vec = histories
            .iter()
            .map(|h| h.naive_extrapolate())
            .collect_vec();
        assert_eq!(ans_vec, vec![18, 28, 68]);
    }

    #[test]
    fn parse_oasis_multiline_correctly() {
        let buffer = fs::read_to_string("./data/mirage_maintenance_input_short.txt").unwrap();
        let histories: Vec<History> = parse_report(&buffer).unwrap();

        let vector1 = vec![0, 3, 6, 9, 12, 15];
        let vector2 = vec![1, 3, 6, 10, 15, 21];
        let vector3 = vec![10, 13, 16, 21, 30, 45];

        assert_eq!(histories[0].values, vector1);
        assert_eq!(histories[1].values, vector2);
        assert_eq!(histories[2].values, vector3);
    }

    #[test]
    fn parse_oasis_multiline_correctly_and_do_extrapolation_long_txt() {
        let buffer = fs::read_to_string("./data/mirage_maintenance_input_long.txt").unwrap();
        let histories: Vec<History> = parse_report(&buffer).unwrap();
        let sum_of_histories: isize = histories.iter().map(|h| h.naive_extrapolate()).sum();
        assert_eq!(sum_of_histories, 1939607039);
    }

    #[test]
    fn parse_oasis_line_correctly_and_do_extrapolation_reverse() {
        let buffer = "10  13  16  21  30  45";
        let history: History = buffer.parse().unwrap();
        assert_eq!(history.naive_extrapolate_rev(), 5);
    }

    #[test]
    fn parse_oasis_multiline_correctly_and_do_reverse_extrapolation_long_txt() {
        let buffer = fs::read_to_string("./data/mirage_maintenance_input_long.txt").unwrap();
        let histories: Vec<History> = parse_report(&buffer).unwrap();
        let sum_of_histories: isize = histories.iter().map(|h| h.naive_extrapolate_rev()).sum();
        assert_eq!(sum_of_histories, 1041);
    }
}
