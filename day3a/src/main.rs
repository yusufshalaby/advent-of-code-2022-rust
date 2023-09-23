fn main() {
    println!(
        "{:?}",
        include_bytes!("../input.txt")
            .split(|&c| c == b'\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.split_at(l.len() / 2))
            .map(|(a, b)| b
                .iter()
                .filter(|bval| a.contains(bval))
                .map(|bval| if bval >= &b'a' {
                    (bval - b'a') as i16 + 1
                } else {
                    (bval - b'A') as i16 + 27
                })
                .next()
                .unwrap())
            .sum::<i16>()
    )
}

