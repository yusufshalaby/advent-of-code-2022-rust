fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|l| match l.get(0..4) {
            Some("noop") => (1 as i32, 0),
            Some("addx") => (
                2 as i32,
                l.split_once(" ").unwrap().1.parse::<i32>().unwrap(),
            ),
            _ => panic!("Unknown instruction: {}", l),
        });

    let mut cum_cycle = 0;
    let mut sprite_pos: i32 = 1;
    let mut result = vec![".".repeat(40); 6];
    let mut row = 0;
    for line in input {
        let (cycle, x) = line;
        for _ in 1..=cycle {
            let x_index = cum_cycle % 40;
            if (sprite_pos - 1..sprite_pos + 2).contains(&x_index) {
                result[row as usize].replace_range(x_index as usize..x_index as usize + 1, "#")
            };
            cum_cycle += 1;
            row = cum_cycle / 40
        }
        sprite_pos += x;
    }

    println!("{}", result.join("\n"));
}
