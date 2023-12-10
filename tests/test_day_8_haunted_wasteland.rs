#[cfg(test)]
mod test_challenge_parsing {
    use advent_of_code_2023::day_8_haunted_wasteland::{factorize, Direction, GhostMap};
    use itertools::Itertools;
    use std::fs;

    #[test]
    fn parse_ghost_card_correctly() {
        let buffer = fs::read_to_string("./data/haunted_wasteland_input_short.txt").unwrap();
        let ghost_map: GhostMap = buffer.parse().unwrap();
        assert_eq!(
            ghost_map.directions,
            vec![Direction::Right, Direction::Left]
        );

        assert_eq!(
            ghost_map
                .sorted_keys
                .iter()
                .map(|gk| gk.full.clone())
                .collect_vec(),
            vec!["AAA", "BBB", "CCC", "DDD", "EEE", "GGG", "ZZZ",]
        );

        assert_eq!(
            ghost_map
                .node_map
                .iter()
                .map(|(gk1, gk2)| (gk1.index.clone(), gk2.index.clone()))
                .collect_vec(),
            vec![(1, 2), (3, 4), (6, 5), (3, 3), (4, 4), (5, 5), (6, 6)]
        );
    }

    #[test]
    fn ghost_card_takes_two_steps_to_zzz_from_aaa() {
        let buffer = fs::read_to_string("./data/haunted_wasteland_input_short.txt").unwrap();
        let ghost_map: GhostMap = buffer.parse().unwrap();
        assert_eq!(
            ghost_map.walk_from_key("AAA".to_string(), "ZZZ".to_string()),
            2
        );
    }

    #[test]
    fn ghost_card_takes_six_steps_to_zzz_from_aaa() {
        let buffer = fs::read_to_string("./data/haunted_wasteland_input_mini.txt").unwrap();
        let ghost_map: GhostMap = buffer.parse().unwrap();
        assert_eq!(
            ghost_map.walk_from_key("AAA".to_string(), "ZZZ".to_string()),
            6
        );
    }

    #[test]
    fn ghost_card_long() {
        let buffer = fs::read_to_string("./data/haunted_wasteland_input_long.txt").unwrap();
        let ghost_map: GhostMap = buffer.parse().unwrap();
        assert_eq!(
            ghost_map.walk_from_key("AAA".to_string(), "ZZZ".to_string()),
            16579
        );
    }

    #[test]
    fn ghost_card_short_spacetime() {
        let buffer =
            fs::read_to_string("./data/haunted_wasteland_spacetime_input_short.txt").unwrap();
        let ghost_map: GhostMap = buffer.parse().unwrap();
        assert_eq!(ghost_map.walk_across_spacetime_from_ending('A', 'Z'), 6);
    }

    #[test]
    fn ghost_card_long_spacetime() {
        let buffer = fs::read_to_string("./data/haunted_wasteland_input_long.txt").unwrap();
        let ghost_map: GhostMap = buffer.parse().unwrap();
        assert_eq!(
            ghost_map.walk_across_spacetime_from_ending('A', 'Z'),
            12927600769609
        );
    }

    #[test]
    fn factorize_works() {
        assert_eq!(factorize(20), vec![2, 2, 5]);
        assert_eq!(factorize(48), vec![2, 2, 2, 2, 3]);
        assert_eq!(factorize(75), vec![3, 5, 5]);
        assert_eq!(factorize(98), vec![2, 7, 7]);
        assert_eq!(factorize(105), vec![3, 5, 7]);
        assert_eq!(factorize(132), vec![2, 2, 3, 11]);
        assert_eq!(factorize(154), vec![2, 7, 11]);
        assert_eq!(factorize(200), vec![2, 2, 2, 5, 5]);
        assert_eq!(factorize(225), vec![3, 3, 5, 5]);
        assert_eq!(factorize(270), vec![2, 3, 3, 3, 5]);
    }
}
