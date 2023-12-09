#[cfg(test)]
mod test_challenge_parsing {
    use advent_of_code_2023::day_6_waitforit::{Document, RaceRecord};
    use std::fs;

    #[test]
    fn parse_document() {
        let document_str = fs::read_to_string("./data/waitforit_input_short.txt").unwrap();
        let doc: Document = document_str.parse().unwrap();
        let race_records = doc.into_vec();
        assert_eq!(
            race_records,
            vec![
                RaceRecord {
                    duration_ms: 7,
                    distance_mm: 9
                },
                RaceRecord {
                    duration_ms: 15,
                    distance_mm: 40
                },
                RaceRecord {
                    duration_ms: 30,
                    distance_mm: 200
                },
            ]
        );
    }

    #[test]
    fn compute_scenarios() {
        assert_eq!(RaceRecord::compute_distance(0, 7), 0);
        assert_eq!(RaceRecord::compute_distance(1, 7), 6);
        assert_eq!(RaceRecord::compute_distance(2, 7), 10);
        assert_eq!(RaceRecord::compute_distance(3, 7), 12);
        assert_eq!(RaceRecord::compute_distance(4, 7), 12);
        assert_eq!(RaceRecord::compute_distance(5, 7), 10);
        assert_eq!(RaceRecord::compute_distance(6, 7), 6);
        assert_eq!(RaceRecord::compute_distance(7, 7), 0);
    }

    #[test]
    fn find_ways_to_beat() {
        let document_str = fs::read_to_string("./data/waitforit_input_short.txt").unwrap();
        let doc: Document = document_str.parse().unwrap();
        let ways_to_beat = doc.ways_to_beat();
        assert_eq!(ways_to_beat, vec![4, 8, 9]);

        let product = doc.product_of_ways_to_beat();
        assert_eq!(product, 288);
    }

    #[test]
    fn find_ways_to_beat_input_mid() {
        let document_str = fs::read_to_string("./data/waitforit_input_mid.txt").unwrap();
        let doc: Document = document_str.parse().unwrap();
        let product = doc.product_of_ways_to_beat();
        assert_eq!(product, 140220);
    }

    #[test]
    fn find_ways_to_beat_input_long() {
        let document_str = fs::read_to_string("./data/waitforit_input_long.txt").unwrap();
        let doc: Document = document_str.parse().unwrap();
        let product = doc.product_of_ways_to_beat();
        assert_eq!(product, 71503);
    }

    #[test]
    fn find_ways_to_beat_input_much_long() {
        let document_str = fs::read_to_string("./data/waitforit_input_much_long.txt").unwrap();
        let doc: Document = document_str.parse().unwrap();
        let product = doc.product_of_ways_to_beat();
        assert_eq!(product, 1);
    }
}
