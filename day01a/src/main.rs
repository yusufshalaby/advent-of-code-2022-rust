fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .max()
            .unwrap()
    );
}

