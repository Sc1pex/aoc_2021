use std::{cmp, fs::File, io::Read, mem::swap};

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day5.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let lines = input.trim().split('\n');

    let mut field = [[0; 1000]; 1000];
    let mut points = 0;
    for line in lines {
        let mut split = line.split(" -> ");
        let mut pos1 = split.next().unwrap().split(',');
        let mut pos2 = split.next().unwrap().split(',');

        let mut x1: i32 = pos1.next().unwrap().parse().unwrap();
        let mut y1: i32 = pos1.next().unwrap().parse().unwrap();
        let mut x2: i32 = pos2.next().unwrap().parse().unwrap();
        let mut y2: i32 = pos2.next().unwrap().parse().unwrap();
        if x1 > x2 {
            swap(&mut x1, &mut x2);
        }
        if y1 > y2 {
            swap(&mut y1, &mut y2);
        }

        if x1 == x2 {
            for i in y1..=y2 {
                if field[x2 as usize][i as usize] == 1 {
                    points += 1;
                }
                field[x2 as usize][i as usize] += 1;
            }
        } else if y1 == y2 {
            for i in x1..=x2 {
                if field[i as usize][y2 as usize] == 1 {
                    points += 1;
                }
                field[i as usize][y2 as usize] += 1;
            }
        }
    }

    println!("Day 5 Part 1: {}", points);
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day5.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let lines = input.trim().split('\n');

    let mut field = [[0; 1000]; 1000];
    let mut points = 0;
    for line in lines {
        let mut split = line.split(" -> ");
        let mut pos1 = split.next().unwrap().split(',');
        let mut pos2 = split.next().unwrap().split(',');

        let mut x1: i32 = pos1.next().unwrap().parse().unwrap();
        let mut y1: i32 = pos1.next().unwrap().parse().unwrap();
        let mut x2: i32 = pos2.next().unwrap().parse().unwrap();
        let mut y2: i32 = pos2.next().unwrap().parse().unwrap();

        if x1 == x2 {
            if y1 > y2 {
                swap(&mut y1, &mut y2);
            }
            for i in y1..=y2 {
                if field[x2 as usize][i as usize] == 1 {
                    points += 1;
                }
                field[x2 as usize][i as usize] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                swap(&mut x1, &mut x2);
            }
            for i in x1..=x2 {
                if field[i as usize][y2 as usize] == 1 {
                    points += 1;
                }
                field[i as usize][y2 as usize] += 1;
            }
        } else {
            let len = cmp::max(x1, x2) - cmp::min(x1, x2);
            let dirx = if x1 > x2 { -1 } else { 1 };
            let diry = if y1 > y2 { -1 } else { 1 };
            for i in 0..=len {
                if field[(x1 + i * dirx) as usize][(y1 + i * diry) as usize] == 1 {
                    points += 1;
                }
                field[(x1 + i * dirx) as usize][(y1 + i * diry) as usize] += 1;
            }
        }
    }

    println!("Day 5 Part 2: {}", points);
}
