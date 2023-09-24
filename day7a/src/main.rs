struct Dir {
    size: u32,
}

const MAX_DIR_SIZE: u32 = 100000;

fn main() {
    let mut cur_dirs: Vec<Dir> = Vec::new();
    let mut result = 0;
    let input = include_str!("../input.txt")
        .lines()
        .filter(|line| !line.is_empty());

    for line in input {
        let segments = line.split(" ").collect::<Vec<&str>>();
        match segments[0] {
            "$" => match segments[1] {
                "cd" => match segments[2] {
                    ".." => {
                        if let Some(dir) = cur_dirs.pop() {
                            if dir.size <= MAX_DIR_SIZE {
                                result += dir.size;
                            }
                        }
                    }
                    _ => {
                        cur_dirs.push(Dir { size: 0 });
                    }
                },
                _ => {}
            },
            "dir" => {}
            _ => {
                let size = segments[0].parse::<u32>().unwrap();
                cur_dirs.iter_mut().for_each(|dir| dir.size += size);
            }
        }
    }
    cur_dirs.iter().for_each(|dir| {
        if dir.size <= MAX_DIR_SIZE {
            result += dir.size;
        }
    });
    println!("{}", result);
}

