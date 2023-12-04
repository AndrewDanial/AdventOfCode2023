#![allow(unused)]
const FILE: &str = include_str!("../input.txt");
use std::collections::{BTreeMap, HashSet};
struct Scratch {
    score: i32,
    winning: HashSet<i32>,
    numbers: HashSet<i32>,
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i32 {
    let split: Vec<&str> = FILE.lines().collect();
    let mut sums = vec![];
    for line in split {
        let idx = line.chars().position(|c| c == ':').unwrap();
        let new_line: Vec<&str> = line[idx + 1..].trim().split(" | ").collect();
        let winning: HashSet<i32> = new_line[0]
            .split(" ")
            .map(|s| {
                let a = s.parse::<i32>();
                if a.is_ok() {
                    a.unwrap()
                } else {
                    -1
                }
            })
            .collect();
        let numbers: HashSet<i32> = new_line[1]
            .split(" ")
            .map(|s| {
                let a = s.parse::<i32>();
                if a.is_ok() {
                    a.unwrap()
                } else {
                    -2
                }
            })
            .collect();

        let mut scratch = Scratch {
            score: 0,
            winning,
            numbers,
        };
        for i in scratch.numbers {
            if scratch.winning.contains(&i) {
                if scratch.score == 0 {
                    scratch.score = 1
                } else {
                    scratch.score *= 2;
                }
            }
        }
        sums.push(scratch.score);
    }
    sums.iter().sum()
}
fn part2() -> i32 {
    let split: Vec<&str> = FILE.lines().collect();
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    for (i, line) in split.iter().enumerate() {
        let idx = line.chars().position(|c| c == ':').unwrap();
        let new_line: Vec<&str> = line[idx + 1..].trim().split(" | ").collect();
        let winning: HashSet<i32> = new_line[0]
            .split(" ")
            .map(|s| {
                let a = s.parse::<i32>();
                if a.is_ok() {
                    a.unwrap()
                } else {
                    -1
                }
            })
            .collect();
        let numbers: HashSet<i32> = new_line[1]
            .split(" ")
            .map(|s| {
                let a = s.parse::<i32>();
                if a.is_ok() {
                    a.unwrap()
                } else {
                    -2
                }
            })
            .collect();
        let mut count = numbers
            .iter()
            .fold(0, |acc, x| if winning.contains(&x) { acc + 1 } else { acc });

        *map.entry(i as i32 + 1).or_insert(0) += 1;

        for mut x in 1..=count {
            // for all the numbers we won, add to the
            *map.entry((i as i32 + x + 1)).or_insert(0) += *map.get(&(i as i32 + 1)).unwrap();
        }
    }
    println!("{:?}", map);
    map.values().sum()
}
