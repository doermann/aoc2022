// https://adventofcode.com/2022/day/4

use std::io::{BufRead, Cursor};
use std::ops::Range;

const INPUT: &'static str = include_str!(r"../data/day4.txt");

/// reads a line of 2 ranges
fn read_line(str: String) -> (Range<u32>, Range<u32>) {
    let parts: Vec<u32> = str
        .split([',', '-'])
        .map(|p| p.parse::<u32>().unwrap())
        .collect();

    (
        Range {
            start: parts[0],
            end: parts[1],
        },
        Range {
            start: parts[2],
            end: parts[3],
        },
    )
}

pub fn part1() -> u32 {
    let lines = Cursor::new(INPUT).lines();

    let mut sum = 0;

    for line in lines {
        let (first, second) = read_line(line.unwrap());

        if (first.start >= second.start && first.end <= second.end)
            || (first.start <= second.start && first.end >= second.end)
        {
            sum += 1;
        }
    }

    sum
}

pub fn part2() -> u32 {
    let lines = Cursor::new(INPUT).lines();

    let mut sum = 0;

    for line in lines {
        let (first, second) = read_line(line.unwrap());

        if (first.end >= second.start && first.start <= second.end)
            || (first.end <= second.start && first.start >= second.end)
        {
            sum += 1;
        }
    }

    sum
}
