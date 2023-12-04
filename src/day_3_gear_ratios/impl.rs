use crate::day_3_gear_ratios;
use crate::day_3_gear_ratios::{CharInfo, PartNumber, Schematic, Symbol};
use itertools::Itertools;
use std::str::FromStr;

impl Symbol {
    fn from(char_info: CharInfo) -> Option<Symbol> {
        if day_3_gear_ratios::is_symbol(&char_info.character) {
            return Some(Symbol {
                character: char_info.character,
                row: char_info.row,
                column: char_info.column,
            });
        }
        return None;
    }
}

impl Schematic {
    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.chars.get(y)?.get(x)
    }

    pub fn get_number_row_boundaries(&self, x: usize, y: usize) -> Option<PartNumber> {
        let the_row_chars = self.chars.get(y)?;
        the_row_chars.get(x)?.to_digit(10)?;

        // Find rightmost digit
        let rightmost = (x..the_row_chars.len())
            .take_while(|&i| the_row_chars[i].is_digit(10))
            .last()
            .unwrap();

        // Find leftmost digit
        let leftmost = (0..=x)
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
            row: y,
            column_start: leftmost,
            column_end: rightmost,
        })
    }
    pub fn get_numbers_adjacent_to_symbol(&self, symbol: Symbol) -> Vec<PartNumber> {
        let y_iter = (symbol.row as i32 - 1..=symbol.row as i32 + 1)
            .into_iter()
            .filter(|&x| (0 <= x) && (x < self.width as i32));
        let x_iter = (symbol.column as i32 - 1..=symbol.column as i32 + 1)
            .into_iter()
            .filter(|&y| (0 <= y) && (y < self.height as i32));
        x_iter
            .cartesian_product(y_iter)
            .map(|(x, y)| self.get_number_row_boundaries(x as usize, y as usize))
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
        let mut ratios: Vec<usize> = vec![];

        for symbol in self.get_symbols() {
            if symbol.character != '*' {
                continue;
            }
            let adjacent_numbers = self.get_numbers_adjacent_to_symbol(symbol);
            if adjacent_numbers.len() != 2 {
                continue;
            }
            ratios.push(adjacent_numbers[0].number * adjacent_numbers[1].number)
        }

        return ratios;
    }
    pub fn get_part_numbers(&self) -> Vec<usize> {
        self.get_symbols()
            .into_iter()
            .map(|symbol| self.get_numbers_adjacent_to_symbol(symbol))
            .collect::<Vec<Vec<PartNumber>>>()
            .concat()
            .iter()
            .unique()
            .map(|part_number| part_number.number)
            .collect()
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
