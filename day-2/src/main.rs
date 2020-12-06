use std::{fs::File, env, io::{self, BufRead}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(filepath) = &env::args().collect::<Vec<String>>().get(1) {
        let valid_passwords: usize = read_lines(filepath)?.into_iter().filter(|line| {
            if let Ok(line) = line {
                let mut split_line = line.split(|c| c == '-' || c == ' ' || c == ':');
                let i1 = split_line.next().unwrap().parse::<u32>().unwrap();
                let i2 = split_line.next().unwrap().parse::<u32>().unwrap();
                let character = split_line.next().unwrap().chars().next().unwrap();
                split_line.next();
                let mut count = 0;
                for (i, c) in split_line.next().unwrap().chars().enumerate() {
                    if (i as u32 == i1 - 1 || i as u32 == i2 - 1) && c == character {
                        count += 1;
                    }
                }
                count == 1
            } else { false }
        }).count();
        Ok(println!("There are {} valid passwords.", valid_passwords))
    } else {
        Ok(println!("Please provide a filepath argument."))
    }
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
