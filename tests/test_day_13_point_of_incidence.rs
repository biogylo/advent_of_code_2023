#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::day_13_point_of_incidence::summarize_reflections;
    use std::fs;

    #[test]
    fn the_example_from_challenge_part_1_works() {
        let cluster_str = fs::read_to_string("./data/point_of_incidence_samples.txt").unwrap();
        let summary = summarize_reflections(&cluster_str, 0);
        assert_eq!(summary, 405);
    }

    #[test]
    fn challenge_part_1_input_long_works() {
        let cluster_str = fs::read_to_string("./data/point_of_incidence_input_long.txt").unwrap();
        let summary = summarize_reflections(&cluster_str, 0);
        assert_eq!(summary, 35691);
    }

    #[test]
    fn the_example_from_challenge_part_2_works() {
        let cluster_str = fs::read_to_string("./data/point_of_incidence_samples.txt").unwrap();
        let summary = summarize_reflections(&cluster_str, 1);
        assert_eq!(summary, 400);
    }

    #[test]
    fn challenge_part_2_input_long_works() {
        let cluster_str = fs::read_to_string("./data/point_of_incidence_input_long.txt").unwrap();
        let summary = summarize_reflections(&cluster_str, 1);
        assert_eq!(summary, 39037);
    }
}
