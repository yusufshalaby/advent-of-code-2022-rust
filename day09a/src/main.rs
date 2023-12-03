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
    let mut pos_h: (i32, i32) = (0, 0);
    let mut pos_t: (i32, i32) = (0, 0);
    target_touched.insert(pos_t);
    for (direction, steps) in input {
        for _ in 0..steps {
            pos_h.0 += direction.0;
            pos_h.1 += direction.1;
            if (pos_h.0).abs_diff(pos_t.0) > 1 || (pos_h.1).abs_diff(pos_t.1) > 1 {
                pos_t.0 += (pos_h.0 - pos_t.0).signum();
                pos_t.1 += (pos_h.1 - pos_t.1).signum();
                target_touched.insert(pos_t);
            }
        }
    }

    let result = target_touched.len();
    println!("{}", result);
}
