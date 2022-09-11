use std::char::from_digit;
use std::cmp::min;

const MINE: char = '*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(r_index, r)| {
            r.chars()
                .enumerate()
                .map(|(c_index, c)| match c {
                    MINE => MINE,
                    _ => count_mines(r_index, c_index, minefield),
                })
                .collect()
        })
        .collect()
}

pub fn count_mines(r_index: usize, c_index: usize, minefield: &[&str]) -> char {
    let max_row_index: usize = minefield.len() - 1;
    let max_char_index: usize = minefield[r_index].len() - 1;

    let mut counter: u32 = 0;

    for y in r_index.saturating_sub(1)..=min(max_row_index, r_index + 1) {
        for x in c_index.saturating_sub(1)..=min(max_char_index, c_index + 1) {
            // println!(
            //     "r_index: {}, y: {}, c_index: {}, x: {}",
            //     r_index, y, c_index, x
            // );
            if minefield[y].chars().nth(x) == Some(MINE) {
                counter += 1;
            }
        }
    }

    if counter == 0 {
        ' '
    } else {
        from_digit(counter, 10).unwrap()
    }
}
