fn main() {
    println!(
        "{:?}",
        include_bytes!("../input.txt")
            .split(|&x| x == b'\n')
            .filter(|line| line.len() > 0)
            .map(|line| ((line[0] - b'A') as i16, (line[2] - b'X') as i16,))
            .map(|(a, b)| { b * 3 + 1 + (2 + a + b) % 3 })
            .sum::<i16>()
    );
}
