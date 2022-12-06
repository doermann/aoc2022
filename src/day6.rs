// https://adventofcode.com/2022/day/6

use std::collections::HashSet;

const INPUT: &'static str = include_str!(r"../data/day6.txt");

/// finds the end index of the first unique series of `size`
fn find_unique_consecutive(chars: Vec<char>, size: usize) -> usize {
    for (i, win) in chars.windows(size).enumerate() {
        let values: HashSet<&char> = HashSet::from_iter(win.iter());
        if values.len() == size {
            return i + size;
        }
    }

    panic!("value not found");
}

pub fn part1() -> usize {
    find_unique_consecutive(INPUT.chars().collect(), 4)
}

pub fn part2() -> usize {
    find_unique_consecutive(INPUT.chars().collect(), 14)
}
