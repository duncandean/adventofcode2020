use std::io::{self, BufRead};
use std::path::Path;
use std::{env, fs::File};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = &env::args().collect::<Vec<String>>()[1];
    let map = read_lines(filepath)?.collect::<Result<Vec<String>, io::Error>>()?;

    let trees_product: u32 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| count_trees(&map, slope.0, slope.1))
        .product();

    Ok(println!("The product is: {}", trees_product))
}

fn count_trees(map: &Vec<String>, right_step: u32, down_step: u32) -> u32 {
    let row_length = map.first().unwrap().len() as u32;
    let mut x = 0;
    map.iter()
        .step_by(down_step as usize)
        .map(|row| {
            let possible_tree = row.chars().collect::<Vec<char>>()[x as usize];
            x += right_step;
            x %= row_length;
            possible_tree
        })
        .filter(|c| c == &'#')
        .count() as u32
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
