use std::{collections::HashMap, fs::File, io::Read};

fn number_of_paths(
    start: String,
    end: &str,
    visited: &mut HashMap<String, u8>,
    map: &HashMap<String, Vec<String>>,
    num: &mut i32,
) {
    if start.to_lowercase() == start {
        *visited.get_mut(&start).unwrap() = 1;
    }

    if start == *end {
        *num += 1;
    } else if let Some(x) = map.get(&start) {
        x.iter().for_each(|s| {
            if s.to_lowercase() == *s {
                if *visited.get(s).unwrap() == 0 {
                    number_of_paths(s.to_string(), end, visited, map, num);
                }
            } else {
                number_of_paths(s.to_string(), end, visited, map, num);
            }
        });
    }

    if start.to_lowercase() == start {
        *visited.get_mut(&start).unwrap() = 0;
    }
}

pub fn part1() {
    let mut input = String::new();
    File::open("inputs/day12.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut visited: HashMap<String, u8> = HashMap::new();
    for line in input.trim().split('\n') {
        let mut line = line.split('-');
        let point1 = line.next().unwrap().to_string();
        let point2 = line.next().unwrap().to_string();

        match map.get_mut(&point1) {
            Some(x) => {
                x.push(point2.clone());
            }
            None => {
                map.insert(point1.clone(), vec![point2.clone()]);
            }
        }
        match map.get_mut(&point2) {
            Some(x) => {
                x.push(point1.clone());
            }
            None => {
                map.insert(point2.clone(), vec![point1.clone()]);
            }
        }

        if visited.get_mut(&point1) == None {
            visited.insert(point1, 0);
        }
        if visited.get_mut(&point2) == None {
            visited.insert(point2, 0);
        }
    }

    let mut num = 0;
    number_of_paths("start".to_string(), "end", &mut visited, &map, &mut num);

    println!("Day 12 Part 1: {}", num);
}

fn number_of_paths2(
    start: String,
    end: &str,
    visited: &mut HashMap<String, u8>,
    map: &HashMap<String, Vec<String>>,
    double: &str,
    num: &mut i32,
) {
    if start.to_lowercase() == start {
        *visited.get_mut(&start).unwrap() += 1;
    }

    if start == *end {
        *num += 1;
    } else if let Some(x) = map.get(&start) {
        x.iter().for_each(|s| {
            if s.to_lowercase() == *s {
                if *visited.get(s).unwrap() == 0 || (s == double && *visited.get(s).unwrap() == 1) {
                    number_of_paths2(s.to_string(), end, visited, map, double, num);
                }
            } else {
                number_of_paths2(s.to_string(), end, visited, map, double, num);
            }
        });
    }

    if start.to_lowercase() == start {
        *visited.get_mut(&start).unwrap() -= 1;
    }
}

pub fn part2() {
    let mut input = String::new();
    File::open("inputs/day12.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut visited: HashMap<String, u8> = HashMap::new();
    let mut doubles = Vec::new();
    for line in input.trim().split('\n') {
        let mut line = line.split('-');
        let point1 = line.next().unwrap().to_string();
        let point2 = line.next().unwrap().to_string();

        match map.get_mut(&point1) {
            Some(x) => {
                x.push(point2.clone());
            }
            None => {
                map.insert(point1.clone(), vec![point2.clone()]);
            }
        }
        match map.get_mut(&point2) {
            Some(x) => {
                x.push(point1.clone());
            }
            None => {
                map.insert(point2.clone(), vec![point1.clone()]);
            }
        }

        if visited.get_mut(&point1) == None {
            visited.insert(point1.clone(), 0);
            if point1 != "start" && point1 != "end" && point1 == point1.to_lowercase() {
                doubles.push(point1);
            }
        }
        if visited.get_mut(&point2) == None {
            visited.insert(point2.clone(), 0);
            if point2 != "start" && point2 != "end" && point2 == point2.to_lowercase() {
                doubles.push(point2);
            }
        }
    }

    let mut num = 0;
    let mut repeat = 0;
    number_of_paths("start".to_string(), "end", &mut visited, &map, &mut repeat);
    for double in doubles {
        number_of_paths2(
            "start".to_string(),
            "end",
            &mut visited,
            &map,
            &double,
            &mut num,
        );
        num -= repeat;
    }
    num += repeat;

    println!("Day 12 Part 2: {}", num);
}
