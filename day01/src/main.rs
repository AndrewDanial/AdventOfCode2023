#![allow(warnings)]
use std::collections::HashMap;
fn main() {
    let file = include_str!("../input.txt");
    // println!("{}", part1(file));
    println!("{}", part2(file));
}

fn part1(input: &str) -> i32 {
    let split = input.split_whitespace().collect::<Vec<&str>>();
    let mut vals: Vec<i32> = vec![];
    for string in split {
        let mut first = true;
        let mut latest = ' ';
        let mut combo = String::from("");
        for char in string.chars() {
            if char.is_numeric() {
                latest = char;
                if first == true {
                    combo += &latest.to_string();
                    first = false;
                }
            }
        }
        combo += &latest.to_string();
        vals.push(combo.parse::<i32>().unwrap());
        combo.clear();
    }
    vals.iter().sum()
}

fn part2(input: &str) -> i32 {
    let split = input.split_whitespace().collect::<Vec<&str>>();
    let map: HashMap<&str, &str> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();
    let mut vals: Vec<i32> = vec![];

    let patterns = vec![
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    for string in split {
        let mut combo = String::from("");
        let mut first: (usize, &str) = (usize::MAX, "");
        let mut last: (usize, &str) = (usize::MIN, "");

        for pat in &patterns {
            let matches: Vec<(usize, &str)> = string.match_indices(pat).collect();
            if matches.is_empty() {
                continue;
            }
            if matches[0].0 < first.0 {
                first = matches[0];
            }

            if matches[matches.len() - 1].0 >= last.0 {
                last = matches[matches.len() - 1];
            }
        }

        if map.contains_key(first.1) {
            combo += map.get(first.1).unwrap();
        } else {
            combo += first.1;
        }

        if map.contains_key(last.1) {
            combo += map.get(last.1).unwrap();
        } else {
            combo += last.1;
        }

        vals.push(combo.parse::<i32>().unwrap());
        combo.clear();
    }

    vals.iter().sum()
}
