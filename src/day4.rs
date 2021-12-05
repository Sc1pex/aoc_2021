use std::{fs::File, io::Read, str::FromStr};

fn bingo(v: &[i32]) -> bool {
    for i in 0..5 {
        if v[i] == 5 {
            return true;
        }
    }
    false
}

fn calculate_board(mut board: Vec<Vec<i32>>, nums: &[i32]) -> (i32, i32) {
    let mut rows = [0; 5];
    let mut columns = [0; 5];
    let mut marked = 0;
    for k in 0..nums.len() {
        let mut check = false;
        for i in 0..5 {
            for j in 0..5 {
                if board[i][j] == nums[k] {
                    board[i][j] = -1;
                    rows[j] += 1;
                    columns[i] += 1;
                    check = true;
                    marked += 1;
                    break;
                }
            }
        }
        if check {
            if bingo(&rows) {
                let mut sum = 0;
                for i in 0..board.len() {
                    sum += board[i].iter().sum::<i32>();
                }
                sum += marked;
                return (k as i32, sum * nums[k]);
            }
            if bingo(&columns) {
                let mut sum = 0;
                for i in 0..board.len() {
                    sum += board[i].iter().sum::<i32>();
                }
                sum += marked;
                return (k as i32, sum * nums[k]);
            }
        }
    }
    (50, 0)
}

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day4.txt")
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read file");
    let input = input
        .lines()
        .map(|s| String::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let nums = input[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 2;
    let mut best_score = -1;
    let mut best_rezult = 0;
    while i < input.len() {
        let mut board = Vec::new();
        for j in 0..5 {
            board.push(
                input[i + j]
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        let (score, rezult) = calculate_board(board, &nums);
        if score < best_score || best_score == -1 {
            best_score = score;
            best_rezult = rezult;
        }
        i += 6;
    }
    println!("Day 4 Part 1: {}, {}", best_rezult, best_score);
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day4.txt")
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read file");
    let input = input
        .lines()
        .map(|s| String::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let nums = input[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 2;
    let mut best_score = -1;
    let mut best_rezult = 0;
    while i < input.len() {
        let mut board = Vec::new();
        for j in 0..5 {
            board.push(
                input[i + j]
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        let (score, rezult) = calculate_board(board, &nums);
        if score > best_score {
            best_score = score;
            best_rezult = rezult;
        }
        i += 6;
    }
    println!("Day 4 Part 2: {}, {}", best_rezult, best_score);
}
