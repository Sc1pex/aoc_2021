use std::{fs::File, io::Read};

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day6.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut values = [0; 9];
    let mut count = nums.len();
    let days = 80;
    for num in nums {
        values[num as usize] += 1;
    }

    for _ in 1..=days {
        let x = values[0];
        for i in 0..8 {
            values[i] = values[i + 1];
        }
        values[6] += x;
        values[8] = x;
        count += x;
    }

    println!("Day 6 Part 1 {}", count);
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day6.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let nums: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut values = [0; 9];
    let mut count = nums.len();
    let days = 256;
    for num in nums {
        values[num as usize] += 1;
    }

    for _ in 1..=days {
        let x = values[0];
        for i in 0..8 {
            values[i] = values[i + 1];
        }
        values[6] += x;
        values[8] = x;
        count += x;
    }

    println!("Day 6 Part 1 {}", count);
}
