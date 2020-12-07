use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

struct Slope {
    x: usize,
    y: usize,
}

fn next_x(curr_x: usize, slope: &Slope, line_len: usize) -> usize {
    (curr_x + slope.x) % line_len
}

fn next_y(curr_y: usize, slope: &Slope) -> usize {
    curr_y + slope.y
}

fn part_one(lines: Vec<String>, slope: Slope) -> Result<isize, &'static str> {
    let num_rows = lines.len();
    println!("num rows {}", num_rows);
    let mut tree_count = 0;
    let mut curr_y = 0;
    let mut curr_x = 0;

    while curr_y < num_rows {
        let line = lines.get(curr_y).expect("Failed to get line at curr_y");
        let char = line
            .chars()
            .nth(curr_x)
            .expect("Could not get char at index curr_x");

        if curr_y + slope.y > num_rows {
            println!("NEAR THE END {} {}", curr_x, curr_y)
        }

        if char == '#' {
            tree_count += 1;
        }

        curr_y = next_y(curr_y, &slope);
        curr_x = next_x(curr_x, &slope, line.len());
    }

    println!("fin {}", curr_y);

    Ok(tree_count)
}

pub fn solve(input: &str) -> Result<isize, &'static str> {
    let file = File::open(input).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let slope = Slope { x: 1, y: 2 };

    return part_one(lines, slope);
}

// 1, 1 - 63
// 3, 1 - 254
// 5, 1 - 62
// 7, 1 - 56
// 1, 2 - 30
// 1666768320
