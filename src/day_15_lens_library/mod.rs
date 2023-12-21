use itertools::Itertools;
use std::str::FromStr;

pub trait LensHashable {
    fn lens_hash(&self) -> u8;
}

impl LensHashable for [u8] {
    fn lens_hash(&self) -> u8 {
        let mut running_hash = 0;
        for &c in self.iter() {
            let intermediate: u16 = ((running_hash as u16) + (c as u16)) * 17_u16;
            running_hash = intermediate as u8; // Unsafe but expected
        }
        running_hash
    }
}

pub struct InitializationSequence {
    steps: Vec<Vec<u8>>,
}

impl InitializationSequence {
    pub fn sum_of_hashes(&self) -> usize {
        self.steps
            .iter()
            .map(|step| step.as_slice().lens_hash() as usize)
            .sum()
    }
}
impl FromStr for InitializationSequence {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .trim()
            .lines()
            .flat_map(|line| line.split(",").map(|step| step.as_bytes().to_vec()))
            .collect_vec();
        Ok(InitializationSequence { steps })
    }
}
