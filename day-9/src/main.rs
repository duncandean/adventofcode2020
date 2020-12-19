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

    let mut start = 0;
    while !equal(&numbers[start..]) {
        start += 1;
    }

    Ok(())
}

fn equal(set: &[u64]) -> bool {
    let mut sum = 0;

    for (i, n) in set.iter().enumerate() {
        sum += n;
        if sum > 21806024 {
            return false;
        }

        if sum == 21806024 {
            println!(
                "Sum: {}",
                set[0..(i + 1)].iter().max().unwrap() + set[0..(i + 1)].iter().min().unwrap()
            );
            return true;
        }
    }
    false
}

// fn main() -> Result<(), io::Error> {
//     let filepath = &env::args().collect::<Vec<String>>()[1];
//     let numbers: Vec<u64> = read_lines(filepath)?
//         .map(|s| s.unwrap().parse::<u64>().unwrap())
//         .collect();

//     let mut start = 0;
//     while is_valid(&numbers[start..start + 25], numbers[start + 25]) {
//         start += 1;
//     }

//     Ok(println!("{}", numbers[start + 25]))
// }

// fn is_valid(preamble: &[u64], number: u64) -> bool {
//     for i in preamble {
//         for j in preamble {
//             if i + j == number {
//                 return true;
//             }
//         }
//     }
//     false
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
