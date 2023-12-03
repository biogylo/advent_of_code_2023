#[cfg(test)]
mod tests {
    use advent_of_code_2023::friend::Friends;
    use advent_of_code_2023::gear_ratios::Schematic;
    use std::fs;
    use std::str::FromStr;

    #[test]
    fn example_gives_adequate_part_numbers_and_the_sum_is_correct() {
        let example = fs::read_to_string("./data/gear_ratios_input_short.txt").unwrap();
        let sch: Schematic = Schematic::from_str(example.as_str()).unwrap();
        assert_eq!(sch.get_part_numbers().iter().sum::<usize>(), 4361)
    }

    #[test]
    fn example_gives_adequate_gears_and_the_ratios_are_correct() {
        let example = fs::read_to_string("./data/gear_ratios_input_short.txt").unwrap();
        let sch: Schematic = Schematic::from_str(example.as_str()).unwrap();
        let gears = sch.get_gear_ratios();
        assert_eq!(gears, vec![16345, 451490]);
        assert_eq!(gears.iter().sum::<usize>(), 467835);
    }

    #[test]
    fn example_big_gives_adequate_part_numbers_and_the_sum_is_correct() {
        let example = fs::read_to_string("./data/gear_ratios_input_long.txt").unwrap();
        let sch: Schematic = Schematic::from_str(example.as_str()).unwrap();
        assert_ne!(sch.get_part_numbers().iter().sum::<usize>(), 329623); //329623 is not it
        assert_eq!(sch.get_part_numbers().iter().sum::<usize>(), 537732); //329623 is not it
    }

    #[test]
    fn example_big_gives_adequate_part_numbers_and_the_gears_is_correct() {
        let example = fs::read_to_string("./data/gear_ratios_input_long.txt").unwrap();
        let sch: Schematic = Schematic::from_str(example.as_str()).unwrap();
        let gears = sch.get_gear_ratios();
        assert_eq!(gears.iter().sum::<usize>(), 84883664);
    }
}
