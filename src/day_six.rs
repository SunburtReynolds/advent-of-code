use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

// fn count_or_yeses(lines: &Vec<&String>) -> isize {
//     let mut set: HashSet<char> = HashSet::new();

//     for line in lines {
//         for char in line.chars() {
//             set.insert(char);
//         }
//     }

//     // not ideal
//     return set.len() as isize;
// }

fn count_and_yeses(lines: &Vec<&String>) -> isize {
    let mut map: HashMap<char, isize> = HashMap::new();

    for line in lines {
        for char in line.chars() {
            map.entry(char).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    let must_be = lines.len();
    let mut and_count: isize = 0;
    for v in map.values() {
        if *v == must_be as isize {
            and_count += 1;
        }
    }

    and_count
}

fn part_one(lines: Vec<String>) -> Result<isize, &'static str> {
    let mut distinct_count: isize = 0;
    let mut group_lines: Vec<&String> = vec![];

    for (index, line) in lines.iter().enumerate() {
        if line.len() > 0 {
            group_lines.push(line);
        }

        if line.len() == 0 || index == lines.len() - 1 {
            distinct_count += count_and_yeses(&group_lines);
            group_lines.clear();
        }
    }

    Ok(distinct_count)
}

pub fn solve(input: &str) -> Result<isize, &'static str> {
    let file = File::open(input).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    return part_one(lines);
}
