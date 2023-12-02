use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Data {
    List(Vec<Data>),
    Integer(u32),
}

impl Data {
    fn push(&mut self, data: Data) {
        match self {
            Data::List(list) => list.push(data),
            _ => panic!("Cannot push to non-list data"),
        }
    }

    fn new(mut i: usize, line: &str) -> (usize, Data) {
        let mut data: Data = Data::List(vec![]);
        let mut num = String::new();
        while i < line.len() {
            let c = line.chars().nth(i).unwrap();
            match c {
                '[' => {
                    let (delta_i, nested) = Data::new(i + 1, &line);
                    data.push(nested);
                    i = delta_i;
                }
                ']' => {
                    if !num.is_empty() {
                        data.push(Data::Integer(num.parse::<u32>().unwrap()));
                    }
                    return (i + 1, data);
                }
                ',' => {
                    if !num.is_empty() {
                        data.push(Data::Integer(num.parse::<u32>().unwrap()));
                        num = String::new();
                    }
                    i += 1;
                }
                '0'..='9' => {
                    num.push(c);
                    i += 1;
                }
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            }
        }
        return (i, data);
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::Integer(a), Data::Integer(b)) => a.cmp(b),
            (Data::List(a), Data::List(b)) => a.cmp(b),
            (Data::Integer(a), Data::List(b)) => vec![Data::Integer(*a)].cmp(b),
            (Data::List(a), Data::Integer(b)) => a.cmp(&vec![Data::Integer(*b)]),
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Data::new(0, line).1);

    let mut result = 0;
    let mut i = 0;

    while let Some(line) = input.clone().nth(i + 1) {
        if input.clone().nth(i).unwrap() <= line {
            result += 1 + i / 2;
        }
        i += 2;
    }

    println!("Final result: {}", result);

}
