mod r#impl;

pub struct CharInfo {
    pub character: char,
    pub row: usize,
    pub column: usize,
}

pub struct Symbol(CharInfo);

fn is_symbol(character: &char) -> bool {
    !character.is_digit(10) && *character != '.'
}

#[derive(Eq, PartialEq, Hash, Clone)]
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
