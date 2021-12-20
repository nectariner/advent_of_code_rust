use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1() -> i32 {
    let file = File::open("./problem_2_input.txt").expect("Cannot open file");
    let file_contents = BufReader::new(file);

    let mut forward: i32 = 0;
    let mut down: i32 = 0;

    let file_lines = file_contents.lines();
    for line in file_lines {
        let split = line.unwrap();
        let split: Vec<&str> = split.split_whitespace().collect();
        match split[0] {
            "forward" => forward += split[1].parse::<i32>().unwrap(),
            "down" => down += split[1].parse::<i32>().unwrap(),
            "up" => down -= split[1].parse::<i32>().unwrap(),
            _ => panic!("Error reading lines"),
        }
    }
    forward * down
}

fn part_2() -> i32 {
    let file = File::open("./problem_2_input.txt").expect("Cannot open file");
    let file_contents = BufReader::new(file);

    let mut forward: i32 = 0;
    let mut down: i32 = 0;
    let mut aim: i32 = 0;

    let file_lines = file_contents.lines();
    for line in file_lines {
        let split = line.unwrap();
        let split: Vec<&str> = split.split_whitespace().collect();
        match split[0] {
            "forward" => {
                forward += split[1].parse::<i32>().unwrap();
                down += split[1].parse::<i32>().unwrap() * aim;
            }
            "down" => {
                aim += split[1].parse::<i32>().unwrap();
            }
            "up" => {
                aim -= split[1].parse::<i32>().unwrap();
            }
            _ => panic!("Error reading lines"),
        }
    }
    forward * down
}

fn main() {
    println!("part_1 = {}", part_1());
    println!("part_2 = {}", part_2());
}
