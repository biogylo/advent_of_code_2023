#[cfg(test)]
mod tests {
    use advent_of_code_2023::day_12_hot_springs::HotSpringsMemo;

    #[test]
    fn small_pattern_works() {
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo.get_arrangements_for_row("? 1").unwrap();
        assert_eq!(arrangements, 1);
        //
    }

    #[test]
    fn small_pattern_2_works() {
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo.get_arrangements_for_row("?? 1").unwrap();
        assert_eq!(arrangements, 2);
        //
    }

    #[test]
    fn small_pattern_3_works() {
        let mut memo = HotSpringsMemo::new();
        let arrangements = memo.get_arrangements_for_row("???? 1,2").unwrap();
        assert_eq!(arrangements, 1);
        //
    }
}
