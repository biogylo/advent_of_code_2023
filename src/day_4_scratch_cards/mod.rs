use itertools::Itertools;
use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;

pub struct ScratchCard {
    pub id: usize,
    pub winning_numbers: HashSet<usize>,
    pub actual_numbers: Vec<usize>,
}

pub struct ScratchPile(Vec<ScratchCard>);

impl FromStr for ScratchCard {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let starter_string = s.trim().replace("Card ", "");
        let (id_token, numbers_token) = starter_string
            .split_once(":")
            .ok_or("Unable to parse since the colon is missing".to_string())?;

        let id: usize = id_token
            .trim()
            .parse()
            .or(Err("Unable to parse since there is no card ID"))?;
        let (winning_numbers_token, actual_numbers_token) = numbers_token
            .trim()
            .split_once("|")
            .ok_or("There was no delimiter in the numbers token, unable to parse!")?;

        let winning_numbers = Self::parse_list_of_numbers(winning_numbers_token)?
            .into_iter()
            .collect();
        let actual_numbers = Self::parse_list_of_numbers(actual_numbers_token)?;
        Ok(ScratchCard {
            id,
            winning_numbers,
            actual_numbers,
        })
    }
}

impl ScratchCard {
    pub fn has_matches(&self) -> bool {
        self.match_count() > 0
    }
    pub fn match_iter(&self) -> impl Iterator<Item = &usize> {
        self.actual_numbers
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
    }
    pub fn match_count(&self) -> usize {
        self.match_iter().count()
    }
    pub fn matches(&self) -> Vec<usize> {
        self.match_iter().into_iter().cloned().collect_vec()
    }
    fn parse_list_of_numbers(actual_numbers_token: &str) -> Result<Vec<usize>, String> {
        if actual_numbers_token
            .chars()
            .filter(|c| !(c.is_digit(10) || *c == ' '))
            .count()
            != 0
        {
            return Err("Unable to parse list of numbers".to_string());
        }

        Ok(actual_numbers_token
            .trim()
            .split(" ")
            .into_iter()
            .map(|num| num.trim().parse().ok())
            .flatten()
            .collect::<Vec<usize>>())
    }
}

impl ScratchPile {
    pub fn total_matches(&self) -> usize {
        self.0.iter().map(|card| card.match_count()).sum()
    }

    pub fn matches_double(&self) -> usize {
        self.0
            .iter()
            .filter(|card| card.has_matches())
            .map(|card| 2_usize.pow((card.match_count() - 1) as u32))
            .sum()
    }
    pub fn worth(&self) -> usize {
        let card_count = self.0.len();

        let mut instances: Vec<usize> = vec![1; card_count];
        let points_per_card: Vec<usize> = self
            .0
            .iter()
            .enumerate()
            .map(|(i, card)| min(card_count - i - 1, card.match_count()))
            // .map(|(i, j)| i + j)
            .collect_vec();

        points_per_card
            .iter()
            .enumerate()
            .map(|(i, &points)| (0..points).for_each(|j| instances[i + j + 1] += instances[i]))
            .collect_vec();

        instances.into_iter().sum()
    }
}

impl FromStr for ScratchPile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ScratchPile {
            0: s.lines()
                .map(|line| line.parse().ok())
                .collect::<Option<Vec<ScratchCard>>>()
                .ok_or("Unable to parse all lines of scratchpile")?,
        })
    }
}
