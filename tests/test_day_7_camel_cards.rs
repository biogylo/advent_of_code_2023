#[cfg(test)]
mod test_challenge_parsing {
    use advent_of_code_2023::day_7_camel_cards::{CamelCard, CamelHand, CamelHandSet};
    use itertools::Itertools;
    use std::fs;

    #[test]
    fn parse_card_and_sort_correctly() {
        let mut card_vec: Vec<CamelCard> = vec![
            CamelCard::from_char('J').unwrap(),
            CamelCard::from_char('3').unwrap(),
            CamelCard::from_char('2').unwrap(),
            CamelCard::from_char('5').unwrap(),
        ];
        card_vec.sort();
        assert!(card_vec[0].character == '2');
        assert!(card_vec[1].character == '3');
        assert!(card_vec[2].character == '5');
        assert!(card_vec[3].character == 'J');
    }

    // #[test]
    // fn parse_card_and_sort_correctly() {}
    #[test]
    fn parse_one_of_each_and_they_sort_correctly() {
        let five_of_a_kind: CamelHand = "AAAAA 1".parse().unwrap();
        let four_of_a_kind: CamelHand = "AA8AA 1".parse().unwrap();
        let full_house: CamelHand = "23332 1".parse().unwrap();
        let full_house_higher: CamelHand = "43334 1".parse().unwrap();
        let three_of_a_kind: CamelHand = "TTT98 1".parse().unwrap();
        let two_pair: CamelHand = "23432 1".parse().unwrap();
        let one_pair: CamelHand = "A23A4 1".parse().unwrap();
        let one_pair_joker: CamelHand = "J3456 1".parse().unwrap();
        let one_pair_lower: CamelHand = "223A4 1".parse().unwrap();
        let high_card: CamelHand = "34567 1".parse().unwrap();

        let vec_cards_unordered = vec![
            high_card.clone(),
            one_pair_lower.clone(),
            one_pair.clone(),
            two_pair.clone(),
            three_of_a_kind.clone(),
            full_house.clone(),
            full_house_higher.clone(),
            four_of_a_kind.clone(),
            five_of_a_kind.clone(),
            one_pair_joker.clone(),
        ];

        let vec_cards_expected_order = vec![
            high_card,
            one_pair_joker,
            one_pair_lower,
            one_pair,
            two_pair,
            three_of_a_kind,
            full_house,
            full_house_higher,
            four_of_a_kind,
            five_of_a_kind,
        ];
        let vec_card_ordered = vec_cards_unordered.into_iter().sorted().collect_vec();
        assert_eq!(vec_cards_expected_order, vec_card_ordered)
    }

    #[test]
    fn short_exercise_works_as_expected() {
        let camelset: CamelHandSet = fs::read_to_string("./data/camel_cards_input_short.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(camelset.total_winnings(), 5905);
    }

    #[test]
    fn big_exercise_works_as_expected() {
        let camelset: CamelHandSet = fs::read_to_string("./data/camel_cards_input_long.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(camelset.total_winnings(), 255632664);
    }
}
