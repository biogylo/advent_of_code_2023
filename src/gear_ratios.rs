use std::collections::HashSet;
use std::io::SeekFrom;
use std::str::FromStr;

pub struct Symbol {
    pub character: char,
    pub row: usize,
    pub column: usize,
}
#[derive(Eq, PartialEq, Hash)]
pub struct PartNumber {
    pub number: usize,
    pub row: usize,
    pub column_start: usize,
    pub column_end: usize,
}
pub struct Schematic {
    chars: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Schematic {
    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.chars.get(y)?.get(x)
    }

    fn is_digit(&self, x: usize, y: usize) -> bool {
        if let Some(c) = self.get(x, y) {
            c.is_digit(10)
        } else {
            false
        }
    }

    pub fn get_number_row_boundaries(&self, x: usize, y: usize) -> Option<PartNumber> {
        let row = self.chars.get(y)?;

        if !self.is_digit(x, y) {
            return None;
        }

        // Find rightmost digit
        let mut rightmost = x;

        for i in x..row.len() {
            if row[i].is_digit(10) {
                rightmost = i
            } else {
                break;
            }
        }

        // Find leftmost digit
        let mut leftmost = x;

        for i in (0..=x).rev() {
            if row[i].is_digit(10) {
                leftmost = i
            } else {
                break;
            }
        }

        Some(PartNumber {
            number: row[leftmost..=rightmost]
                .iter()
                .collect::<String>()
                .parse()
                .unwrap(),
            row: y,
            column_start: leftmost,
            column_end: rightmost,
        })
    }
    pub fn get_numbers_adjacent_to_symbol(&self, symbol: Symbol) -> Vec<PartNumber> {
        let x = symbol.column;
        let y = symbol.row;
        let mut adjacent_numbers: HashSet<PartNumber> = HashSet::new();
        for i in x as isize - 1..=x as isize + 1 {
            for j in y as isize - 1..=y as isize + 1 {
                if let Some(part) = self.get_number_row_boundaries(i as usize, j as usize) {
                    adjacent_numbers.insert(part);
                }
            }
        }
        return adjacent_numbers.into_iter().collect::<Vec<PartNumber>>();
    }

    pub fn get_symbols(&self) -> Vec<Symbol> {
        let mut symbols: Vec<Symbol> = vec![];
        for (row_num, row) in self.chars.iter().enumerate() {
            for (col_num, character) in row.iter().enumerate() {
                let char_clone = character.clone();
                if char_clone != '.' && !char_clone.is_digit(10) {
                    symbols.push(Symbol {
                        character: char_clone,
                        row: row_num,
                        column: col_num,
                    })
                }
            }
        }
        symbols
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
        let mut part_numbers: HashSet<PartNumber> = HashSet::new();
        for symbol in self.get_symbols() {
            self.get_numbers_adjacent_to_symbol(symbol)
                .into_iter()
                .for_each(|number| {
                    part_numbers.insert(number);
                });
        }

        return part_numbers
            .into_iter()
            .collect::<Vec<PartNumber>>()
            .iter()
            .map(|number| number.number)
            .collect();
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

fn get_part_numbers(engine_schematic: ()) -> Vec<usize> {
    todo!()
}
