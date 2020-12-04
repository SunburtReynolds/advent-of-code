use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn solve(input: &str) -> Result<isize, &'static str> {
    let file = File::open(input).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // do math here
    for outer in 0..(lines.len() - 2) {
        for middle in outer..(lines.len() - 1) {
            for inner in outer..lines.len() {
                let outer_val = lines[outer]
                    .parse::<isize>()
                    .expect("failed to parse outer integer value");
                let middle_val = lines[middle]
                    .parse::<isize>()
                    .expect("failed to parse middle integer value");
                let inner_val = lines[inner]
                    .parse::<isize>()
                    .expect("failed to parse inner integer value");
                let sum = outer_val + middle_val + inner_val;
                if sum == 2020 {
                    println!("found them {} {} {}", outer_val, middle, inner_val);
                    return Ok(outer_val * middle_val * inner_val);
                }
            }
        }
    }

    Err("could not find correct combination of values")
}
