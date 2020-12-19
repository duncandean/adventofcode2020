use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), io::Error> {
    let filepath = &env::args().collect::<Vec<String>>()[1];
    let numbers: Vec<u64> = read_lines(filepath)?
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect();

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
