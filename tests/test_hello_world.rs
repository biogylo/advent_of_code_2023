#[cfg(test)]
mod test_convert {
    use advent_of_code_2023::day_10_pipe_maze::ugly_pipe_maze_to_cute_pipe_maze;
    use std::fs;

    #[test]
    fn convert_puzzle_input_into_easier() {
        let buffer = fs::read_to_string("./data/pipe_maze_easy_loop.txt").unwrap();
        let cute_buffer = fs::read_to_string("./data/pipe_maze_easy_loop_cute.txt").unwrap();
        assert!(
            cute_buffer.len() > 10,
            "The cute buffer probably didn't load correctly"
        );

        let cutiefied_buffer = ugly_pipe_maze_to_cute_pipe_maze(buffer.trim());
        assert_eq!(cutiefied_buffer.trim(), cute_buffer.trim());
    }
}
