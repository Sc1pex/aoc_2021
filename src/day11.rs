use std::{fs::File, io::Read};

static DIR_X: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
static DIR_Y: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
fn increment(x: usize, y: usize, nums: &mut Vec<Vec<u8>>, flash: &mut Vec<Vec<i32>>) {
    for k in 0..7 {
        let j = (x as i32 + DIR_X[k]) as usize;
        let i = (y as i32 + DIR_Y[k]) as usize;
        if i < nums[x].len() && j < nums.len() {
            nums[i][j] += 1;
            if nums[i][j] > 9 && flash[i][j] == 0 {
                flash[i][j] = 1;
                increment(i, j, nums, flash);
            }
        }
    }
}

fn print(_nums: &[Vec<u8>], _flash: &[Vec<i32>]) {
    // for i in 0..nums.len() {
    //     println!("{:?}", nums[i]);
    // }
    // println!("--------------------------");
    // for i in 0..nums.len() {
    //     println!("{:?}", flash[i]);
    // }
    // println!("--------------------------\n");
}

fn step(nums: &mut Vec<Vec<u8>>) -> i32 {
    for i in 0..nums.len() {
        for j in 0..nums[i].len() {
            nums[i][j] += 1;
        }
    }

    let mut flash = vec![vec![0; nums[0].len()]; nums.len()];
    print(nums, &flash);

    for i in 0..nums.len() {
        for j in 0..nums[i].len() {
            if nums[i][j] > 9 && flash[i][j] == 0 {
                flash[i][j] = 1;
                increment(i, j, nums, &mut flash);
            }
        }
    }

    for i in 0..nums.len() {
        for j in 0..nums[i].len() {
            if flash[i][j] == 1 {
                nums[i][j] = 0;
            }
        }
    }

    print(nums, &flash);

    flash.into_iter().map(|x| x.into_iter().sum::<i32>()).sum()
}

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day11.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut nums = Vec::new();
    for line in input.trim().split('\n') {
        nums.push(line.chars().map(|x| x as u8 - b'0').collect::<Vec<u8>>());
        // println!("{:?}", nums.last().unwrap());
    }
    // println!("--------------------------\n\n");

    let mut flashes = 0;
    // for _ in 0..10 {
    let f = step(&mut nums);
    // println!("{}", f);
    flashes += f;
    // }

    println!("Day 11 Part 1: {}", flashes);
}

pub fn part2() {
    println!("Day 11 Part 2: {}", 0);
}
