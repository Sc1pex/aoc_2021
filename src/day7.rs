use std::{fs::File, io::Read};

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day7.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let nums = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut best_sum = -1;
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    for i in min..=max {
        let sum = nums.iter().map(|x| (x - i).abs()).sum();
        if sum < best_sum || best_sum == -1 {
            best_sum = sum;
        }
    }

    println!("Day 7 Part 1: {}", best_sum);
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day7.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let nums = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut best_sum = -1;
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    for i in min..=max {
        let sum = nums
            .iter()
            .map(|x| {
                let d = (x - i).abs();
                d * (d + 1) / 2
            })
            .sum();
        if sum < best_sum || best_sum == -1 {
            best_sum = sum;
        }
    }

    println!("Day 7 Part 2: {}", best_sum);
}
