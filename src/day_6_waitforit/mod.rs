use itertools::Itertools;
use std::f64;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RaceRecord {
    pub duration_ms: isize,
    pub distance_mm: isize,
}
impl RaceRecord {
    pub fn compute(
        hold_time_ms: isize,
        race_duration_ms: isize,
        starting_speed_mms: isize,
        acceleration_mmss: isize,
    ) -> RaceRecord {
        let total_speed: isize = (acceleration_mmss * hold_time_ms) + starting_speed_mms;
        let total_release_time: isize = race_duration_ms.clone() - total_speed.clone();
        let total_distance: isize = total_speed.clone() * total_release_time.clone();
        return RaceRecord {
            duration_ms: race_duration_ms.clone(),
            distance_mm: total_distance.clone(),
        };
    }

    pub fn compute_distance(hold_time_ms: isize, duration_ms: isize) -> isize {
        let total_release_time: isize = duration_ms - hold_time_ms;
        let total_distance: isize = hold_time_ms * total_release_time.clone();
        total_distance
    }

    pub fn ways_to_beat(&self) -> isize {
        // The distance is actually a quadratic formula, where x is the hold time
        let quadratic = |x: isize| RaceRecord::compute_distance(x, self.duration_ms);

        // ax^2 + bx + c = y
        // x is hold time
        // y is distance accomplished

        // Step 1: Plug in 0
        // a^2 + bx + c - y = 0    -> where x = 0
        // c = y0
        let c = quadratic(0) as f64;

        // Step 2: Plug in two numbers to get another system
        let _1a_plus_1b = quadratic(1) as f64 - c;
        let _4a_plus_2b = quadratic(2) as f64 - c;
        let a = (_4a_plus_2b - 2.0 * _1a_plus_1b) / 2.0;
        let b = _1a_plus_1b - a;

        // Step 2: Get the vertex
        // h =-b/2a   -> x value of the vertex
        // k = quadratic(h)  -> y value of the vertex
        let h_x_vertex = -b / (2.0 * a);
        let k_y_vertex = h_x_vertex * h_x_vertex * a + b * h_x_vertex + c;

        let inverse_of_quadratic = |y| h_x_vertex - f64::sqrt((y - k_y_vertex) / a);

        // Get the first value after the record.
        // We solve for x that corresponds to the record, we truncate, and add 1
        // to get the next discrete x value that is bigger
        let first_x_after_record = inverse_of_quadratic(self.distance_mm as f64) as isize + 1;

        // Depending on whether there is one middle point or two, we apply math
        let first_middle_point = self.duration_ms / 2;
        let even_number_of_discrete_choices = (self.duration_ms + 1) % 2 == 1;
        if even_number_of_discrete_choices {
            // One single middle point
            ((first_middle_point - first_x_after_record) * 2) + 1
        } else {
            // Two middle points
            (first_middle_point - first_x_after_record + 1) * 2
        }
    }
}

pub struct Document {
    race_records: Vec<RaceRecord>,
}

impl Document {
    pub fn into_vec(self) -> Vec<RaceRecord> {
        self.race_records
    }

    pub fn ways_to_beat(&self) -> Vec<isize> {
        self.race_records
            .clone()
            .into_iter()
            .map(|record| record.ways_to_beat())
            .collect_vec()
    }

    pub fn product_of_ways_to_beat(&self) -> isize {
        self.ways_to_beat().iter().cloned().product()
    }
}

fn parse_number_list<T: FromStr>(tokens: &str) -> Result<Vec<T>, String> {
    tokens
        .trim()
        .split_whitespace()
        .map(|token| token.parse::<T>().ok())
        .collect::<Option<Vec<T>>>()
        .ok_or("Unable to parse number list".to_string())
}
impl FromStr for Document {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.trim().lines();

        let [time_line, distance_line] = line_iter
            .next_chunk()
            .map_err(|_| "Unable to find time and distance lines".to_string())?;

        let [_, time_number_token] = time_line
            .split(":")
            .next_chunk()
            .map_err(|_| "Unable to split time line".to_string())?;

        let [_, distance_number_token] = distance_line
            .split(":")
            .next_chunk()
            .map_err(|_| "Unable to split distance line".to_string())?;

        let time_millis_list: Vec<isize> = parse_number_list(time_number_token)?;
        let distance_mm_list: Vec<isize> = parse_number_list(distance_number_token)?;

        if time_millis_list.len() != distance_mm_list.len() {
            Err("The time and distance lists don't have the same length!!".to_string())
        } else {
            let time_and_distance = time_millis_list
                .into_iter()
                .zip(distance_mm_list.into_iter());
            let race_records = time_and_distance
                .map(|(duration_ms, distance_mm)| RaceRecord {
                    duration_ms: duration_ms,
                    distance_mm: distance_mm,
                })
                .collect_vec();
            Ok(Document { race_records })
        }
    }
}
