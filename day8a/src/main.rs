use std::iter;

fn find_visible(input: Vec<Vec<(&u8, i32)>>) -> Vec<Vec<(&u8, i32)>> {
    let mut output = Vec::new();
    for row in input {
        let mut output_row = Vec::new();
        let mut tallest = u8::MIN;
        for (col, seen) in row {
            if col > &tallest {
                tallest = *col;
                output_row.push((col, 1));
            } else {
                output_row.push((col, seen));
            }
        }
        output.push(output_row);
    }
    output
}

fn main() {
    let input = include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.iter().zip(iter::repeat(0)).collect::<Vec<_>>())
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
        "{}",
        output
            .iter()
            .map(|l| l.iter().map(|(_, s)| s).sum::<i32>())
            .sum::<i32>()
    );
}
