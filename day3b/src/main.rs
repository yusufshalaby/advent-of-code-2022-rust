fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&c| c == b'\n')
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|sliced| sliced[0]
                .into_iter()
                .filter(|val| sliced[1].contains(val) && sliced[2].contains(val))
                .map(|val| if val >= &b'a' {
                    (val - b'a' + 1) as i16
                } else {
                    (val - b'A' + 27) as i16
                })
                .next()
                .unwrap())
            .sum::<i16>()
    )
}
