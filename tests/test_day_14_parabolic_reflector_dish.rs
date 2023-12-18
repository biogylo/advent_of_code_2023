#[cfg(test)]
mod tests {
    use advent_of_code_2023::day_14_parabolic_reflector_dish::Platform;
    use std::fs;

    #[test]
    fn p() {
        let platform_str =
            fs::read_to_string("./data/parabolic_reflector_dish_sample.txt").unwrap();
        let platform: Platform = platform_str.parse().unwrap();
        println!("Before tilting:\n{}\n", platform);
        let tilted_platform: Platform = platform.tilted();
        println!("After tilting:\n{}\n", tilted_platform);
        let total_north_load = tilted_platform.total_load();
        assert_eq!(total_north_load, 136)
    }

    #[test]
    fn p2() {
        let platform_str =
            fs::read_to_string("./data/parabolic_reflector_dish_input_long.txt").unwrap();
        let platform: Platform = platform_str.parse().unwrap();
        println!("Before tilting:\n{}\n", platform);
        let tilted_platform: Platform = platform.tilted();
        println!("After tilting:\n{}\n", tilted_platform);
        let total_north_load = tilted_platform.total_load();
        assert_eq!(total_north_load, 105982)
    }
}
