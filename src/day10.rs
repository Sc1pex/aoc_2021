use std::{fs::File, io::Read};

fn is_open(c: char) -> bool {
    c == '(' || c == '{' || c == '<' || c == '['
}

fn is_pair(c1: char, c2: char) -> bool {
    if c1 == '(' {
        return c2 == ')';
    }
    c2 as i32 - c1 as i32 == 2
}

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day10.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut sum = 0;
    for line in input.trim().split('\n') {
        let mut open_chunks = Vec::new();
        for c in line.chars() {
            if is_open(c) {
                open_chunks.push(c);
            } else if is_pair(*open_chunks.last().unwrap(), c) {
                open_chunks.pop();
            } else {
                if c == ')' {
                    sum += 3;
                } else if c == ']' {
                    sum += 57;
                } else if c == '}' {
                    sum += 1197;
                } else {
                    sum += 25137;
                }
                break;
            }
        }
    }

    println!("Day 10 Part 1: {}", sum);
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day10.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut scores = Vec::new();
    for line in input.trim().split('\n') {
        let mut open_chunks = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            if is_open(c) {
                open_chunks.push(c);
            } else if is_pair(*open_chunks.last().unwrap(), c) {
                open_chunks.pop();
            } else {
                corrupt = true;
                break;
            }
        }
        if !corrupt {
            let mut sum: u64 = 0;
            while let Some(x) = open_chunks.last() {
                sum *= 5;
                if *x == '(' {
                    sum += 1;
                } else if *x == '[' {
                    sum += 2;
                } else if *x == '{' {
                    sum += 3;
                } else {
                    sum += 4;
                }
                open_chunks.pop();
            }
            scores.push(sum);
        }
    }

    scores.sort_unstable();
    println!("Day 10 Part 2: {}", scores[scores.len() / 2]);
}
