fn check_dups(s: &Vec<char>) -> bool {
    let mut s = s.clone();
    s.sort_unstable();
    s.dedup();
    s.len() == 4
}

fn main() {
    let mut input = include_str!("../input.txt").chars();
    let mut last_4 = (&mut input).take(4).collect::<Vec<char>>();
    if check_dups(&last_4) {
        println!("4");
        return;
    }
    for (i, c) in input.enumerate() {
        last_4.remove(0);
        last_4.push(c);
        if check_dups(&last_4) {
            println!("{}", i + 5);
            return;
        }
    }
}

