use std::{fs::File, io::Read};

pub fn part1() {
    let mut prev = -1;
    let mut total = 0;
    let mut input = String::new();
    File::open("inputs/day1.txt")
        .expect("Failed to open input file")
        .read_to_string(&mut input)
        .expect("Failed to read input file");
    let split = input.split('\n');
    for s in split {
        if let Ok(x) = s.parse::<i32>() {
            if prev > -1 && x > prev {
                total += 1;
            }
            prev = x;
        }
    }
    println!("Day 1 Part 1: {}", total);
}

pub fn part2() {
    let mut sum;
    let mut total = 0;
    let mut prev_sum = -1;
    let mut input = String::new();
    File::open("inputs/day1.txt")
        .expect("Failed to open input file")
        .read_to_string(&mut input)
        .expect("Failed to read input file");
    let nums = input.lines().collect::<Vec<_>>();
    let nums = nums
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    sum = nums[0] + nums[1] + nums[2];
    for i in 3..nums.len() {
        sum = sum + nums[i] - nums[i - 3];
        if prev_sum > -1 && sum > prev_sum {
            total += 1;
        }
        prev_sum = sum;
    }
    println!("Day 1 Part 2: {}", total);
}
