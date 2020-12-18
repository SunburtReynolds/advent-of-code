use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    isize::MAX,
};

struct Position {
    row: isize,
    column: isize,
}

impl Position {
    fn to_id(self) -> isize {
        self.row * 8 + self.column
    }
}

const MAX_ROW: isize = 127;
const MAX_COLUMN: isize = 7;

fn middle(from: isize, to: isize) -> isize {
    (from + to) / 2
}

fn calculate_val(chars: Vec<char>, from_val: isize, to_val: isize) -> isize {
    if chars.len() == 0 {
        return from_val;
    }

    let next_char = chars
        .iter()
        .next()
        .expect("Attempted to take a char that doesn't exist");

    let chars_left_count = chars.len() - 1;
    let chars_left: Vec<char> = chars
        .iter()
        .skip(1)
        .take(chars_left_count)
        .map(|c| *c)
        .collect();

    let middle = middle(from_val, to_val);
    match next_char {
        'F' => {
            return calculate_val(chars_left, from_val, middle);
        }
        'L' => {
            return calculate_val(chars_left, from_val, middle);
        }
        'B' => {
            return calculate_val(chars_left, middle + 1, to_val);
        }
        'R' => {
            return calculate_val(chars_left, middle + 1, to_val);
        }
        &_ => return 0,
    }
}

fn calculate_position(line: String) -> Position {
    let row_chars: Vec<char> = line.chars().take(7).collect();
    let column_chars: Vec<char> = line.chars().skip(7).take(3).collect();
    let row = calculate_val(row_chars, 0, MAX_ROW);
    let column = calculate_val(column_chars, 0, MAX_COLUMN);
    Position { row, column }
}

// fn part_one(lines: Vec<String>) -> Result<isize, &'static str> {
//     let mut largest_seat_id = 0;

//     for line in lines {
//         if line.len() != 10 {
//             return Err("Invalid line input");
//         }

//         let position = calculate_position(line);
//         let id = position.to_id();
//         println!("Seat id: {}", id);
//         if id > largest_seat_id {
//             largest_seat_id = id;
//         }
//     }

//     Ok(largest_seat_id)
// }

fn part_two(lines: Vec<String>) -> Result<isize, &'static str> {
    let mut largest_seat_id: isize = 0;
    let mut smallest_seat_id: isize = MAX;
    let mut seat_id_set: HashSet<String> = HashSet::new();

    for line in lines {
        if line.len() != 10 {
            return Err("Invalid line input");
        }

        let position = calculate_position(line);
        let id = position.to_id();

        if id > largest_seat_id {
            largest_seat_id = id;
        }
        if id < smallest_seat_id {
            smallest_seat_id = id;
        }

        seat_id_set.insert(id.to_string());
    }

    println!("Smallest id: {}", smallest_seat_id);
    println!("Largest id: {}", largest_seat_id);

    let lower_bound = smallest_seat_id + 1;
    let upper_bound = largest_seat_id; // loop exclusive

    for possible_id in lower_bound..upper_bound {
        let has_id = seat_id_set.contains(&possible_id.to_string());
        let has_lower = seat_id_set.contains(&(possible_id - 1).to_string());
        let has_upper = seat_id_set.contains(&(possible_id + 1).to_string());
        if !has_id && has_upper && has_lower {
            return Ok(possible_id);
        }
    }

    Err("Could not find a valid ID")
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
