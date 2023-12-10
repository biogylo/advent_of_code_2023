use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct History {
    pub values: Vec<isize>,
}

pub fn get_diff_vec(vector: &Vec<isize>) -> Vec<isize> {
    vector
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

pub fn get_diff_decomp(vector: &Vec<isize>) -> Vec<Vec<isize>> {
    // It generates vectors based on differences
    let mut decomp = vec![];
    let mut last_decomp = vector.clone();
    loop {
        if last_decomp.iter().all(|n| *n == 0) {
            return decomp;
        }
        let diff_vec = get_diff_vec(&last_decomp);
        decomp.push(last_decomp);
        last_decomp = diff_vec;
    }
}
impl History {
    pub fn extrapolate(&self) -> isize {
        self.naive_extrapolate()
    }

    pub fn naive_extrapolate(&self) -> isize {
        // It generates vectors based on differences
        let decomp: Vec<Vec<isize>> = get_diff_decomp(&self.values);

        let mut new_last: isize = 0;
        for row in decomp.iter().rev() {
            new_last += row.last().unwrap()
        }
        new_last
    }

    pub fn naive_extrapolate_rev(&self) -> isize {
        // It generates vectors based on differences
        let decomp: Vec<Vec<isize>> = get_diff_decomp(&self.values);

        let mut new_last: isize = 0;
        for row in decomp.iter().rev() {
            new_last = row.first().unwrap() - new_last;
        }
        new_last
    }
}

pub fn parse_report(s: &str) -> Result<Vec<History>, ParseIntError> {
    s.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse())
        .collect()
}
impl FromStr for History {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<isize>, ParseIntError>>()?;
        Ok(History { values })
    }
}
