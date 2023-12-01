#[cfg(test)]
mod tests {
    use advent_of_code_2023::friend::Friends;

    #[test]
    fn hector_string_is_correct() {
        assert_eq!(Friends::Hector.to_string(), String::from("Hector"));
    }

    #[test]
    fn juan_string_is_correct() {
        assert_eq!(Friends::Juan.to_string(), String::from("Juan"));
    }

    #[test]
    fn pancho_string_is_correct() {
        assert_eq!(Friends::Pancho.to_string(), String::from("Pancho"));
    }

    #[test]
    fn sebastian_string_is_correct() {
        assert_eq!(Friends::Sebastian.to_string(), String::from("Sebastian"));
    }
}
