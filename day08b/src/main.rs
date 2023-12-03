use std::iter;

fn find_visible(input: Vec<Vec<(&u8, i32)>>) -> Vec<Vec<(&u8, i32)>> {
    let mut output = Vec::new();
    for row in input {
        let mut counters = [0; 10];
        let mut output_row = Vec::new();
        for (col, score) in row {
            let index = *col - b'0';
            output_row.push((col, score * counters[index as usize]));
            counters[..=index as usize].iter_mut().for_each(|c| *c = 1);
            counters[index as usize + 1..].iter_mut().for_each(|c| *c += 1);
        }
        output.push(output_row);
    }
    output
}

fn main() {
    let input = include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.iter().zip(iter::repeat(1)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let output = find_visible(input);

    let input = output
        .iter()
        .map(|l| l.iter().rev().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let output = find_visible(input);

    let input = (0..output[0].len())
        .map(|i| {
            output
                .iter()
                .map(|l| l[output[0].len() - 1 - i])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let output = find_visible(input);

    let input = output
        .iter()
        .map(|l| l.iter().rev().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let output = find_visible(input);

    println!(
        "{:?}",
        output
            .iter()
            .map(|l| l.iter().map(|(_, s)| s).max())
            .max()
    );
}
