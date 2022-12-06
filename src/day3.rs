// https://adventofcode.com/2022/day/3

use std::{
    collections::HashSet,
    io::{BufRead, Cursor},
};

const INPUT: &'static str = include_str!(r"../data/day3.txt");

/// converts a char to a priority value
fn priority_value(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - ('A' as u32) + 27
    } else {
        (c as u32) - ('a' as u32) + 1
    }
}

pub fn part1() -> u32 {
    let lines = Cursor::new(INPUT).lines();

    let mut sum = 0;

    for line in lines {
        let str = line.unwrap();
        let (part1, part2) = str.split_at(str.len() / 2);

        let set: HashSet<char> = HashSet::from_iter(part1.chars());
        let duplicate = part2.chars().find(|c| set.contains(c)).unwrap();

        sum += priority_value(duplicate);
    }

    sum
}

pub fn part2() -> u32 {
    let lines: Vec<String> = Cursor::new(INPUT).lines().map(|l| l.unwrap()).collect();

    let mut sum = 0;

    for chunk in lines.as_slice().chunks(3) {
        // make 3 sets from the lines
        let sets: Vec<HashSet<char>> = chunk
            .iter()
            .map(|l| HashSet::from_iter(l.chars()))
            .collect();

        // 3 way intersection
        let first: HashSet<char> = sets[0].intersection(&sets[1]).map(|c| *c).collect();
        let badge = first.intersection(&sets[2]).next().unwrap();

        sum += priority_value(*badge);
    }

    sum
}
