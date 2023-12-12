#[cfg(test)]
mod test_convert {
    use advent_of_code_2023::day_10_pipe_maze::PipeMaze;
    use std::fs;

    #[test]
    fn pipe_maze_gets_parsed_correctly() {
        let buffer = fs::read_to_string("./data/pipe_maze_easy_loop.txt").unwrap();
        let cute_buffer = fs::read_to_string("./data/pipe_maze_easy_loop_cute.txt").unwrap();
        let pipe_maze: PipeMaze = buffer.parse().unwrap();
        assert_eq!(format!("{}", pipe_maze), format!("{}", cute_buffer.trim()));
    }

    #[test]
    fn pipe_maze_finds_start_and_infers_the_right_pipe_and_loop_length() {
        let buffer = fs::read_to_string("./data/pipe_maze_easy_loop_with_s.txt").unwrap();
        let pipe_maze: PipeMaze = buffer.trim().parse().unwrap();
        println!("{}", pipe_maze);
        assert_eq!(pipe_maze.start.expect("Has to have found one!"), (2, 1));

        // Also loops
        let length = pipe_maze.loop_length().unwrap();
        let distance_to_furthest = pipe_maze.farthest_point_distance().unwrap();
        assert_eq!(length, 7);
        assert_eq!(distance_to_furthest, 4)
    }

    #[test]
    fn pipe_maze_finds_right_furthest_point() {
        let buffer = fs::read_to_string("./data/pipe_maze_easy_loop_2_with_s.txt").unwrap();
        let pipe_maze: PipeMaze = buffer.trim().parse().unwrap();
        let distance_to_furthest = pipe_maze.farthest_point_distance().unwrap();
        assert_eq!(distance_to_furthest, 8)
    }

    #[test]
    fn pipe_maze_finds_right_furthest_point_using_the_challenge_input() {
        let buffer = fs::read_to_string("./data/pipe_maze_input_long.txt").unwrap();
        let pipe_maze: PipeMaze = buffer.trim().parse().unwrap();
        let distance_to_furthest = pipe_maze.farthest_point_distance().unwrap();
        assert_eq!(distance_to_furthest, 6800)
    }
}
#[cfg(test)]
mod test_part_2 {
    use advent_of_code_2023::day_10_pipe_maze::PipeMaze;
    use std::fs;

    #[test]
    fn area_inside_small_loop() {
        let buffer = fs::read_to_string("./data/pipe_maze_area_loop_easy.txt").unwrap();
        let pipe_maze: PipeMaze = buffer.parse().unwrap();
        println!("{}", pipe_maze);
        let area = pipe_maze.count_area_inside_loop();
        assert_eq!(area, 4);
    }

    #[test]
    fn area_inside_huge_loop() {
        let buffer = fs::read_to_string("./data/pipe_maze_input_long.txt").unwrap();
        let pipe_maze: PipeMaze = buffer.parse().unwrap();
        println!("{}", pipe_maze);
        let area = pipe_maze.count_area_inside_loop();
        assert_eq!(area, 483);
    }
    #[test]
    fn area_inside_another_loop() {
        let buffer = fs::read_to_string("./data/pipe_maze_another_input.txt").unwrap();
        let pipe_maze: PipeMaze = buffer.parse().unwrap();
        println!("{}", pipe_maze);
        let area = pipe_maze.count_area_inside_loop();
        assert_eq!(area, 10);
    }
}
