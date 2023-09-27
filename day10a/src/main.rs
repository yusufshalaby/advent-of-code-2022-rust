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
    let mut cum_x = 1;
    let mut result = 0;
    for line in input {
        let (cycle, x) = line;
        for _ in 1..=cycle {
            cum_cycle += 1;
            if cum_cycle % 40 == 20 {
                result += cum_x * cum_cycle;
            }
        }
        cum_x += x;
        // println!("{} {}", cum_cycle, cum_x);
    }

    println!("{:?}", result);
}
