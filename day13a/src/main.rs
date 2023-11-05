#[derive(Debug)]
enum Data {
    List(Box<Vec<Data>>),
    Integer(u32),
}

impl Data {
    fn new(mut i: usize, line: &str) -> (usize, Vec<Data>) {
        let mut data: Vec<Data> = Vec::new();
        while i < line.len() {
            let c = line.chars().nth(i).unwrap();
            // println!("{}", c);
            match c {
                '[' => {
                    let (delta_i, nested) = Data::new(i + 1, &line);
                    data.push(Data::List(Box::new(nested)));
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

fn main() {
    let input = include_str!("../test.txt")
        .lines()
        .filter(|line| !line.is_empty())
        // .skip(14)
        .map(|line| Data::new(0, line.replace(",", "").as_str()).1)
        .collect::<Vec<Vec<Data>>>();
        // .nth(3);

    println!("{:?}", input);
}
