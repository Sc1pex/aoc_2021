use std::{fs::File, io::Read};

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day9.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut nums = Vec::new();
    for line in input.trim().split('\n') {
        nums.push(
            line.chars()
                .map(|s| (s as u8 - b'0') as i32)
                .collect::<Vec<i32>>(),
        );
        nums.last_mut().unwrap().insert(0, 10);
        nums.last_mut().unwrap().push(10);
    }
    let len = nums.last().unwrap().len();
    nums.insert(0, vec![10; len]);
    nums.push(vec![10; len]);

    let dir_x = [1, -1, 0, 0];
    let dir_y = [0, 0, 1, -1];
    let mut sum = 0;
    for i in 1..nums.len() - 1 {
        for j in 1..nums[i].len() - 1 {
            let mut min = 10;
            for k in 0..4 {
                min = min.min(nums[(i as i32 + dir_y[k]) as usize][(j as i32 + dir_x[k]) as usize]);
            }
            if min > nums[i][j] {
                sum += nums[i][j] + 1;
            }
        }
    }

    println!("Day 9 Part 1: {}", sum);
}

fn fill(v: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    if v[y][x] == 9 {
        return 0;
    }
    v[y][x] = 9;
    let dir_x = [1, -1, 0, 0];
    let dir_y = [0, 0, 1, -1];
    (0..4)
        .map(|e| {
            fill(
                v,
                (x as i32 + dir_x[e]) as usize,
                (y as i32 + dir_y[e]) as usize,
            )
        })
        .sum::<i32>()
        + 1
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day9.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut nums = Vec::new();
    for line in input.trim().split('\n') {
        nums.push(
            line.chars()
                .map(|s| (s as u8 - b'0') as i32)
                .collect::<Vec<i32>>(),
        );
        nums.last_mut().unwrap().insert(0, 9);
        nums.last_mut().unwrap().push(9);
    }
    let len = nums.last().unwrap().len();
    nums.insert(0, vec![9; len]);
    nums.push(vec![9; len]);

    let mut prod = Vec::new();
    for i in 1..nums.len() - 1 {
        for j in 1..nums[i].len() - 1 {
            if nums[i][j] != 9 {
                prod.push(fill(&mut nums, j, i));
            }
        }
    }
    prod.sort_by(|a, b| b.partial_cmp(a).unwrap());

    println!(
        "Day 9 Part 2: {}",
        prod.into_iter().take(3).product::<i32>()
    );
}
