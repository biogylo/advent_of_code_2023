use crate::day_3_gear_ratios;
use crate::day_3_gear_ratios::{CharInfo, PartNumber, Schematic, Symbol};
use itertools::Itertools;
use std::ops::RangeInclusive;
use std::str::FromStr;

impl Symbol {
    fn from(char_info: CharInfo) -> Option<Symbol> {
        if day_3_gear_ratios::is_symbol(&char_info.character) {
            return Some(Symbol { 0: char_info });
        }
        return None;
    }

    fn char(&self) -> &char {
        &self.0.character
    }
    fn row_adjacent_index_iter(&self) -> RangeInclusive<i32> {
        (self.0.row as i32 - 1..=self.0.row as i32 + 1).into_iter()
    }

    fn column_adjacent_index_iter(&self) -> RangeInclusive<i32> {
        (self.0.column as i32 - 1..=self.0.column as i32 + 1).into_iter()
    }
}

impl Schematic {
    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.chars.get(y)?.get(x)
    }

    pub fn get_number_row_boundaries(&self, row: usize, column: usize) -> Option<PartNumber> {
        let the_row_chars = self.chars.get(row)?;

        if !the_row_chars.get(column)?.is_digit(10) {
            return None;
        }

        // Find rightmost digit
        let rightmost = (column..the_row_chars.len())
            .take_while(|&i| the_row_chars[i].is_digit(10))
            .last()
            .unwrap();

        // Find leftmost digit
        let leftmost = (0..=column)
            .rev()
            .take_while(|&i| the_row_chars[i].is_digit(10))
            .last()
            .unwrap();

        let number: usize = the_row_chars[leftmost..=rightmost]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        Some(PartNumber {
            number,
            row,
            column_start: leftmost,
            column_end: rightmost,
        })
    }
    pub fn get_numbers_adjacent_to_symbol(&self, symbol: &Symbol) -> Vec<PartNumber> {
        let row_iter = symbol.row_adjacent_index_iter().into_iter();
        let column_iter = symbol.column_adjacent_index_iter().into_iter();
        row_iter
            .cartesian_product(column_iter)
            .map(|(row, column)| self.get_number_row_boundaries(row as usize, column as usize))
            .flatten()
            .unique()
            .collect()
    }

    pub fn get_symbols(&self) -> Vec<Symbol> {
        let row_indices = 0..self.height;
        let column_indices = 0..self.width;
        let row_column_iterator = row_indices.cartesian_product(column_indices);
        let char_infos = row_column_iterator.map(|(row, column)| CharInfo {
            character: self.chars[row][column],
            row,
            column,
        });
        char_infos.map(Symbol::from).flatten().collect()
    }
    pub fn get_gear_ratios(&self) -> Vec<usize> {
        self.get_symbols()
            .iter()
            .filter(|&symbol| *symbol.char() == '*')
            .map(|symbol| self.get_numbers_adjacent_to_symbol(symbol))
            .filter(|vector| vector.len() == 2)
            .map(|vector| vector.get(0).unwrap() * vector.get(1).unwrap())
            .collect()
    }
    pub fn get_part_numbers(&self) -> Vec<usize> {
        self.get_symbols()
            .into_iter()
            .map(|symbol| self.get_numbers_adjacent_to_symbol(&symbol))
            .collect::<Vec<Vec<PartNumber>>>()
            .concat()
            .iter()
            .unique()
            .map(|part_number| part_number.number)
            .collect()
    }
}

impl std::ops::Mul<&PartNumber> for &PartNumber {
    type Output = usize;

    fn mul(self, rhs: &PartNumber) -> usize {
        rhs.number * self.number
    }
}
impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<Vec<char>> = s
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        let height = chars.len();
        let width = chars.iter().next().unwrap().len();

        if !chars.iter().all(|row| row.len() == width) {
            return Err(
                "Unable to create schematic since all rows are not equal length".to_string(),
            );
        }
        Ok(Self {
            chars,
            width,
            height,
        })
    }
}
