// https://adventofcode.com/2022/day/1

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, Cursor};

const INPUT: &'static str = include_str!(r"../data/day1.txt");

/// parses an optional u32 from a line
fn parse_line(line: String) -> Option<u32> {
    if line.len() != 0 {
        Some(line.parse::<u32>().unwrap())
    } else {
        None
    }
}

pub fn part1() -> u32 {
    let lines = Cursor::new(INPUT).lines();

    let mut max = 0;
    let mut current = 0;

    for line in lines {
        match parse_line(line.unwrap()) {
            Some(val) => current += val,
            None => {
                if current > max {
                    max = current;
                }

                current = 0;
            }
        }
    }

    max
}

pub fn part2() -> u32 {
    let lines = Cursor::new(INPUT).lines();

    // min-heap to track top 3
    let mut heap: BinaryHeap<Reverse<u32>> = BinaryHeap::with_capacity(3);
    let mut third_max = 0; // the third highest value
    let mut current = 0;

    for line in lines {
        match parse_line(line.unwrap()) {
            Some(val) => current += val,
            None => {
                if current > third_max {
                    if heap.len() == 3 {
                        heap.pop();
                    }
                    heap.push(Reverse(current));
                    third_max = heap.peek().unwrap().0;
                }

                current = 0;
            }
        }
    }

    heap.iter().map(|i| i.0).sum()
}
