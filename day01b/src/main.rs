fn main() {
    let mut cals = include_str!("../input.txt")
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    cals.sort_unstable();
    cals.reverse();
    println!("{:?}", cals.iter().take(3).sum::<u32>());
}
