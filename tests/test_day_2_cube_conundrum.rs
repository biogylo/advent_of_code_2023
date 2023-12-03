#[cfg(test)]
mod tests {
    use advent_of_code_2023::cube_conundrum::{Bag, CubeColor, Turn};
    use std::collections::HashMap;
    use std::str::FromStr;
    #[test]
    fn parse_turn() {
        let parsed_turn = Turn::from_str(" 3 blue, 4 red").unwrap();

        let desired_turn = Turn {
            red_balls_shown: 4,
            green_balls_shown: 0,
            blue_balls_shown: 3,
        };

        assert_eq!(desired_turn, parsed_turn);
    }
    #[test]
    fn parse_turn_2() {
        let parsed_turn = Turn::from_str(" 50 blue, 5   green,   4 red   ").unwrap();

        let desired_turn = Turn {
            red_balls_shown: 4,
            green_balls_shown: 5,
            blue_balls_shown: 50,
        };

        assert_eq!(desired_turn, parsed_turn);
    }
}
