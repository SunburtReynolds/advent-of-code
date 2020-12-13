use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    // fn is_valid(self) -> bool {
    //     self.byr.is_some()
    //         && self.iyr.is_some()
    //         && self.eyr.is_some()
    //         && self.hgt.is_some()
    //         && self.hcl.is_some()
    //         && self.ecl.is_some()
    //         && self.pid.is_some()
    // }

    fn byr_valid(&self) -> bool {
        return match &self.byr {
            Some(byr) => {
                if byr.len() != 4 {
                    return false;
                }
                let byr_num = byr.parse::<usize>().unwrap_or(0);
                return byr_num >= 1920 && byr_num <= 2002;
            }
            None => false,
        };
    }

    fn iyr_valid(&self) -> bool {
        return match &self.iyr {
            Some(iyr) => {
                if iyr.len() != 4 {
                    return false;
                }
                let iyr_num = iyr.parse::<usize>().unwrap_or(0);
                return iyr_num >= 2010 && iyr_num <= 2020;
            }
            None => false,
        };
    }

    fn eyr_valid(&self) -> bool {
        return match &self.eyr {
            Some(eyr) => {
                if eyr.len() != 4 {
                    return false;
                }
                let eyr_num = eyr.parse::<usize>().unwrap_or(0);
                return eyr_num >= 2020 && eyr_num <= 2030;
            }
            None => false,
        };
    }

    fn hgt_valid(&self) -> bool {
        return match &self.hgt {
            Some(hgt) => {
                if hgt.contains("cm") {
                    let cm_parts: Vec<&str> = hgt.split("cm").collect();
                    let dig_str = cm_parts[0];
                    let dig = dig_str.parse::<usize>().unwrap_or(0);
                    return dig >= 150 && dig <= 193;
                }

                if hgt.contains("in") {
                    let in_parts: Vec<&str> = hgt.split("in").collect();
                    let dig_str = in_parts[0];
                    let dig = dig_str.parse::<usize>().unwrap_or(0);
                    return dig >= 59 && dig <= 76;
                }

                return false;
            }
            None => false,
        };
    }

    fn hcl_valid(&self) -> bool {
        return match &self.hcl {
            Some(hcl) => {
                let chars: Vec<char> = hcl.chars().collect();
                if chars.len() != 7 {
                    return false;
                }

                if chars[0] != '#' {
                    return false;
                }

                let regex = Regex::new(r"([A-F]|[0-9]){6}").unwrap();
                return regex.is_match(hcl.to_uppercase().as_str());
            }
            None => false,
        };
    }

    fn ecl_valid(&self) -> bool {
        return match &self.ecl {
            Some(ecl) => match ecl.as_str() {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                &_ => false,
            },
            None => false,
        };
    }

    fn pid_valid(&self) -> bool {
        return match &self.pid {
            Some(pid) => {
                if pid.chars().count() != 9 {
                    return false;
                }

                return pid.parse::<usize>().map_or(false, |_| true);
            }
            None => false,
        };
    }

    fn is_valid_part_two(&self) -> bool {
        self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
    }
}

fn parse_passport(lines: &Vec<&String>) -> Passport {
    let mut passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
    };
    for line in lines {
        let pairs = line.split(" ");
        for pair in pairs {
            let mut parts = pair.split(":");
            let key = parts.nth(0).expect("Failed to get key of pair");
            let value = String::from(parts.nth(0).expect("Failed to get value of pair"));
            match key {
                "byr" => passport.byr = Some(value),
                "iyr" => passport.iyr = Some(value),
                "eyr" => passport.eyr = Some(value),
                "hgt" => passport.hgt = Some(value),
                "hcl" => passport.hcl = Some(value),
                "ecl" => passport.ecl = Some(value),
                "pid" => passport.pid = Some(value),
                "cid" => passport.cid = Some(value),
                &_ => panic!("There was an unexpected key"),
            }
        }
    }
    passport
}

fn part_one(lines: Vec<String>) -> Result<isize, &'static str> {
    let mut valid_passports_count = 0;
    let mut passport_lines: Vec<&String> = vec![];
    let mut total_count = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.len() > 0 {
            passport_lines.push(line);
        }

        if line.len() == 0 || index == lines.len() - 1 {
            total_count += 1;
            let passport = parse_passport(&passport_lines);
            if passport.is_valid_part_two() {
                valid_passports_count += 1;
            }
            passport_lines.clear();
        }
    }

    println!("Total count: {}", total_count);

    Ok(valid_passports_count)
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
