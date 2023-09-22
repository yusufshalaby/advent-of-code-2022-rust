fn main() {
    println!(
        "{:?}",
        include_bytes!("../input.txt")
            .split(|&x| x == b'\n')
            .filter(|line| line.len() > 0)
            .map(|line| ((line[0] - b'A') as i16, (line[2] - b'X') as i16,))
            .map(|(a, b)| { b + 1 + (3 * (b == a) as i16) + (6 * ((b-a) == 1) as i16) + (6 * ((b-a) == -2) as i16) })
            .sum::<i16>()
    );
}
