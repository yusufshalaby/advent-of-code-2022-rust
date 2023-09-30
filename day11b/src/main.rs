use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Multiply(Value),
    Add(Value),
}

#[derive(Debug)]
enum Value {
    Number(i64),
    Old,
}

impl Value {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Value::Number(number) => *number,
            Value::Old => old,
        }
    }
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Operation::Multiply(value) => value.apply(old) * old,
            Operation::Add(value) => value.apply(old) + old,
        }
    }
}
    
#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test_factor: i64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn parse_input(input: &str) -> Monkey {
        let items = input
            .lines()
            .nth(1)
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<VecDeque<i64>>();

        let operation = input
            .lines()
            .nth(2)
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split_once(" ")
            .map(|(operator, number)| {
                if let Ok(number) = number.parse::<i64>() {
                    match operator {
                        "*" => Operation::Multiply(Value::Number(number)),
                        "+" => Operation::Add(Value::Number(number)),
                        _ => panic!("Unknown operation"),
                    }
                } else {
                    return Operation::Multiply(Value::Old);
                }
            })
            .unwrap();

        let test_stuff = input
            .lines()
            .skip(3)
            .map(|line| line.split(" ").last().unwrap().parse::<i64>())
            .collect::<Vec<Result<i64, _>>>();

        Monkey {
            items,
            operation,
            test_factor: test_stuff[0].to_owned().unwrap(),
            if_true: test_stuff[1].to_owned().unwrap() as usize,
            if_false: test_stuff[2].to_owned().unwrap() as usize,
        }
    }
}

fn main() {
    let mut monkeys = Vec::<Monkey>::new();

    let input = include_str!("../input.txt").split("\n\n");

    for raw_monkey in input {
        monkeys.push(Monkey::parse_input(raw_monkey));
    }

    let common_multiple = monkeys
        .iter()
        .map(|monkey| monkey.test_factor)
        .fold(1, |acc, x| acc * x);

    // println!("{:?}", monkeys)

    let mut result: Vec<i64> = vec![0; monkeys.len()];
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                result[i] += 1;
                let item_new = monkeys[i].operation.apply(item) % common_multiple;
                if item_new % monkeys[i].test_factor == 0 {
                    let new_monkey = monkeys[i].if_true;
                    monkeys[new_monkey].items.push_back(item_new);
                } else {
                    let new_monkey = monkeys[i].if_false;
                    monkeys[new_monkey].items.push_back(item_new);
                }
            }
        }
    }

    println!("{:?}", result);
    result.sort_unstable();
    result.reverse();
    println!("{:?}", result[0] * result[1]);
}
