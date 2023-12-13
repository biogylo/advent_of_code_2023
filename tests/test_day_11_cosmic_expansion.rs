#[cfg(test)]
mod test_convert {
    use advent_of_code_2023::day_11_cosmic_expansion::SpaceImage;
    use std::fs;

    #[test]
    fn cosmic_expansion_rows_and_columns_get_increased() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_short.txt").unwrap();
        let cute_buffer =
            fs::read_to_string("./data/cosmic_expansion_input_short_expanded.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        let expanded_image = space_image.age_by(1);
        assert_eq!(format!("{}", expanded_image), format!("{}", cute_buffer));
    }

    #[test]
    fn cosmic_expansion_rows_and_columns_get_increased_distance_is_correct() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_short.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        assert_eq!(
            space_image.find_smallest_distances(2).iter().sum::<usize>(),
            374
        );
    }

    #[test]
    fn cosmic_expansion_by_100_rows_and_columns_get_increased_distance_is_correct() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_short.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        assert_eq!(
            space_image
                .find_smallest_distances(10)
                .iter()
                .sum::<usize>(),
            1030
        );
    }
    #[test]
    fn cosmic_expansion_rows_and_columns_for_long_input_get_increased_distance_is_correct() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_long.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        assert_eq!(
            space_image.find_smallest_distances(2).iter().sum::<usize>(),
            9742154
        );
    }

    #[test]
    fn huge_cosmic_expansion_rows_and_columns_for_long_input_get_increased_distance_is_correct() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_long.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        assert_eq!(
            space_image
                .find_smallest_distances(1000000)
                .iter()
                .sum::<usize>(),
            411142919886
        );
    }
}
