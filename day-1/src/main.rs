use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(read_to_string("inputs/day1.txt")?
        .split('\n')
        .filter_map(|s| s.parse::<usize>().ok())
        .combinations(3)
        .find(|v| v.iter().sum::<usize>() == 2020)
        .map(|v| v.iter().product::<usize>())
        .map_or_else(
            || println!("No solution found!"),
            |p| println!("Solution: {}", p),
        ))
}
