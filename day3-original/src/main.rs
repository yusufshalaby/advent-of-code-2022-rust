use std::collections::HashSet;

static ASCII: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn get_part1_priority(line: &str) -> usize {
    let midpoint = line.len() / 2;
    let first_compartment = &line[..midpoint];
    let second_compartment = &line[midpoint..];
    let mut common_char: Option<char> = None;
    'outer: for char1 in first_compartment.chars() {
        for char2 in second_compartment.chars() {
            if char1 == char2 {
                common_char = Some(char1);
                break 'outer;
            }
        }
    }
    ASCII
        .iter()
        .position(|r| {
            r == &common_char.expect("both compartments should always have a common char")
        })
        .expect("ASCII constant has all possible chars in problem")
        + 1
}

fn get_part2_priority(line: &str, group_index: usize, elf_group_items: &mut HashSet<char>) -> usize {
    match group_index {
        1 => {
            *elf_group_items = HashSet::new();
            for c in line.chars() {
                elf_group_items.insert(c);
            }
        }
        _ => {
            let mut temp_items = HashSet::new();
            for c in line.chars() {
                if elf_group_items.contains(&c) {
                    temp_items.insert(c);
                }
            }
            *elf_group_items = temp_items;
        }
    }
    if group_index == 3 {
        let common_char = elf_group_items.iter().next();
        return ASCII
            .iter()
            .position(|r| {
                r == common_char.expect("both compartments should always have a common char")
            })
            .expect("ASCII constant has all possible chars in problem")
            + 1;
    } else {
        0
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut part1_priority = 0;
    let mut part2_priority = 0;
    let mut elf_group_items: HashSet<char> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        // part 1
        part1_priority += get_part1_priority(line);

        //part 2
        let group_index = (i % 3) + 1;
        part2_priority += get_part2_priority(line, group_index, &mut elf_group_items)
    }
    println!("Part 1 priority: {part1_priority}");
    println!("Part 2 priority: {part2_priority}");
}
