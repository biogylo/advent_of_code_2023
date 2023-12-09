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
        // Gets the parabola formula, by taking three points

        // axx + bx + c = y
        // x is hold time
        // y is distance accomplished

        // axx + bx + c - y = 0      -> where x = 0
        // c = y0
        let quadratic = |x: isize| RaceRecord::compute_distance(x, self.duration_ms);
        let c = quadratic(0);
        // axx + bx =  y - c
        // Plug in two numbers to get another system
        let a_plus_b = quadratic(1) - c;
        let a4_plus_b2 = quadratic(2) - c;
        let a = (a4_plus_b2 - 2 * a_plus_b) / 2;
        let b = a_plus_b - a;

        println!("we need to find the first record hold time, add one and find the distance to x_vertex times 2");
        println!("x is hold time, y is distance");
        println!("{}x^2 + {}x + {}", a, b, c);
        // Got all the values! Now need to find the roots, and their distance to the record!
        let sqroot_term = f64::sqrt(((b * b) - (4 * a * c)) as f64);
        let x_root_0 = (-b as f64 + sqroot_term) / (2 * a) as f64;
        let x_root_1 = (-b as f64 - sqroot_term) / (2 * a) as f64;
        println!("Roots {} && {}", x_root_0, x_root_1);
        println!("Record is distance {}, for hold time ", self.distance_mm,);

        // Vertex is x_v =-b/2a
        let h_x_vertex = -b as f64 / (2 * a) as f64;
        let k_y_vertex =
            h_x_vertex * h_x_vertex * (a as f64) + (b as f64) * h_x_vertex + (c as f64);

        let small_inverse = |y| h_x_vertex - (f64::sqrt((y - k_y_vertex) / (a as f64)));
        println!("Vertex is {},{}", h_x_vertex, k_y_vertex);
        println!("First point {}", small_inverse(self.distance_mm as f64));
        let first_x_after_record = (small_inverse(self.distance_mm as f64)).floor() as isize + 1;
        println!("Value from actual: {}", first_x_after_record);

        // Actually, we need to find the middle point, or middle two points, and those should be equidistant to records
        let first_middle_point = self.duration_ms / 2;

        if (self.duration_ms + 1) % 2 == 1 {
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
