use std::{collections::VecDeque, num::ParseIntError};

fn part1(input: &String) -> Result<u32, ParseIntError> {
    let mut elf_max_calories: u32 = 0;
    let mut elf_total_calories: u32 = 0;
    for line in input.lines() {
        if line == "" {
            elf_total_calories = 0;
            continue;
        }
        let calories: u32 = line.parse()?;
        elf_total_calories += calories;
        if elf_total_calories > elf_max_calories {
            elf_max_calories = elf_total_calories;
        }
    }

    Ok(elf_max_calories)
}

fn part2(input: &String) -> Result<u32, ParseIntError> {
    let mut top3 = VecDeque::from([0, 0, 0]);
    let mut elf_total_calories: u32 = 0;
    for line in input.lines() {
        if line == "" {
            update_top3(&mut top3, &elf_total_calories);
            elf_total_calories = 0;
            continue;
        }

        let calories: u32 = line.parse()?;
        elf_total_calories += calories;
    }

    let top3_total = top3.iter().sum();
    Ok(top3_total)
}

fn update_top3(top3: &mut VecDeque<u32>, elf_total_calories: &u32) -> () {
    let mut i: usize = 0;
    for val in top3.iter() {
        if elf_total_calories < val {
            break;
        }
        i += 1;
    }
    if i > 0 {
        top3.insert(i, *elf_total_calories);
        top3.pop_front();
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let part1result = part1(&input);
    match part1result {
        Ok(answer) => println!("The answer to part 1 is {answer}"),
        Err(e) => println!("Error in part 1: {e}"),
    }

    let part2result = part2(&input);
    match part2result {
        Ok(answer) => println!("The answer to part 2 is {answer}"),
        Err(e) => println!("Error in part 2: {e}"),
    }
}
