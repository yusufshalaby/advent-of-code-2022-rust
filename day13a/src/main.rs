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
        while i < line.len() {
            let c = line.chars().nth(i).unwrap();
            // println!("{}", c);
            match c {
                '[' => {
                    let (delta_i, nested) = Data::new(i + 1, &line);
                    data.push(nested);
                    i = delta_i;
                }
                ']' => {
                    return (i + 1, data);
                }
                _ => {
                    data.push(Data::Integer(c.to_digit(10).unwrap()));
                    i += 1;
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
        .map(|line| Data::new(0, line.replace(",", "").as_str()).1);
    // .take(2)
    // .collect::<Vec<Vec<Data>>>();
    // .nth(3);

    let mut result = 0;
    let mut i = 0;

    while let Some(line) = input.clone().nth(i + 1) {
        if input.clone().nth(i).unwrap() <= line {
            println!("{:?}", line);
            println!("{}", 1 + i / 2);
            result += 1 + i / 2;
        }
        i += 2;
    }

    println!("Final result: {}", result);

    // println!("{:?}", input[0] < input[1]);
}
