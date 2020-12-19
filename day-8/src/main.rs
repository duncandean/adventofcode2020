use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
enum OpCode {
    UNKNOWN,
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

fn main() -> Result<(), io::Error> {
    let filepath = &env::args().collect::<Vec<String>>()[1];
    let instructions = read_lines(filepath)?.collect::<Result<Vec<String>, io::Error>>()?;

    let mut executed_lines: HashSet<i32> = HashSet::new();
    let mut current_line = 0;
    let mut accumulator = 0;

    loop {
        if executed_lines.contains(&current_line) {
            println!(
                "Will repeat: {} : {:?}",
                current_line + 1,
                parse_op(&instructions[current_line as usize])
            );
            break;
        }

        if current_line >= instructions.len() as i32 {
            break;
        }
        let increment = execute(&instructions[current_line as usize], &mut accumulator);
        executed_lines.insert(current_line);
        current_line += increment;
    }

    Ok(println!("{}", accumulator))
}

fn parse_op(op: &str) -> OpCode {
    let split_op: Vec<&str> = op.split(' ').collect();
    let op_string = split_op[0];
    let value = split_op[1].parse::<i32>().unwrap();

    println!("{}, {}", op_string, value);

    match op_string {
        "nop" => OpCode::NOP(value),
        "acc" => OpCode::ACC(value),
        "jmp" => OpCode::JMP(value),
        _ => OpCode::UNKNOWN,
    }
}

fn execute(op: &str, accumulator: &mut i32) -> i32 {
    let op_code = parse_op(op);
    match op_code {
        OpCode::NOP(_) => 1,
        OpCode::ACC(value) => {
            *accumulator += value;
            1
        }
        OpCode::JMP(value) => value,
        _ => 0,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
