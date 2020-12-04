use std::{env, fs};

static MAX_U24: i32 = 16777215;

fn main() {
    let filepath = &env::args().collect::<Vec<String>>()[1];
    let valid_count = fs::read_to_string(filepath)
        .unwrap()
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|passport| is_valid(passport))
        .count();

    println!("There are {} valid passports.", valid_count)
}

fn is_valid(passport: &str) -> bool {
    passport
        .split_whitespace()
        .map(|field| {
            if let Some(value) = field.get(4..) {
                return match field.get(0..4) {
                    Some("byr:") => validate_year(value, 1920, 2002),
                    Some("iyr:") => validate_year(value, 2010, 2020),
                    Some("eyr:") => validate_year(value, 2020, 2030),
                    Some("hgt:") => validate_height(value),
                    Some("hcl:") => validate_hair_colour(value),
                    Some("ecl:") => {
                        value == "amb"
                            || value == "blu"
                            || value == "brn"
                            || value == "gry"
                            || value == "grn"
                            || value == "hzl"
                            || value == "oth"
                    }
                    Some("pid:") => value.len() == 9 && value.parse::<i32>().is_ok(),
                    _ => false,
                };
            }
            false
        })
        .filter(|x| *x)
        .count()
        == 7
}

fn validate_year(year: &str, min: i32, max: i32) -> bool {
    if let Ok(year) = year.parse::<i32>() {
        min <= year && year <= max
    } else {
        false
    }
}

fn validate_hair_colour(value: &str) -> bool {
    if let Some(colour) = value.get(1..) {
        if !value.starts_with('#') {
            return false;
        }
        if let Ok(hex_number) = i32::from_str_radix(colour, 16) {
            return hex_number <= MAX_U24;
        }
    }
    false
}

fn validate_height(value: &str) -> bool {
    let min_cm = 150;
    let max_cm = 193;
    let min_in = 59;
    let max_in = 76;

    if let Some(unit) = value.get((value.len() - 2)..) {
        if let Some(height) = value.get(..(value.len() - 2)) {
            let height = height.parse::<i32>().unwrap_or(0);
            if unit == "cm" {
                return min_cm <= height && height <= max_cm;
            }

            if unit == "in" {
                return min_in <= height && height <= max_in;
            }
        }
    }
    false
}
