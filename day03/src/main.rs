const FILE: &str = include_str!("../input.txt");
fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i32 {
    let split: Vec<Vec<char>> = FILE
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let mut nums = vec![];
    let mut i = 0;
    while i < split.len() {
        let mut j = 0;
        let mut symbol = false;
        let mut curr_num = String::from("");
        while j < split[i].len() {
            let mut curr_char = split[i][j];
            while curr_char.is_numeric() {
                curr_num += &curr_char.to_string();
                if check_surroundings(i as i32, j as i32, &split) {
                    symbol = true;
                }
                j += 1;
                if j > split[i].len() - 1 {
                    break;
                }
                curr_char = split[i][j];
            }

            if symbol {
                nums.push(curr_num.parse().unwrap());
            }
            symbol = false;
            curr_num = String::from("");

            j += 1;
        }
        i += 1;
    }
    nums.iter().sum()
}

fn part2() -> i32 {
    let split: Vec<Vec<char>> = FILE
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let mut i = 0;
    let mut totals = vec![];
    while i < split.len() {
        let mut j = 0;
        while j < split[i].len() {
            if split[i][j] == '*' {
                let nums = check_nums(i as i32, j as i32, &split);
                println!("{:?}", nums);
                if nums.len() == 2 {
                    let mut gears = vec![];
                    for x in nums {
                        gears.push(collect_nums(x.1 as usize, &split[x.0 as usize]));
                    }
                    println!("{:?}", gears);
                    totals.push(gears.iter().fold(1, |acc, x| acc * x));
                }
            }
            j += 1;
        }
        i += 1;
    }
    totals.iter().sum()
}

fn check_surroundings(i: i32, j: i32, split: &Vec<Vec<char>>) -> bool {
    for x in -1..=1 {
        for y in -1..=1 {
            if i + x < 0 {
                continue;
            }
            if i + x >= split.len() as i32 {
                continue;
            }
            if j + y < 0 {
                continue;
            }
            if j + y >= split[0].len() as i32 {
                continue;
            }
            if x == 0 && y == 0 {
                continue;
            }

            let curr_char = split[(i + x) as usize][(j + y) as usize];
            if curr_char != '.' && !curr_char.is_numeric() {
                return true;
            }
        }
    }
    false
}

fn check_nums(i: i32, j: i32, split: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut nums: Vec<(i32, i32)> = vec![];
    for x in -1..=1 {
        if i + x < 0 {
            continue;
        }
        if i + x >= split.len() as i32 {
            continue;
        }
        let row = split[(i + x) as usize][(j - 1) as usize].to_string()
            + &split[(i + x) as usize][(j) as usize].to_string()
            + &split[(i + x) as usize][(j + 1) as usize].to_string();
        println!("{:?}", row);
        for y in -1..=1 {
            if j + y < 0 {
                continue;
            }
            if j + y >= split[0].len() as i32 {
                continue;
            }

            let curr_char = split[(i + x) as usize][(j + y) as usize];

            let i_check = if nums.is_empty() {
                true
            } else {
                (nums[nums.len() - 1].0) == i + x
            };
            let j_check = if nums.is_empty() {
                true
            } else {
                !((nums.last().unwrap().1 - (j + y)).abs() > 0)
            };
            if curr_char.is_numeric() {
                if i_check {
                    let check_symbol = !row.chars().nth(1).unwrap().is_numeric();
                    if check_symbol {
                        nums.push((i + x, j + y));
                        continue;
                    }
                    if j_check {
                        nums.push((i + x, j + y));
                    }
                } else {
                    nums.push((i + x, j + y));
                }
            }
        }
    }
    nums
}

fn collect_nums(j: usize, split: &Vec<char>) -> i32 {
    let mut curr_num = String::from(split[j].to_string());

    for i in (0..j).rev() {
        if !split[i].is_numeric() {
            break;
        }
        curr_num = split[i].to_string() + &curr_num;
    }
    for i in j + 1..split.len() {
        if !split[i].is_numeric() {
            break;
        }
        curr_num = curr_num + &split[i].to_string();
    }
    curr_num.parse().unwrap()
}
