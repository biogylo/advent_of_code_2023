#[cfg(test)]
mod test_challenge_parsing {
    use advent_of_code_2023::day_5_fertilizer::{Almanac, MapRange, MapRangeList};
    use itertools::Itertools;
    use std::fs;

    #[test]
    fn parse_single_map() {
        let map_str = "50 98 2";
        let map: MapRange = map_str.parse().unwrap();
        assert!(map.destination.clone().eq(50..=51));
        assert!(map.source.clone().eq(98..=99));
    }
    #[test]
    fn parse_pair_map() {
        let map_str = " 50  98  2 \n 52  50  48 ";
        let map_list: MapRangeList = map_str.parse().unwrap();
        let maps = map_list.into_vec();
        println!("Maps vec {:?}", maps);
        assert!(maps[0].destination.clone().eq(52..52 + 48));
        assert!(maps[0].source.clone().eq(50..50 + 48));
        assert!(maps[1].destination.clone().eq(50..50 + 2));
        assert!(maps[1].source.clone().eq(98..98 + 2));
    }

    #[test]
    fn parse_almanac_indexes_correctly() {
        let almanac_str = fs::read_to_string("./data/fertilizer_almanac_input_short.txt").unwrap();
        let almanac: Almanac = almanac_str.parse().unwrap();
        let locations = almanac.get_seeds_as_individual_ranges();
        let locations_vec = locations.iter().cloned().collect_vec();
        assert_eq!(
            locations_vec,
            vec![13_isize..15_isize, 55_isize..56_isize, 79_isize..80_isize,]
        );
        println!("Passed assertion 1");
        let closest = almanac.get_lowest_individual_seed_location();
        assert_eq!(closest, 35_isize);
    }

    #[test]
    fn parse_almanac_indexes_correctly2() {
        let almanac_str = fs::read_to_string("./data/fertilizer_almanac_input_long.txt").unwrap();
        let almanac: Almanac = almanac_str.parse().unwrap();
        let closest = almanac.get_lowest_individual_seed_location();
        assert_eq!(closest, 84470622_isize);
    }

    #[test]
    fn almanac_gets_location_right_with_seed_ranges() {
        let almanac_str = fs::read_to_string("./data/fertilizer_almanac_input_short.txt").unwrap();
        let almanac: Almanac = almanac_str.parse().unwrap();
        let closest = almanac.get_lowest_seed_ranges_locations();
        assert_eq!(closest, 46);
    }

    #[test]
    fn almanac_gets_location_right_with_seed_ranges_long() {
        let almanac_str = fs::read_to_string("./data/fertilizer_almanac_input_long.txt").unwrap();
        let almanac: Almanac = almanac_str.parse().unwrap();
        let closest = almanac.get_lowest_seed_ranges_locations();
        assert_eq!(closest, 26714516);
    }
}
