use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1(lines: &Vec<u32>) -> u32 {
    let mut previous: u32 = u32::MAX;
    let mut increases: u32 = 0;
    for num in lines {
        if num > &previous {
            increases += 1;
        }
        previous = *num;
    }
    increases
}

fn part_2(lines: &Vec<u32>) -> u32 {
    let mut previous_scans_sum = 0;
    let mut scans = vec![];
    let mut increases: u32 = 0;

    for num in lines {
        if scans.len() < 3 {
            scans.push(num.clone());
            continue;
        }

        let current_scans_sum = scans.iter().copied().reduce(|acc, num| acc + num).unwrap();
        if current_scans_sum > previous_scans_sum {
            increases += 1;
        }

        previous_scans_sum = current_scans_sum;
        scans.remove(0);
        scans.push(num.clone());
    }
    increases as u32
}

fn main() {
    let file = File::open("./problem_1_input.txt").expect("Cannot open file");
    let file_contents = BufReader::new(file);
    let file_lines: Vec<u32> = file_contents
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();
    println!("Part 1: {}", part_1(&file_lines));
    println!("Part 2: {}", part_2(&file_lines));
}
