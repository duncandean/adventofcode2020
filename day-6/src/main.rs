use std::{collections::HashMap, env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = &env::args().collect::<Vec<String>>()[1];
    let count: i32 = fs::read_to_string(filepath)?
        .split("\n\n")
        .into_iter()
        .map(|card| {
            println!("=> {} <=", card);
            count_answers(card)
        })
        .sum();

    Ok(println!("{}", count))
}

fn count_answers(customs_card: &str) -> i32 {
    let mut answered: HashMap<char, i32> = HashMap::new();
    let people: Vec<&str> = customs_card.split('\n').collect();
    let group_size = people.len();

    for person in people {
        person.chars().for_each(|c| {
            let mut value_to_insert = 1;
            if let Some(count) = &answered.get(&c) {
                value_to_insert += *count;
            }
            answered.insert(c, value_to_insert);
        })
    }

    answered
        .iter()
        .filter_map(|entry| {
            if *entry.1 as usize == group_size {
                return Some(entry.0);
            }
            None
        })
        .count() as i32
}
