use std::{fs::File, io::Read};

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day2.txt")
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read file");
    let pairs = input.lines().collect::<Vec<_>>();
    let pairs = pairs
        .iter()
        .map(|s| {
            let mut p = s.split(" ");
            (p.next(), p.next())
        })
        .collect::<Vec<_>>();
    let mut horizontal = 0;
    let mut depth = 0;
    for pair in pairs {
        if let (Some(s), Some(x)) = pair {
            let x = x.parse::<i32>().unwrap();
            match s {
                "forward" => horizontal += x,
                "down" => depth += x,
                "up" => depth -= x,
                _ => assert!(false),
            }
        }
    }
    println!(
        "Day 2 Part 1: h {}, d {}, h*d {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day2.txt")
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read file");
    let pairs = input.lines().collect::<Vec<_>>();
    let pairs = pairs
        .iter()
        .map(|s| {
            let mut p = s.split(" ");
            (p.next(), p.next())
        })
        .collect::<Vec<_>>();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for pair in pairs {
        if let (Some(s), Some(x)) = pair {
            let x = x.parse::<i32>().unwrap();
            match s {
                "forward" => {
                    horizontal += x;
                    depth += aim * x;
                }
                "down" => aim += x,
                "up" => aim -= x,
                _ => assert!(false),
            }
        }
    }
    println!(
        "Day 2 Part 1: h {}, d {}, h*d {}",
        horizontal,
        depth,
        horizontal * depth
    );
}
