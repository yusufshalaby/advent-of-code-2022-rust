#[derive(Debug)]
enum List {
    Value(u32),
    Nested(Box<Vec<List>>),
}

impl List {
    fn new(mut i: usize, line: &str) -> (usize, Vec<List>) {
        // println!("new nest");
        let mut list: Vec<List> = Vec::new();
        while i < line.len() {
            let c = line.chars().nth(i).unwrap();
            // println!("{}", c);
            match c {
                '[' => {
                    let (delta_i, nested) = List::new(i + 1, &line);
                    list.push(List::Nested(Box::new(nested)));
                    i = delta_i;
                }
                ']' => {
                    // println!("close nest");
                    return (i + 1, list);
                }
                _ => {
                    list.push(List::Value(c.to_digit(10).unwrap()));
                    i += 1;
                }
            }
        }
        return (i, list);
    }
}

fn main() {
    let input = include_str!("../test.txt")
        .lines()
        .filter(|line| !line.is_empty())
        // .skip(14)
        .map(|line| List::new(0, line.replace(",", "").as_str()).1)
        .collect::<Vec<Vec<List>>>();
        // .nth(3);

    println!("{:?}", input);
}
