#[cfg(test)]
mod tests {
    use advent_of_code_2023::day_2_cube_conundrum::{Conundrum, CubeColor, CubeCounts, Game};
    use std::fs;
    use std::str::FromStr;
    #[test]
    fn parse_turn() {
        let parsed_turn = CubeCounts::from_turn_str(" 3 blue, 4 red").unwrap();

        let desired_turn = CubeCounts::from([
            (CubeColor::Red, 4),
            (CubeColor::Green, 0),
            (CubeColor::Blue, 3),
        ]);

        assert_eq!(desired_turn, parsed_turn);
    }
    #[test]
    fn parse_turn_ugly_case() {
        let parsed_turn = CubeCounts::from_turn_str(" 50 blue, 5   green,   4 red   ").unwrap();

        let desired_turn = CubeCounts::from([
            (CubeColor::Red, 4),
            (CubeColor::Green, 5),
            (CubeColor::Blue, 50),
        ]);

        assert_eq!(desired_turn, parsed_turn);
    }

    #[test]
    fn cube_counts_power_implemented() {
        let counts = CubeCounts::from([
            (CubeColor::Red, 4),
            (CubeColor::Green, 5),
            (CubeColor::Blue, 50),
        ]);

        assert_eq!(counts.power(), 4 * 5 * 50);
    }
    #[test]
    fn parse_game_1() {
        let parsed_game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse()
            .unwrap();
        let desired_game = Game {
            id: 1,
            turns: vec![
                CubeCounts::from([(CubeColor::Red, 4), (CubeColor::Blue, 3)]),
                CubeCounts::from([
                    (CubeColor::Red, 1),
                    (CubeColor::Green, 2),
                    (CubeColor::Blue, 6),
                ]),
                CubeCounts::from([(CubeColor::Green, 2)]),
            ],
        };

        assert_eq!(desired_game, parsed_game);
    }

    #[test]
    fn parse_game_3() {
        let parsed_game = Game::from_str(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        )
        .unwrap();
        let desired_game = Game {
            id: 3,
            turns: vec![
                CubeCounts::from([
                    (CubeColor::Green, 8),
                    (CubeColor::Red, 20),
                    (CubeColor::Blue, 6),
                ]),
                CubeCounts::from([
                    (CubeColor::Red, 4),
                    (CubeColor::Green, 13),
                    (CubeColor::Blue, 5),
                ]),
                CubeCounts::from([(CubeColor::Red, 1), (CubeColor::Green, 5)]),
            ],
        };

        assert_eq!(desired_game, parsed_game);
    }

    #[test]
    fn bag_with_less_greens_cant_do_game_with_more() {
        let the_game = Game {
            id: 3,
            turns: vec![
                CubeCounts::from([
                    (CubeColor::Green, 8),
                    (CubeColor::Red, 20),
                    (CubeColor::Blue, 6),
                ]),
                CubeCounts::from([
                    (CubeColor::Red, 4),
                    (CubeColor::Green, 13),
                    (CubeColor::Blue, 5),
                ]),
                CubeCounts::from([(CubeColor::Red, 1), (CubeColor::Green, 5)]),
            ],
        };
        let bag: CubeCounts = CubeCounts::from([
            (CubeColor::Green, 2),
            (CubeColor::Red, 999),
            (CubeColor::Blue, 999),
        ]);
        assert!(!the_game.is_possible(&bag));
    }

    #[test]
    fn big_bag_can_do_game() {
        let the_game = Game {
            id: 3,
            turns: vec![
                CubeCounts::from([
                    (CubeColor::Green, 8),
                    (CubeColor::Red, 20),
                    (CubeColor::Blue, 6),
                ]),
                CubeCounts::from([
                    (CubeColor::Red, 4),
                    (CubeColor::Green, 13),
                    (CubeColor::Blue, 5),
                ]),
                CubeCounts::from([(CubeColor::Red, 1), (CubeColor::Green, 5)]),
            ],
        };
        let bag: CubeCounts = CubeCounts::from([
            (CubeColor::Green, 999),
            (CubeColor::Red, 999),
            (CubeColor::Blue, 999),
        ]);
        assert!(the_game.is_possible(&bag));
    }

    #[test]
    fn fewest_possible_cubes_to_do_game() {
        let the_game = Game {
            id: 12,
            turns: vec![
                CubeCounts::from([
                    (CubeColor::Red, 20),
                    (CubeColor::Green, 8),
                    (CubeColor::Blue, 6),
                ]),
                CubeCounts::from([
                    (CubeColor::Red, 4),
                    (CubeColor::Green, 13),
                    (CubeColor::Blue, 5),
                ]),
                CubeCounts::from([
                    (CubeColor::Red, 10),
                    (CubeColor::Green, 10),
                    (CubeColor::Blue, 30),
                ]),
            ],
        };
        let expected_bag: CubeCounts = CubeCounts::from([
            (CubeColor::Green, 13),
            (CubeColor::Red, 20),
            (CubeColor::Blue, 30),
        ]);
        assert_eq!(the_game.smallest_bag(), expected_bag);
    }

    #[test]
    fn two_game_conundrum_gets_parsed_correctly() {
        let parsed_conundrum:Conundrum = "\n Game 4: 1 green,  3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n".parse().unwrap();

        let expected_conundrum = Conundrum {
            0: vec![
                Game {
                    id: 4,
                    turns: vec![
                        CubeCounts::from([
                            (CubeColor::Red, 3),
                            (CubeColor::Green, 1),
                            (CubeColor::Blue, 6),
                        ]),
                        CubeCounts::from([(CubeColor::Red, 6), (CubeColor::Green, 3)]),
                        CubeCounts::from([
                            (CubeColor::Red, 14),
                            (CubeColor::Green, 3),
                            (CubeColor::Blue, 15),
                        ]),
                    ],
                },
                Game {
                    id: 5,
                    turns: vec![
                        CubeCounts::from([
                            (CubeColor::Red, 6),
                            (CubeColor::Green, 3),
                            (CubeColor::Blue, 1),
                        ]),
                        CubeCounts::from([
                            (CubeColor::Red, 1),
                            (CubeColor::Green, 2),
                            (CubeColor::Blue, 2),
                        ]),
                    ],
                },
            ],
        };
        assert_eq!(expected_conundrum, parsed_conundrum);
    }

    #[test]
    fn possible_games_from_challenge_example_are_correct() {
        let conundrum: Conundrum = fs::read_to_string("./data/cube_conundrum_input_short.txt")
            .unwrap()
            .parse()
            .unwrap();
        let bag: CubeCounts = CubeCounts::from([
            (CubeColor::Red, 12),
            (CubeColor::Green, 13),
            (CubeColor::Blue, 14),
        ]);
        assert_eq!(conundrum.possible_games(&bag), vec![1, 2, 5]);
        assert_eq!(conundrum.sum_of_possible_game_ids(&bag), 8);
    }

    #[test]
    fn possible_games_from_challenge_part1_are_correct() {
        let conundrum: Conundrum = fs::read_to_string("./data/cube_conundrum_input_mid.txt")
            .unwrap()
            .parse()
            .unwrap();
        let bag: CubeCounts = CubeCounts::from([
            (CubeColor::Red, 12),
            (CubeColor::Green, 13),
            (CubeColor::Blue, 14),
        ]);
        assert_eq!(conundrum.sum_of_possible_game_ids(&bag), 2528);
    }

    #[test]
    fn powers_of_each_games_minimum_bag_added_up_are_correct_for_the_short_example() {
        let conundrum: Conundrum = fs::read_to_string("./data/cube_conundrum_input_short.txt")
            .unwrap()
            .parse()
            .unwrap();

        let powers: Vec<usize> = conundrum.powers();
        assert_eq!(powers, vec![48, 12, 1560, 630, 36]);
        let sum: usize = powers.iter().sum();
        assert_eq!(sum, 2286);
    }

    #[test]
    fn powers_of_each_games_minimum_bag_added_up_are_correct_for_the_mid_example() {
        let conundrum: Conundrum = fs::read_to_string("./data/cube_conundrum_input_mid.txt")
            .unwrap()
            .parse()
            .unwrap();

        let sum: usize = conundrum.powers().iter().sum();
        assert_eq!(sum, 67363);
    }
}
