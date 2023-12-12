#[cfg(test)]
mod test_convert {
    use advent_of_code_2023::day_11_cosmic_expansion::{Observation, SpaceImage};
    use advent_of_code_2023::day_8_haunted_wasteland::Direction;
    use std::collections::HashSet;
    use std::fs;

    #[test]
    fn cosmic_expansion_rows_and_columns_get_increased() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_short.txt").unwrap();
        let cute_buffer =
            fs::read_to_string("./data/cosmic_expansion_input_short_expanded.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        let expanded_image = space_image.expand();
        assert_eq!(format!("{}", expanded_image), format!("{}", cute_buffer));
    }

    #[test]
    fn cosmic_expansion_rows_and_columns_get_increased_distance_is_correct() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_short.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        let expanded_image = space_image.expand();
        assert_eq!(
            expanded_image
                .find_smallest_distances()
                .iter()
                .sum::<usize>(),
            374
        );
    }
    #[test]
    fn cosmic_expansion_rows_and_columns_for_long_input_get_increased_distance_is_correct() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_long.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        let expanded_image = space_image.expand();
        assert_eq!(
            expanded_image
                .find_smallest_distances()
                .iter()
                .sum::<usize>(),
            9742154
        );
    }

    #[test]
    fn cosmic_expansion_finds_correct_galaxy_locations() {
        let buffer = fs::read_to_string("./data/cosmic_expansion_input_short.txt").unwrap();
        let space_image: SpaceImage = buffer.parse().unwrap();
        let expanded_image = space_image.expand();

        assert_eq!(
            expanded_image.find_galaxies(),
            HashSet::from([
                (3, 0),
                (8, 4),
                (7, 1),
                (5, 0),
                (5, 8),
                (9, 1),
                (5, 3),
                (5, 5),
                (0, 3),
                (10, 9),
                (0, 1),
                (4, 3),
                (11, 6),
                (9, 11),
                (1, 4),
                (6, 12),
                (1, 7),
                (8, 12),
                (2, 12),
                (4, 2),
                (4, 10),
                (1, 2),
                (0, 9),
                (4, 6),
                (7, 5),
                (5, 4),
                (7, 0),
                (9, 2),
                (9, 9),
                (9, 3),
                (7, 3),
                (1, 3),
                (5, 10),
                (0, 11),
                (8, 11),
                (2, 0),
                (6, 2),
                (8, 6),
                (11, 8),
                (5, 9),
                (1, 10),
                (4, 4),
                (4, 11),
                (6, 0),
                (6, 9),
                (9, 5),
                (3, 10),
                (8, 10),
                (11, 1),
                (0, 12),
                (9, 8),
                (1, 9),
                (4, 8),
                (7, 4),
                (7, 7),
                (11, 2),
                (11, 9),
                (10, 0),
                (9, 10),
                (1, 12),
                (6, 1),
                (1, 5),
                (3, 4),
                (11, 10),
                (0, 8),
                (3, 7),
                (6, 8),
                (6, 10),
                (7, 9),
                (6, 6),
                (9, 6),
                (1, 6),
                (4, 12),
                (11, 12),
                (4, 7),
                (5, 6),
                (6, 11),
                (9, 7),
                (11, 0),
                (8, 1),
                (1, 0),
                (2, 1),
                (3, 11),
                (5, 7),
                (2, 9),
                (9, 12),
                (6, 4),
                (9, 0),
                (11, 7),
                (5, 1),
                (7, 12),
                (0, 6),
                (11, 11),
                (0, 7),
                (5, 11),
                (1, 8),
                (6, 7),
                (10, 10),
                (0, 0),
                (0, 2),
                (7, 8),
                (10, 5),
                (11, 3),
                (10, 11),
                (8, 7),
                (10, 6),
                (7, 11),
                (8, 3),
                (8, 8),
                (2, 7),
                (8, 9),
                (2, 5),
                (0, 5),
                (3, 12),
                (2, 6),
                (5, 12),
                (3, 5),
                (2, 3),
                (4, 9),
                (6, 5),
                (3, 2),
                (8, 0),
                (7, 2),
                (6, 3),
                (2, 8),
                (3, 1),
                (1, 11),
                (3, 9),
                (0, 10),
                (3, 8),
                (1, 1),
                (4, 0),
                (10, 12),
                (5, 2),
                (7, 6),
                (7, 10),
                (0, 4),
                (2, 10),
                (2, 4),
                (8, 5),
                (2, 11),
                (10, 1),
                (10, 8),
                (11, 4),
                (9, 4),
                (2, 2),
                (11, 5),
                (8, 2),
                (4, 1),
                (10, 3),
                (3, 6),
                (10, 4),
                (10, 7),
                (4, 5),
                (10, 2),
                (3, 3)
            ])
        );
    }
}
