// https://adventofcode.com/2022/day/2

use std::io::{BufRead, Cursor};

const INPUT: &'static str = include_str!(r"../data/day2.txt");

/// reads the first and third chars from a line
fn parse_line(str: String) -> (char, char) {
    (str.chars().nth(0).unwrap(), str.chars().nth(2).unwrap())
}

pub fn part1() -> u32 {
    let lines = Cursor::new(INPUT).lines();

    let mut score = 0;

    for line in lines {
        let (opp, you) = parse_line(line.unwrap());

        // value of shape
        score += (you as u32) - ('X' as u32) + 1;

        // value of game result
        score += match (opp, you) {
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // tie
            ('C', 'X') | ('A', 'Y') | ('B', 'Z') => 6, // win
            _ => 0,                                    // loss
        }
    }

    score
}

pub fn part2() -> u32 {
    let lines = Cursor::new(INPUT).lines();

    let mut score = 0;

    for line in lines {
        let (opp, result) = parse_line(line.unwrap());

        // value of shape
        score += match (opp, result) {
            ('B', 'X') | ('A', 'Y') | ('C', 'Z') => 1, // rock
            ('C', 'X') | ('B', 'Y') | ('A', 'Z') => 2, // paper
            _ => 3,                                    // scissors
        };

        // value of game result
        score += match result {
            'Y' => 3, // tie
            'Z' => 6, // win
            _ => 0,   // loss
        };
    }

    score
}
