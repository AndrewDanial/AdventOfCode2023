// #![allow(unused)]
fn main() {
    let file = include_str!("../input.txt");
    println!("{:?}", part1(file));
    println!("{:?}", part2(file));
}

fn part1(input: &str) -> usize {
    let split: Vec<&str> = input.lines().collect();
    let mut res = vec![];
    for line in split {
        let index = line.chars().position(|c| c == ':').unwrap();
        let id = line[5..index].parse::<usize>().unwrap();
        let sets: Vec<&str> = line[index + 2..].split("; ").collect();
        let mut add = true;
        for i in sets {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let real: Vec<&str> = i.split(", ").collect();
            for i in real {
                let check = i.split(" ").collect::<Vec<&str>>();
                let num = check[0].parse::<i32>().unwrap();
                if check[1] == "red" {
                    red += num;
                }
                if check[1] == "green" {
                    green += num;
                }
                if check[1] == "blue" {
                    blue += num;
                }
            }
            if !(red <= 12 && green <= 13 && blue <= 14) {
                add = false;
                break;
            }
        }
        if add {
            res.push(id);
        }
    }

    res.iter().sum()
}

fn part2(input: &str) -> i32 {
    let split: Vec<&str> = input.lines().collect();
    let mut res = vec![];
    for line in split {
        let index = line.chars().position(|c| c == ':').unwrap();
        let sets: Vec<&str> = line[index + 2..].split("; ").collect();
        let mut red = vec![];
        let mut green = vec![];
        let mut blue = vec![];
        for i in sets {
            let real: Vec<&str> = i.split(", ").collect();
            for i in real {
                let check = i.split(" ").collect::<Vec<&str>>();
                let num = check[0].parse::<i32>().unwrap();

                if check[1] == "red" {
                    red.push(num);
                }
                if check[1] == "green" {
                    green.push(num);
                }
                if check[1] == "blue" {
                    blue.push(num);
                }
            }
        }
        let red_max = red.iter().max().unwrap();
        let green_max = green.iter().max().unwrap();
        let blue_max = blue.iter().max().unwrap();
        res.push(red_max * green_max * blue_max);
        red.clear();
        green.clear();
        blue.clear();
    }

    res.iter().sum()
}
