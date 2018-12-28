#![allow(dead_code)]
extern crate regex;
extern crate chrono;
extern crate time;

use std::fs;
use std::collections::{HashSet, HashMap};
use regex::Regex;
use chrono::prelude::*;

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

enum ShiftAction {
    Start,
    Sleep,
    Wake,
}

struct Action {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    guard: u32,
    action: ShiftAction
}

fn get_sleepiest_minute(sleep_list: &Vec<(NaiveDateTime, NaiveDateTime)>) -> (NaiveTime, u32) {
    let mut times_slept_per_min: HashMap<NaiveTime, u32> = HashMap::new();

    for (sleep, wake) in sleep_list {
        let mut intermediate = *sleep;
        while intermediate < *wake {
            *times_slept_per_min.entry(intermediate.time()).or_insert(0) += 1;
            intermediate += time::Duration::minutes(1);
        }
    }

    let mut sleepiest_minute = NaiveTime::from_hms(0, 0, 0);
    let mut max_times_slept = 0;
    for (minute, times) in times_slept_per_min {
        if times > max_times_slept {
            max_times_slept = times;
            sleepiest_minute = minute;
        }
    }
    (sleepiest_minute, max_times_slept)
}

fn four_a(data: &HashMap<u32, Vec<(NaiveDateTime, NaiveDateTime)>>) -> String {
    let mut minutes_slept: HashMap<u32, i64> = HashMap::new();

    for (guard, sleeps) in data {
        for (start, end) in sleeps {
            let duration = *end - *start;
            *minutes_slept.entry(*guard).or_insert(0) += duration.num_minutes();
        }
    }

    let mut sleepiest_guard = 0;
    let mut max_minutes = 0;
    for (guard, minutes) in minutes_slept {
        if minutes > max_minutes {
            max_minutes = minutes;
            sleepiest_guard = guard;
        }
    }

    let (sleepiest_minute, _) = get_sleepiest_minute(data.get(&sleepiest_guard).unwrap());
    format!("{} x {} = {}", sleepiest_guard, sleepiest_minute.minute(),
             sleepiest_guard * sleepiest_minute.minute())
}

fn four_b(data: &HashMap<u32, Vec<(NaiveDateTime, NaiveDateTime)>>) -> String {
    let mut sleepiest_guard = 0;
    let mut max_sleepiest_minute = 0;
    let mut max_times_slept = 0;

    for (guard, sleeps) in data {
        let (sleepiest_minute, times_slept) = get_sleepiest_minute(sleeps);
        if times_slept > max_times_slept {
            sleepiest_guard = *guard;
            max_sleepiest_minute = sleepiest_minute.minute();
            max_times_slept = times_slept;
        }
    }
    format!("{} x {} = {}", sleepiest_guard, max_sleepiest_minute,
             sleepiest_guard * max_sleepiest_minute)
}

fn react_polymer_once(p: &mut Vec<u8>) {
    if p.len() == 0 {
        return
    }

    for i in 0..p.len()-1 {
        if (p[i] as i8 - p[i+1] as i8).abs() == 32 {
            p.remove(i);
            p.remove(i);
            return
        }
    }
}

fn fully_react_polymer(p: &mut Vec<u8>) {
    let mut length = p.len();

    loop {
        react_polymer_once(p);
        let new_length = p.len();
        if length == new_length {
            return
        } else {
            length = new_length;
        }
    }
}

fn five_a(data: &String) -> String {
    let mut polymer = data.clone().into_bytes();

    fully_react_polymer(&mut polymer);

    format!("{}", polymer.len())
}

fn polymer_copy_without_char(p: &Vec<u8>, c: u8) -> Vec<u8>{
    let mut new = p.clone();
    for i in (0..new.len()).rev() {
        if new[i] == c || new[i] == (c - 32) {
            new.remove(i);
        }
    }
    new
}

fn five_b(data: &String) -> String {
    let polymer = data.clone().into_bytes();

    let mut removed: HashSet<u8> = HashSet::new();

    let mut shortest = data.len();

    for c in polymer.iter() {
        if char::from(*c).is_lowercase() && !removed.contains(c) {
            let mut new_polymer = polymer_copy_without_char(&polymer, *c);
            fully_react_polymer(&mut new_polymer);
            if new_polymer.len() < shortest {
                shortest = new_polymer.len();
            }
            removed.insert(*c);
        }
    }

    format!("{}", shortest)
}

fn six_a(data: &Vec<(i32, i32)>) -> String {
    // coordinates at the outside edge of the grid have infinite space and don't count
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (data[0].0, 0, data[0].1, 0);

    for (x, y) in data {
        x_min = std::cmp::min(*x, x_min);
        x_max = std::cmp::max(*x, x_max);
        y_min = std::cmp::min(*y, y_min);
        y_max = std::cmp::max(*y, y_max);
    }

    let mut area_counter: HashMap<usize, u32> = HashMap::new();

    for y in y_min .. y_max+1 {
        for x in x_min .. x_max+1 {
            let mut closest = 0usize;
            let mut min_distance = x_max + y_max;
            let mut tie = false;

            for i in 0 .. data.len() {
                let (cx, cy) = data[i];
                let distance = (cx - x).abs() + (cy - y).abs();
                if distance < min_distance {
                    closest = i as usize;
                    min_distance = distance;
                    tie = false;
                } else if distance == min_distance {
                    tie = true;
                }
            }

            if !tie {
                *area_counter.entry(closest).or_insert(0) += 1;
            }
        }
    }

    let mut max_count = 0;
    for (idx, count) in area_counter {
        let (x, y) = data[idx];
        if !(x == x_min || x == x_max || y == y_min || y == y_max) {
            max_count = std::cmp::max(count, max_count);
        }
    }
    format!("{}", max_count)
}

fn six_b(data: &Vec<(i32, i32)>) -> String {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (data[0].0, 0, data[0].1, 0);

    for (x, y) in data {
        x_min = std::cmp::min(*x, x_min);
        x_max = std::cmp::max(*x, x_max);
        y_min = std::cmp::min(*y, y_min);
        y_max = std::cmp::max(*y, y_max);
    }

    let allowed_distance = 10_000;
    let mut in_distance = 0;

    for y in y_min .. y_max+1 {
        for x in x_min .. x_max+1 {

            let mut total_distance = 0;

            for i in 0 .. data.len() {
                let (cx, cy) = data[i];
                let distance = (cx - x).abs() + (cy - y).abs();

                total_distance += distance;
            }
            if total_distance < allowed_distance {
                in_distance += 1;
            }
        }
    }

    format!("{}", in_distance)
}

struct Puzzle<T> {
    // T is the type that the input gets parsed into
    name: &'static str,
    preprocess: fn(Vec<String>) -> T,
    parts: Vec<fn(&T) -> String>,
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

    let four = Puzzle {
        name: "four",
        parts: vec![four_a, four_b],
        preprocess: |mut v: Vec<String>| {
            v.sort_unstable();
            let re = Regex::new(r"\[(.*)-(.*)-(.*) (.*):(.*)\] (.*)").unwrap();
            let shift = Regex::new(r"Guard #(\d*) begins shift").unwrap();

            let mut guard = 0;

            let mut sleeps: HashMap<u32, Vec<(NaiveDateTime, NaiveDateTime)>> = HashMap::new();

            let mut start_sleeping = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);

            for line in v {
                let caps = re.captures(&line).unwrap();

                let action_str = caps[6].to_string();

                let year = caps[1].parse().unwrap();
                let month = caps[2].parse().unwrap();
                let day = caps[3].parse().unwrap();
                let hour = caps[4].parse().unwrap();
                let min = caps[5].parse().unwrap();

                if action_str.contains("wakes up") {
                    let end_sleeping = NaiveDate::from_ymd(year, month, day).and_hms(hour, min, 0);

                    sleeps.entry(guard).or_insert(Vec::new()).push((start_sleeping, end_sleeping));
                } else if action_str.contains("falls asleep") {
                    start_sleeping = NaiveDate::from_ymd(year, month, day).and_hms(hour, min, 0);
                } else {
                    guard = shift.captures(&action_str).unwrap()[1].parse().unwrap();
                }
            }
            sleeps
        }
    };

    let five = Puzzle {
        name: "five",
        parts: vec![five_a, five_b],
        preprocess: |mut x: Vec<String>| x.pop().unwrap(),
    };

    let six = Puzzle {
        name: "six",
        parts: vec![six_a, six_b],
        preprocess: |v: Vec<String>| {
            v.iter().map(|x| {
                let vals: Vec<&str> = x.split(',').collect();
                let x = vals[0].parse().unwrap();
                let y = vals[1].trim_start().parse().unwrap();
                (x, y)
            }).collect()
        }
    };
    solve_puzzle(six);
}
