#[cfg(test)]
mod test_simple_struct_parsing {
    use advent_of_code_2023::day_4_scratch_cards::{ScratchCard, ScratchPile};
    use std::fs;

    #[test]
    fn single_scratchcard_parsed_correctly() {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratch: ScratchCard = example.parse().unwrap();
        assert_eq!(scratch.matches(), vec![83, 86, 17, 48]);
        assert_eq!(scratch.match_count(), 4);
    }
    #[test]
    fn scratchpile_example_parsed_correctly() {
        let example = fs::read_to_string("./data/scratchcards_input_short.txt").unwrap();
        let scratch: ScratchPile = example.parse().unwrap();
        assert_eq!(scratch.matches_double(), 13);
    }
}

mod test_challenge_parsing {
    use advent_of_code_2023::day_4_scratch_cards::{ScratchCard, ScratchPile};
    use std::fs;
    #[test]
    fn scratchpile_challenge_1_parsed_correctly() {
        let example = fs::read_to_string("./data/scratchcards_input_long.txt").unwrap();
        let scratch: ScratchPile = example.parse().unwrap();
        assert_eq!(scratch.matches_double(), 22193);
    }
    #[test]
    fn scratchpile_challenge_2_example_parsed_correctly() {
        let example = fs::read_to_string("./data/scratchcards_input_short.txt").unwrap();
        let scratch: ScratchPile = example.parse().unwrap();
        assert_eq!(scratch.worth(), 30);
    }
    #[test]
    fn scratchpile_challenge_2_long_example_parsed_correctly() {
        let example = fs::read_to_string("./data/scratchcards_input_long.txt").unwrap();
        let scratch: ScratchPile = example.parse().unwrap();
        assert_eq!(scratch.worth(), 5625994);
    }
}
