use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn part_one(lines: Vec<String>) -> Result<isize, &'static str> {
    let mut valid_count = 0;

    for line in lines {
        let by_spaces: Vec<&str> = line.split(" ").collect();
        if by_spaces.len() != 3 {
            return Err("Row data is sus");
        }

        let policy: Vec<&str> = by_spaces[0].split("-").collect();
        if policy.len() != 2 {
            return Err("Policy data is sus");
        }
        let min = policy[0]
            .parse::<usize>()
            .expect("failed to parse min value");
        let max = policy[1]
            .parse::<usize>()
            .expect("failed to parse min value");

        let letter = by_spaces[1].split(":").collect::<Vec<&str>>()[0];
        let pw = by_spaces[2];

        println!("{} {}", pw, letter);
        let times_matched = pw.matches(letter).count();
        println!("{}", times_matched);
        if times_matched >= min && times_matched <= max {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

fn part_two(lines: Vec<String>) -> Result<isize, &'static str> {
    let mut valid_count = 0;

    for line in lines {
        let by_spaces: Vec<&str> = line.split(" ").collect();
        if by_spaces.len() != 3 {
            return Err("Row data is sus");
        }

        let policy: Vec<&str> = by_spaces[0].split("-").collect();
        if policy.len() != 2 {
            return Err("Policy data is sus");
        }
        let i1 = policy[0]
            .parse::<usize>()
            .expect("failed to parse min value");
        let i2 = policy[1]
            .parse::<usize>()
            .expect("failed to parse min value");

        let letter = by_spaces[1].split(":").collect::<Vec<&str>>()[0]
            .chars()
            .next()
            .unwrap();
        let pw: &str = by_spaces[2];

        if (pw.chars().nth(i1 - 1).unwrap() == letter) ^ (pw.chars().nth(i2 - 1).unwrap() == letter)
        {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

pub fn solve(input: &str) -> Result<isize, &'static str> {
    let file = File::open(input).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    return part_two(lines);
}
