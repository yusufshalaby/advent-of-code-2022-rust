use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(a, b)| {
            (
                match a {
                    "U" => (0, 1),
                    "D" => (0, -1),
                    "R" => (1, 0),
                    "L" => (-1, 0),
                    _ => panic!("Unknown direction"),
                },
                b.parse::<i32>().unwrap(),
            )
        });

    let mut target_touched = HashSet::new();
    let mut pos_array: [(i32, i32); 10] = [(0, 0); 10];
    target_touched.insert(pos_array[9]);
    for (direction, steps) in input {
        for _ in 0..steps {
            pos_array[0].0 += direction.0;
            pos_array[0].1 += direction.1;
            for i in 1..10 {
                if (pos_array[i - 1].0).abs_diff(pos_array[i].0) > 1
                    || (pos_array[i - 1].1).abs_diff(pos_array[i].1) > 1
                {
                    pos_array[i].0 += (pos_array[i - 1].0 - pos_array[i].0).signum();
                    pos_array[i].1 += (pos_array[i - 1].1 - pos_array[i].1).signum();
                }
            }
            target_touched.insert(pos_array[9]);
        }
    }

    let result = target_touched.len();
    println!("{}", result);
}
