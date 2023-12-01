fn get_first_and_last_number(list: Vec<u32>) -> Option<(u32, u32)> {
    let mut iter = list.iter();
    let first: &u32 = iter.next()?;
    let mut last: &u32 = first;
    iter.for_each(|num| {
        last = num;
    });
    return Some((*first, *last));
}
pub fn get_trebuchet(trebuchet: &str) -> Option<u32> {
    let all_digits: Vec<u32> = trebuchet.chars().filter_map(|z| z.to_digit(10)).collect();

    let (first, last) = get_first_and_last_number(all_digits)?;

    Some(first * 10 + last)
}

pub fn get_trebuchet_multiple(trebuchet: &str) -> u32 {
    let list_of_line_sums: Vec<u32> = trebuchet.lines().filter_map(get_trebuchet).collect();
    return list_of_line_sums.iter().sum();
}
