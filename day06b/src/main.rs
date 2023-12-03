fn check_dups(s: &Vec<char>) -> bool {
    let mut s = s.clone();
    s.sort_unstable();
    s.dedup();
    s.len() == 14
}

fn main() {
    let mut input = include_str!("../input.txt").chars();
    let mut last_14 = (&mut input).take(14).collect::<Vec<char>>();
    if check_dups(&last_14) {
        println!("14");
        return;
    }
    for (i, c) in input.enumerate() {
        last_14.remove(0);
        last_14.push(c);
        if check_dups(&last_14) {
            println!("{}", i + 15);
            return;
        }
    }
}

