fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .lines()
            .map(|l| l
                .split(",")
                .map(|s| s
                    .split("-")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>())
            .filter(|v| { !(v[0][1] < v[1][0] || v[0][0] > v[1][1]) })
            .count()
    )
}
