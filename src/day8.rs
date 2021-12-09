use std::{fs::File, io::Read};

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day8.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut num = 0;
    let lines = input.trim().split('\n');
    for line in lines {
        let mut split = line.trim().split(" | ");
        split.next().unwrap();
        let output = split.next().unwrap().split(' ');

        for x in output {
            if x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7 {
                num += 1;
            }
        }
    }

    println!("Day 8 Part 1: {}", num);
}

pub fn part2() {
    println!("Day 8 Part 2: {}", 0);
}
