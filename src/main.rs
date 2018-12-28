#![allow(dead_code)]
extern crate regex;

use std::fs;
use std::collections::{HashSet, HashMap};
use regex::Regex;

fn one_a(data: &Vec<i32>) -> String {
    let mut total = 0;

    for item in data {
        total += item;
    }

    return format!("{}", total);
}

fn one_b(data: &Vec<i32>) -> String {
    let mut map = HashSet::new();
    let mut total = 0;

    loop {
        for item in data {
            total += item;

            if map.contains(&total) {
                return format!("{}", total);
            }

            map.insert(total);
        }
    }
}

fn two_a(data: &Vec<String>) -> String {
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

    format!("{}", has_two * has_three)
}

fn two_b(data: &Vec<String>) -> String {
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

    String::new()
}

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn three_a(data: &Vec<Claim>) -> String {
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

    format!("{}", doubles)
}

fn three_b(data: &Vec<Claim>) -> String {
    let mut fabric = HashMap::new();

    let mut pristine_claims = HashSet::new();

    for claim in data {
        pristine_claims.insert(claim.id);

        for i in claim.x .. claim.x+claim.width {
            for j in claim.y .. claim.y+claim.height {
                let claims = fabric.entry((i, j)).or_insert(HashSet::new());
                claims.insert(claim.id);

                if claims.len() > 1 {
                    for tainted_claim in claims.iter() {
                        pristine_claims.remove(tainted_claim);
                    }
                }
            }
        }
    }

    for claim in pristine_claims.iter() {
        return format!("{}", claim);
    }

    "0".to_string()
}

struct Puzzle<T> {
    // T is the type that the input gets parsed into
    name: &'static str,
    preprocess: fn(Vec<String>) -> Vec<T>,
    parts: Vec<fn(&Vec<T>) -> String>,
}

fn solve_puzzle<T>(p: Puzzle<T>) {
    let debug = false;
    let dir = "inputs";

    let filename = if debug { "test.txt".to_string() } else { format!("{}/{}.txt", dir, p.name) };
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.trim_end()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    let data = (p.preprocess)(lines);

    for f in p.parts {
        println!("{}", f(&data));
    }
}

fn main() {
    #![allow(unused_variables)]
    let one = Puzzle {
        name: "one",
        parts: vec![one_a, one_b],
        preprocess: |v: Vec<String>|
            v.iter()
            .map(|x: &String| { (*x).parse::<i32>().unwrap_or(0) })
            .collect()
    };

    let two = Puzzle {
        name: "two",
        parts: vec![two_a, two_b],
        preprocess: |x| x
    };

    let three = Puzzle {
        name: "three",
        parts: vec![three_a, three_b],
        preprocess: |v: Vec<String>|
            v.iter()
            .map(|x: &String| {
                let re = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();
                let caps = re.captures(&x).unwrap();
                Claim { id: caps[1].parse().unwrap(),
                        x: caps[2].parse().unwrap(),
                        y: caps[3].parse().unwrap(),
                        width: caps[4].parse().unwrap(),
                        height: caps[5].parse().unwrap(),
                }
            })
            .collect()
    };
    solve_puzzle(three);
}
