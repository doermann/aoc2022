// https://adventofcode.com/2022/day/5

use std::io::Lines;
use std::io::{BufRead, Cursor};

const INPUT: &'static str = include_str!(r"../data/day5.txt");

/// read the initial stack data
fn read_stacks(lines: &mut Lines<Cursor<&str>>) -> Vec<Vec<char>> {
    let mut initial: Vec<String> = Vec::new();

    // read until blank line
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.len() == 0 {
            break;
        }

        initial.push(line);
    }

    // make our stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let num = initial.pop().unwrap().split_whitespace().count();
    for _ in 0..num {
        stacks.push(Vec::new());
    }

    // read in reverse and push items
    for line in initial.iter().rev() {
        for (i, _) in line.match_indices("[") {
            let index = i / 4; // 4 chars per column
            stacks[index].push(line.chars().nth(i + 1).unwrap());
        }
    }

    stacks
}

/// read a move line
fn read_line(str: String) -> (usize, usize, usize) {
    let parts: Vec<&str> = str.split_whitespace().collect();

    let num = parts[1].parse::<usize>().unwrap();
    let from = parts[3].parse::<usize>().unwrap() - 1;
    let to = parts[5].parse::<usize>().unwrap() - 1;

    (num, from, to)
}

pub fn part1() -> String {
    let mut lines = Cursor::new(INPUT).lines();
    let mut stacks = read_stacks(&mut lines);

    for line in lines {
        let (num, from, to) = read_line(line.unwrap());

        // push each individually
        for _ in 0..num {
            let val = stacks[from].pop().unwrap();
            stacks[to].push(val);
        }
    }

    // return the top of each stack
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

pub fn part2() -> String {
    let mut lines = Cursor::new(INPUT).lines();
    let mut stacks = read_stacks(&mut lines);

    for line in lines {
        let (num, from, to) = read_line(line.unwrap());

        // push the full stack
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..num {
            temp.insert(0, stacks[from].pop().unwrap());
        }
        stacks[to].append(&mut temp);
    }

    // return the top of each stack
    stacks.iter().map(|s| s.last().unwrap()).collect()
}
