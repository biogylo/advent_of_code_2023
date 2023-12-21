#[cfg(test)]
mod tests {
    use advent_of_code_2023::day_18_lavaduct_lagoon::DigPlan;
    use std::fs;

    #[test]
    fn test_sample_part_1_works() {
        let sample = fs::read_to_string("./data/lavaduct_lagoon_input_short.txt").unwrap();
        let dig_plan: DigPlan = sample.parse().unwrap();
        let outlined_trench = dig_plan.apply(false);
        println!("Trench outline:\n{:}", outlined_trench);

        assert_eq!(outlined_trench.trench_area(), 38);

        let culled_trench = outlined_trench.culled();
        println!("\nCulled trench:\n{:}", culled_trench);
        assert_eq!(culled_trench.trench_area(), 62);
    }

    #[test]
    fn test_challenge_part_1_works() {
        let sample = fs::read_to_string("./data/lavaduct_lagoon_input_long.txt").unwrap();
        let dig_plan: DigPlan = sample.parse().unwrap();
        let outlined_trench = dig_plan.apply(true);
        println!("Trench outline:\n{:}", outlined_trench);

        assert_eq!(outlined_trench.trench_area(), 3376);

        let culled_trench = outlined_trench.culled(true);
        println!("\nCulled trench:\n{:}", culled_trench);
        assert_eq!(culled_trench.trench_area(), 38952);
    }
}
