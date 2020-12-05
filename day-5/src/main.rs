use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), io::Error> {
    let filepath = &env::args().collect::<Vec<String>>()[1];
    let boarding_passes = read_lines(filepath)?.collect::<Result<Vec<String>, io::Error>>()?;

    let mut sorted_seat = boarding_passes
        .iter()
        .map(|pass| boarding_pass_to_seat_id(pass))
        .collect::<Vec<i32>>();
    sorted_seat.sort();
    sorted_seat.windows(2).for_each(|pair| {
        if pair[1] - pair[0] > 1 {
            println!("My seat: {}", pair[0] + 1);
        }
    });

    Ok(())
}

fn boarding_pass_to_seat_id(pass: &str) -> i32 {
    let row_pattern = pass.get(..7).unwrap_or_default();
    let column_pattern = pass.get(7..).unwrap_or_default();

    let row = binary_pattern_search(row_pattern, 'F', 'B', 0, 127);
    let column = binary_pattern_search(column_pattern, 'L', 'R', 0, 7);

    compute_seat_id(row, column)
}

fn compute_seat_id(row: i32, column: i32) -> i32 {
    row * 8 + column
}

fn binary_pattern_search(
    pattern: &str,
    lower_char: char,
    upper_char: char,
    mut min: i32,
    mut max: i32,
) -> i32 {
    let mut chars = pattern.chars();
    while let Some(c) = chars.next() {
        let middle = min + (max - min) / 2;

        if c == upper_char {
            if max - min <= 1 {
                return max;
            }

            min = middle + 1;
            continue;
        }

        if c == lower_char {
            if max - min <= 1 {
                return min;
            }

            max = middle;
            continue;
        }
    }
    -1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
