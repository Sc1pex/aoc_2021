use std::{fs::File, io::Read};

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day3.txt")
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read string");
    let digits = input.lines().collect::<Vec<_>>();
    let bits = digits[0].len();
    let mut counts = vec![0; bits];
    for num in &digits {
        for i in 0..bits {
            if num.as_bytes()[i] == '1' as u8 {
                counts[i] += 1;
            }
        }
    }
    let n = digits.len() / 2;
    let mut x: u32 = 0;
    let mut p: u32 = 1 << 11;
    for i in 0..bits {
        if counts[i] > n {
            x += p;
        }
        p /= 2;
    }
    let y = !x & ((1 << 12) - 1);
    println!("Day 3 Part 1: {}", x * y);
}

fn get_count(bit: usize, nums: &Vec<&str>) -> i32 {
    let mut count = 0;
    for num in nums {
        if num.as_bytes()[bit] == '1' as u8 {
            count += 1;
        }
    }
    count
}

fn binary_str_to_i32(s: &str) -> i32 {
    let mut x = 0;
    let mut p = 1 << (s.len() - 1);
    for ch in s.as_bytes() {
        x += (ch - '0' as u8) as i32 * p;
        p /= 2;
    }
    x
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day3.txt")
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read string");
    let digits = input.lines().collect::<Vec<_>>();

    // Determine oxygen rating
    let mut nums = digits.clone();
    let mut bit = 0;
    while nums.len() > 1 {
        let count = get_count(bit, &nums);
        let n = (nums.len() / 2 + nums.len() % 2) as i32;
        let b = (count >= n) as u8 + '0' as u8;
        let mut i = 0;
        while i < nums.len() as i32 {
            if nums[i as usize].as_bytes()[bit] != b {
                nums.remove(i as usize);
                i -= 1;
            }
            i += 1;
        }
        // println!("{}, {}\n{:?}", bit, b, nums);
        bit += 1;
    }
    let o2_rating = binary_str_to_i32(nums[0]);

    // Determine co2 rating
    let mut nums = digits.clone();
    let mut bit = 0;
    while nums.len() > 1 {
        let count = get_count(bit, &nums);
        let n = (nums.len() / 2 + nums.len() % 2) as i32;
        let b = (count < n) as u8 + '0' as u8;
        let mut i = 0;
        while i < nums.len() as i32 {
            if nums[i as usize].as_bytes()[bit] != b {
                nums.remove(i as usize);
                i -= 1;
            }
            i += 1;
        }
        // println!("{}, {}\n{:?}", bit, b, nums);
        bit += 1;
    }
    let co2_rating = binary_str_to_i32(nums[0]);
    println!(
        "Day 3 Part 2: O2 {}, CO2 {}, {}",
        o2_rating,
        co2_rating,
        o2_rating * co2_rating
    );
}
