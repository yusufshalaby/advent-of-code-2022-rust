use std::{collections::VecDeque, iter, usize};

const NUM_STACKS: usize = 9;

fn main() {
    let input = include_str!("../input.txt").split_once("\n\n");

    let mut stacks: Vec<VecDeque<char>> = iter::repeat_with(|| VecDeque::new())
        .take(NUM_STACKS)
        .collect();
    input.unwrap().0.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push_front(c);
                }
            });
    });

    input.unwrap().1.lines().for_each(|line| {
        let vals = line.split(" ").collect::<Vec<_>>();
        let items = vals[1].parse::<usize>().unwrap();
        let src = vals[3].parse::<usize>().unwrap() - 1;
        let dst = vals[5].parse::<usize>().unwrap() - 1;
        for _ in 0..items {
            let item = stacks[src].pop_front().unwrap();
            stacks[dst].push_front(item);
        }
    });

    let result = stacks
        .iter()
        .map(|stack| stack.front().unwrap())
        .collect::<String>();

    println!("{:?}", result)
}
