use std::{collections::VecDeque, num::ParseIntError};

fn get_top_k_most_calories(input: &String, k: &usize) -> Result<u32, ParseIntError> {
    let mut deque = VecDeque::from(vec![0; *k]);
    let mut elf_total_calories: u32 = 0;
    for line in input.lines() {
        if line == "" {
            update_deque(&mut deque, &elf_total_calories);
            elf_total_calories = 0;
            continue;
        }

        let calories: u32 = line.parse()?;
        elf_total_calories += calories;
    }

    let deque_total = deque.iter().sum();
    Ok(deque_total)
}

fn update_deque(deque: &mut VecDeque<u32>, elf_total_calories: &u32) -> () {
    let mut i: usize = 0;
    for val in deque.iter() {
        if elf_total_calories < val {
            break;
        }
        i += 1;
    }
    if i > 0 {
        deque.insert(i, *elf_total_calories);
        deque.pop_front();
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let part1result = get_top_k_most_calories(&input, &1);
    match part1result {
        Ok(answer) => println!("The answer to part 1 is {answer}"),
        Err(e) => println!("Error in part 1: {e}"),
    }

    let part2result = get_top_k_most_calories(&input, &3);
    match part2result {
        Ok(answer) => println!("The answer to part 2 is {answer}"),
        Err(e) => println!("Error in part 2: {e}"),
    }
}
