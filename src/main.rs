#![allow(dead_code)]
extern crate regex;

use std::fs;
use std::collections::{HashSet, HashMap};
use regex::Regex;

fn one_a(data: &Vec<i32>) -> i32 {
    let mut total = 0;

    for item in data {
        total += item;
        println!("> {}", total);
    }

    return total;
}

fn one_b(data: &Vec<i32>) -> i32 {
    let mut map = HashSet::new();
    let mut total = 0;

    loop {
        for item in data {
            total += item;
            println!("> {}", total);

            if map.contains(&total) {
                return total;
            }

            map.insert(total);
        }
    }
}

fn one() {
    let filename = "inputs/one.txt";
    let contents = fs::read_to_string(filename) .expect("Something went wrong reading the file");

    let lines = contents.trim_end().split('\n');

    let values: Vec<i32> = lines.map(|x| x.parse::<i32>().unwrap_or(0)).collect();

    println!("{}", one_a(&values));
    println!("{}", one_b(&values));
}

fn two_a<'a>(data: &Vec<&'a str>) -> i32 {
    let mut has_two = 0;
    let mut has_three = 0;

    for id in data {
        let mut counts = HashMap::new();
        let mut id_two = 0;
        let mut id_three = 0;
        for ch in id.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        for v in counts.values() {
            if *v == 2 {
                id_two = 1;
            } else if *v == 3 {
                id_three = 1;
            }
        }
        has_two += id_two;
        has_three += id_three;
    }

    println!("{}, {}", has_two, has_three);
    return has_two * has_three;
}

fn two_b<'a>(data: &Vec<&'a str>) -> String {
    for id in data {
        for id2 in data {
            let mut diff = 0;
            let mut same = String::new();

            for (a, b) in id.chars().zip(id2.chars()) {
                if a == b {
                    same.push(a);
                } else {
                    diff += 1;
                }
            }

            if diff == 1 {
                return same;
            }
        }
    }

    return String::new();
}

fn two() {
    let filename = "inputs/two.txt";
    let contents = fs::read_to_string(filename) .expect("Something went wrong reading the file");

    let lines = contents.trim_end().split('\n');

    let values: Vec<&str> = lines.collect();

    println!("{}", two_a(&values));
    println!("{}", two_b(&values));
}

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn three_a(data: &Vec<Claim>) -> i32 {
    let mut fabric = HashMap::new();

    for claim in data {
        for i in claim.x .. claim.x+claim.width {
            for j in claim.y .. claim.y+claim.height {
                *fabric.entry((i, j)).or_insert(0) += 1;
            }
        }
    }

    let mut doubles = 0;

    for v in fabric.values() {
        if *v > 1 {
            doubles += 1;
        }
    }

    return doubles;
}

fn three_b(data: &Vec<Claim>) -> String {
    let mut fabric = HashMap::new();
    let mut overlaps = HashMap::new();

    for claim in data {
        for i in claim.x .. claim.x+claim.width {
            for j in claim.y .. claim.y+claim.height {
                *fabric.entry((i, j)).or_insert(Vec::new()).insert(claim.id);
            }
        }
    }

    return String::new();
}

fn three() {
    let filename = "inputs/three.txt";
    let contents = fs::read_to_string(filename) .expect("Something went wrong reading the file");

    let lines = contents.trim_end().split('\n');

    let re = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

    let data: Vec<Claim> = lines.map(|x| {
        let caps = re.captures(x).unwrap();
        Claim { id: caps[1].parse().unwrap(),
                x: caps[2].parse().unwrap(),
                y: caps[3].parse().unwrap(),
                width: caps[4].parse().unwrap(),
                height: caps[5].parse().unwrap(),
        }
    }).collect();

    println!("{}", three_a(&data));
    println!("{}", three_b(&data));
}

fn main() {
    three();
}
