use std::io::{Lines, BufReader, BufRead, Result};
use std::path::Path;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let range = input_range();
    solve(range.start, range.end);
}

#[derive(Copy, Debug, Clone)]
struct Range {
    start: i32,
    end: i32,
}

fn solve(from: i32, to: i32) {
    let mut count = 0;
    println!("solving from {} to {}", from, to);
    for i in from..to {
        let chars: Vec<char> = i.to_string().chars().collect();

        if !increasing(&chars) { continue; }
        if !has_pairs(&chars) { continue; }
        count += 1;
        println!("Testing {}", i);
    }
    println!("Solution 1: {}", count);
}

fn increasing(c: &Vec<char>) -> bool {
    if c[0] > c[1] { return false; }
    if c[1] > c[2] { return false; }
    if c[2] > c[3] { return false; }
    if c[3] > c[4] { return false; }
    if c[4] > c[5] { return false; }
    true
}

fn has_pairs(c: &Vec<char>) -> bool {
    if c[0] == c[1] && c[1] != c[2] { return true; }
    if c[1] == c[2] && c[0] != c[1] && c[2] != c[3] { return true; }
    if c[2] == c[3] && c[1] != c[2] && c[3] != c[4] { return true; }
    if c[3] == c[4] && c[2] != c[3] && c[4] != c[5] { return true; }
    if c[4] == c[5] && c[3] != c[4] { return true; }
    false
}


fn input_range() -> Range {
    let mut linevec: Vec<String> = vec!();
    if let Ok(lines) = lines("input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                linevec.push(line);
//                println!("{}", linevec[0]);
            }
        }
    }
    let split: Vec<&str> = linevec[0].split("-").collect();
    let start: i32 = split[0].parse().unwrap();
    let end: i32 = split[1].parse().unwrap();
    return Range { start, end };
}

fn lines<P>(file_name: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}