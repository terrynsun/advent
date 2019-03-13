#![allow(dead_code)]
extern crate regex;
extern crate chrono;
extern crate time;

use std::fs;
use std::collections::{HashSet, HashMap};
use regex::Regex;
use chrono::prelude::*;

mod helpers;

fn one_a(data: &Vec<i32>) -> i32 {
    let mut total = 0;

    for item in data {
        total += item;
    }

    total
}

fn one_b(data: &Vec<i32>) -> i32 {
    let mut map = HashSet::new();
    let mut total = 0;

    loop {
        for item in data {
            total += item;

            if map.contains(&total) {
                return total;
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

    doubles
}

fn three_b(data: &Vec<Claim>) -> i32 {
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

    return *pristine_claims.iter().next().unwrap_or(&0);
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

fn four_a(data: &HashMap<u32, Vec<(NaiveDateTime, NaiveDateTime)>>) -> u32 {
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
             sleepiest_guard * sleepiest_minute.minute());

    sleepiest_guard * sleepiest_minute.minute()
}

fn four_b(data: &HashMap<u32, Vec<(NaiveDateTime, NaiveDateTime)>>) -> u32 {
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
             sleepiest_guard * max_sleepiest_minute);

    sleepiest_guard * max_sleepiest_minute
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

fn five_a(data: &String) -> usize {
    let mut polymer = data.clone().into_bytes();

    fully_react_polymer(&mut polymer);

    polymer.len()
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

fn five_b(data: &String) -> usize {
    let polymer = data.clone().into_bytes();

    let mut removed: HashSet<u8> = HashSet::new();

    let mut shortest = data.len();

    for c in polymer.iter() {
        if char::from(*c).is_lowercase() && !removed.contains(c) {
            removed.insert(*c);

            let mut new_polymer = polymer_copy_without_char(&polymer, *c);
            fully_react_polymer(&mut new_polymer);
            shortest = std::cmp::min(new_polymer.len(), shortest);
        }
    }

    shortest
}

fn six_a(data: &Vec<(i32, i32)>) -> u32 {
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
    max_count
}

fn six_b(data: &Vec<(i32, i32)>) -> u32 {
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

    in_distance
}

fn seven_a(data: &HashMap<char, Vec<char>>) -> String {
    let mut done = String::new();
    let steps = data.keys().len();

    while done.len() != steps {
        let mut doable = Vec::new();
        for (step, reqs) in data {
            if done.contains(*step) {
                continue;
            }

            let mut possible = true;
            for req in reqs {
                if !done.contains(*req) {
                    possible = false;
                }
            }
            if possible {
                doable.push(*step);
            }
        }

        // pick the first available step alphabetically
        doable.sort_unstable();

        let step: char = doable.remove(0);
        done.push(step);
    }
    done
}

fn seven_b(data: &HashMap<char, Vec<char>>) -> String {
    let mut done = String::new();
    let steps = data.keys().len();

    let mut started = Vec::new();
    let mut completion_schedule: HashMap<u32, Vec<char>> = HashMap::new();
    let mut second = 0u32;
    let mut workers_free = 5;

    while done.len() != steps {
        second += 1;
        // Mark things done if they have finished by now.
        for step in completion_schedule.get(&second).unwrap_or(&Vec::new()) {
            done.push(*step);
            workers_free += 1;
        }

        // Find things available to work on.
        let mut doable = Vec::new();
        for (step, reqs) in data {
            if done.contains(*step) || started.contains(step) {
                continue;
            }

            let mut possible = true;
            for req in reqs {
                if !done.contains(*req) {
                    possible = false;
                }
            }
            if possible {
                doable.push(*step);
            }
        }

        // Pick the first available step, alphabetically.
        doable.sort_unstable();

        // Get to work.
        while workers_free > 0 && doable.len() > 0 {
            let step: char = doable.remove(0);
            let duration = (step as u32) - ('A' as u32) + 1 + 60;
            completion_schedule.entry(duration + second).or_insert(Vec::new()).push(step);
            started.push(step);
        }
    }

    // All steps actually finished one second ago.
    format!("{}", second - 1)
}

fn parse_nodes_a(data: &Vec<i32>, init_idx: usize) -> (i32, usize) {
    // Returns metadata sum, number of values consumed.
    let num_children = data[init_idx];
    let num_metadata = data[init_idx + 1] as usize;

    let mut sum = 0;
    let mut idx = init_idx + 2;

    for _ in 0 .. num_children {
        let (child_value, new_idx) = parse_nodes_a(data, idx);
        idx = new_idx;
        sum += child_value
    }

    for val in &data[idx .. idx + num_metadata] {
        sum += val;
    }

    idx += num_metadata;
    return (sum, idx);
}

fn eight_a(data: &Vec<i32>) -> i32 {
    let (sum, idx) = parse_nodes_a(data, 0);

    assert!(idx == data.len());

    sum
}

fn parse_nodes_b(data: &Vec<i32>, init_idx: usize) -> (i32, usize) {
    // Returns node value, number of values consumed.
    let num_children = data[init_idx];
    let num_metadata = data[init_idx + 1] as usize;

    let mut total = 0;
    let mut idx = init_idx + 2;

    let mut children = Vec::new();

    if num_children == 0 {
        // The value of a node without children is the sum of its metadata
        for val in &data[idx .. idx + num_metadata] {
            total += val;
        }
    } else {
        // Otherwise it is the sum of the values of the children nodes indicated by the metadata.

        // Parse children.
        for _ in 0 .. num_children {
            let (child_value, new_idx) = parse_nodes_b(data, idx);
            idx = new_idx;
            children.push(child_value);
        }

        // Sum values.
        for &meta_val in &data[idx .. idx + num_metadata] {
            if meta_val == 0 || (meta_val as usize) > children.len() {
                continue;
            } else {
                total += children[(meta_val as usize)-1];
            }
        }
    }

    idx += num_metadata;
    return (total, idx);
}

fn eight_b(data: &Vec<i32>) -> i32 {
    let (sum, idx) = parse_nodes_b(data, 0);

    assert!(idx == data.len());

    sum
}

fn nine_a(data: &(u32, u32)) -> u32 {
    let players = data.0;
    let marbles = data.1;

    let mut scores: HashMap<u32, u32> = HashMap::new();

    let mut current_player = 0;
    let mut idx = 1;

    // The circle starts with one marble.
    // Clockwise goes to the right.
    let mut circle = vec![0, 2, 1];

    for m in 3 .. marbles {
        if m % 100000 == 0 {
            println!("{}\t{}%", m, m as f32/marbles as f32);
        }
        current_player = (current_player + 1) % players;
        if m % 23 == 0 {
            idx = (idx + circle.len() - 7) % circle.len();
            let removed = circle.remove(idx);

            *scores.entry(current_player).or_insert(0) += (m + removed) as u32;
        } else {
            idx = (idx + 2) % circle.len();
            if idx > circle.len() {
                idx %= circle.len();
            }
            circle.insert(idx, m);
        }
    }

    let mut scores_sorted = scores.values().collect::<Vec<&u32>>();
    scores_sorted.sort_unstable();
    *scores_sorted.pop().unwrap()
}

fn nine_b(data: &(u32, u32)) -> u32 {
    // You're supposed to do this with linked lists, but I just let it run for... about an hour.
    nine_a(&(data.0, data.1*100))
}

// position (x, y); velocity (x, y)
// position: x: left = negative
// position: y: up = negative; down = positive
fn ten_a(data: &Vec<(i32, i32, i32, i32)>) -> String {
    let mut positions: Vec<(i32, i32)> = data.into_iter().map( |(x, y, _, _)| (*x, *y)).collect();
    let velocities: Vec<(i32, i32)> = data.into_iter().map( |(_, _, dx, dy)| (*dx, *dy)).collect();

    fn draw(positions: &Vec<(i32, i32)>) -> Option<String> {
        // create board
        let (xmin, xmax, ymin, ymax) = helpers::minmax(positions);
        let ylen = (ymax - ymin + 1) as usize;
        let xlen = (xmax - xmin + 1) as usize;

        // don't waste time rendering boards that are too big
        if ylen > 100 || xlen > 100 {
            return None;
        }

        let mut board = Vec::with_capacity(ylen);
        for _ in 0 .. ylen {
            let mut v = Vec::with_capacity(xlen);
            for _ in 0 .. xlen {
                v.push('.');
            }
            board.push(v);
        }

        // place coordinates
        for (x, y) in positions {
            //println!("x: {}; min-max: ({}, {}), len: {}", x, xmin, xmax, board[0].len());
            //println!("y: {}; min-max: ({}, {}), len: {}", y, ymin, ymax, board[0].len());
            board[(y - ymin) as usize][(x - xmin) as usize] = '#';
        }

        let lines: Vec<String> = board.into_iter().map(|line| line.into_iter().collect()).collect();
        Some(lines.join("\n"))
    }

    // update
    fn update(positions: &mut Vec<(i32, i32)>, velocities: &Vec<(i32, i32)>, steps: i32) {
        for i in 0 .. positions.len() {
            let (x, y) = positions[i];
            let (dx, dy) = velocities[i];
            positions[i] = (x + (dx * steps), y + (dy * steps));
        }
    }

    let seed = 10000;
    update(&mut positions, &velocities, seed);

    // max length of board that can be generated
    let mut prev_ylen = 100 * 100;
    let mut prev_board = String::new();

    for i in 0 .. 20000 {
        // should pass this into the other function but I don't really feel like it
        let (_, _, ymin, ymax) = helpers::minmax(&positions);
        let new_ylen = (ymax - ymin + 1) as usize;
        if new_ylen > prev_ylen {
            println!("?? {}", seed+i-1);
            return prev_board;
        } else {
            println!("!! {}", i);
            let board = draw(&positions);
            if let Some(s) = board {
                prev_board = s;
            }
            prev_ylen = new_ylen;
        }

        update(&mut positions, &velocities, 1);
    }
    println!("{}", 33333);
    String::new()
}

fn eleven_compute_grid(serial: u32, size: usize) -> Vec<Vec<i32>> {
    // get the power value of a single cell
    fn calc(x: i32, y: i32, serial: i32) -> i32 {
        let rack_id = x + 10;
        let mut power = rack_id * y;
        power = power + serial;
        power = power * rack_id;

        power = power / 100;
        power = power % 10;
        power = power - 5;
        power
    }

    let mut grid = Vec::new();
    for x in 0 .. size {
        let mut row = Vec::new();
        for y in 0 .. size {
            row.push(calc(x as i32, y as i32, serial as i32))
        }
        grid.push(row);
    }
    grid
}

fn eleven_square_power_level(grid: &Vec<Vec<i32>>, x: usize, y: usize, size: usize) -> i32 {
    let mut sum = 0;
    for i in 0 .. size {
        for k in 0 .. size {
            sum += grid[(x + i) as usize][(y + k) as usize];
        }
    }
    sum
}

fn eleven_a(serial: &u32) -> String {
    let size = 300usize;
    let grid = eleven_compute_grid(*serial, size);

    let mut max_idx = (0, 0);
    let mut max = 0;

    let square_size = 3;

    for x in 0 .. size-square_size {
        for y in 0 .. size-square_size {
            let sum = eleven_square_power_level(&grid, x, y, square_size);
            if sum > max {
                max = sum;
                max_idx = (x, y);
            }
        }
    }
    format!("{:?}, max={}", max_idx, max)
}

fn eleven_square_outer_level(grid: &Vec<Vec<i32>>, x: usize, y: usize, size: usize) -> i32 {
    // size is off by one as it's "inclusive"
    // size = 0 means the value of that grid cell
    let size = size - 1;

    let mut sum = 0;
    for i in 0 .. size {
        sum += grid[(x + size) as usize][(y + i) as usize];
        sum += grid[(x + i) as usize][(y + size) as usize];
    }
    sum += grid[(x + size) as usize][(y + size) as usize];
    sum
}

fn eleven_b(serial: &u32) -> String {
    let grid_size = 300usize;
    let grid = eleven_compute_grid(*serial, grid_size);

    let mut vals = vec![vec![0; grid_size]; grid_size];
    let mut max_idx = (0, 0);
    let mut max = 0;
    let mut max_size = 0;

    for square_size in 1 .. 300 {
        for x in 0 .. grid_size-square_size {
            for y in 0 .. grid_size-square_size {
                let change = eleven_square_outer_level(&grid, x, y, square_size);
                vals[x][y] += change;
                if vals[x][y] > max {
                    max = vals[x][y];
                    max_idx = (x, y);
                    max_size = square_size;
                }
            }
        }
    }
    format!("{:?}, size={}, max={}", max_idx, max_size, max)
}

struct Puzzle<T, R> {
    // T is the type that the input gets parsed into
    // R is the type that the answer comes in
    name: &'static str,
    parts: Vec<fn(&T) -> R>,
    delimiter: char,
    preprocess: fn(Vec<String>) -> T,
}

fn solve_puzzle<T, R>(p: Puzzle<T, R>) where R: std::fmt::Display {
    let debug = false;
    let dir = "inputs";

    let filename = if debug { "test.txt".to_string() } else { format!("{}/{}.txt", dir, p.name) };
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.trim_end()
        .split(p.delimiter)
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
        delimiter: '\n',
        preprocess: |v: Vec<String>|
            v.iter()
            .map(|x: &String| { (*x).parse::<i32>().unwrap_or(0) })
            .collect()
    };

    let two = Puzzle {
        name: "two",
        parts: vec![two_a, two_b],
        delimiter: '\n',
        preprocess: |x| x
    };

    let three = Puzzle {
        name: "three",
        parts: vec![three_a, three_b],
        delimiter: '\n',
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
        delimiter: '\n',
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
        delimiter: '\n',
        preprocess: |mut x: Vec<String>| x.pop().unwrap(),
    };

    let six = Puzzle {
        name: "six",
        parts: vec![six_a, six_b],
        delimiter: '\n',
        preprocess: |v: Vec<String>| {
            v.iter().map(|x| {
                let vals: Vec<&str> = x.split(',').collect();
                let x = vals[0].parse().unwrap();
                let y = vals[1].trim_start().parse().unwrap();
                (x, y)
            }).collect()
        }
    };

    let seven = Puzzle {
        name: "seven",
        parts: vec![seven_a, seven_b],
        delimiter: '\n',
        preprocess: |v: Vec<String>| {
            let mut dag = HashMap::new();

            for line in v {
                let bytes = line.into_bytes();
                let req = char::from(bytes[5]);
                let step = char::from(bytes[36]);

                dag.entry(step).or_insert(Vec::new()).push(req);
                dag.entry(req).or_insert(Vec::new());
            }

            dag
        }
    };

    let eight = Puzzle {
        name: "eight",
        parts: vec![eight_a, eight_b],
        delimiter: ' ',
        preprocess: |v: Vec<String>| {
            v.iter().map(|x| x.parse::<i32>().unwrap()).collect()
        }
    };

    let nine = Puzzle {
        name: "nine",
        parts: vec![nine_a, nine_b],
        delimiter: '\n',
        preprocess: |v: Vec<String>| {
            let re = Regex::new(r"(\d*) players; last marble is worth (\d*) points").unwrap();

            let caps = re.captures(&v[0]).unwrap();

            (caps[1].parse().unwrap(), caps[2].parse::<u32>().unwrap())
        }
    };

    let ten = Puzzle {
        name: "ten",
        parts: vec![ten_a],
        delimiter: '\n',
        preprocess: |v: Vec<String>| {
            // position=< 9,  1> velocity=< 0,  2>
            let re = Regex::new(r"position=<\s*(-?\d*),\s*(-?\d*)> velocity=<\s*(-?\d*),\s*(-?\d*)>").unwrap();

            v.iter().map(|line| {
                let caps = re.captures(line).unwrap();

                (
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap()
                )
            }).collect()
        }
    };

    let eleven = Puzzle {
        name: "eleven",
        parts: vec![eleven_a, eleven_b],
        delimiter: '\n',
        preprocess: |v: Vec<String>| v[0].parse().unwrap_or(0)
    };

    solve_puzzle(eleven);
}
