#[cfg(test)]
mod tests {
    use advent_of_code_2023::day_15_lens_library::{InitializationSequence, LensHashable};
    use std::fs;

    #[test]
    fn test_hash_function_simple() {
        let sample_str: &str = "HASH";
        let ascii_str: &[u8] = sample_str.as_bytes();
        assert_eq!(ascii_str.lens_hash(), 52);
    }

    #[test]
    fn test_hash_function_on_initialization_sequence() {
        let sample_str: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let sequence: InitializationSequence = sample_str.parse().unwrap();
        assert_eq!(sequence.sum_of_hashes(), 1320);
    }

    #[test]
    fn test_challenge_part_one_initialization_sequence() {
        let sample_str = fs::read_to_string("./data/lens_library_input_long.txt").unwrap();
        let sequence: InitializationSequence = sample_str.parse().unwrap();
        assert_eq!(sequence.sum_of_hashes(), 508498);
    }
}
