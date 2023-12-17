#[cfg(test)]
mod tests {
    use advent_of_code_2023::day_12_hot_springs::HotSpringsMemo;
    use std::fs;

    #[test]
    fn small_pattern_works() {
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo.get_arrangements_for_row("? 1", 1).unwrap();
        assert_eq!(arrangements, 1);
        //
    }

    #[test]
    fn small_pattern_2_works() {
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo.get_arrangements_for_row("?? 1", 1).unwrap();
        assert_eq!(arrangements, 2);
        //
    }

    #[test]
    fn small_pattern_3_works() {
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo.get_arrangements_for_row("???? 1,2", 1).unwrap();
        assert_eq!(arrangements, 1);
        //
    }

    #[test]
    fn small_pattern_4_works() {
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo.get_arrangements_for_row("#???#? 1,2", 1).unwrap();
        assert_eq!(arrangements, 2);
        //
    }

    #[test]
    fn small_pattern_5_works() {
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo
            .get_arrangements_for_row("?###???????? 3,2,1", 1)
            .unwrap();
        assert_eq!(arrangements, 10);
        //
    }

    #[test]
    fn test_case_from_challenge_part_1_works() {
        let the_paragraph = fs::read_to_string("./data/hot_springs_input_short.txt").unwrap();
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo
            .get_arrangements_for_paragraph(the_paragraph.as_str(), 1)
            .unwrap();
        assert_eq!(arrangements, 21);
        //
    }

    #[test]
    fn challenge_part_1_full_input_works() {
        let the_paragraph = fs::read_to_string("./data/hot_springs_input_long.txt").unwrap();
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo
            .get_arrangements_for_paragraph(the_paragraph.as_str(), 1)
            .unwrap();
        assert_eq!(arrangements, 7771);
        //
    }

    #[test]
    fn test_case_from_challenge_part_2_works() {
        let the_paragraph = fs::read_to_string("./data/hot_springs_input_short.txt").unwrap();
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo
            .get_arrangements_for_paragraph(the_paragraph.as_str(), 5)
            .unwrap();
        assert_eq!(arrangements, 525152);
        //
    }

    #[test]
    fn challenge_part_2_full_input_works() {
        let the_paragraph = fs::read_to_string("./data/hot_springs_input_long.txt").unwrap();
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo
            .get_arrangements_for_paragraph(the_paragraph.as_str(), 5)
            .unwrap();
        assert_eq!(arrangements, 0);
        //
    }
}
