struct Dir {
    size: u32,
}

const TOTAL_SIZE: u32 = 70_000_000;
const REQUIRED_SPACE: u32 = 30_000_000;

fn main() {
    let mut cur_dirs: Vec<Dir> = Vec::new();
    let mut scanned_dirs: Vec<Dir> = Vec::new();
    let input = include_str!("../input.txt")
        .lines()
        .filter(|line| !line.is_empty());

    for line in input {
        let segments = line.split(" ").collect::<Vec<&str>>();
        match segments[0] {
            "$" => match segments[1] {
                "cd" => match segments[2] {
                    ".." => {
                        scanned_dirs.push(cur_dirs.pop().unwrap());
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
    while let Some(dir) = cur_dirs.pop() {
       scanned_dirs.push(dir); 
    }
    let outer_dir_size = scanned_dirs.pop().unwrap().size;
    let free_space = TOTAL_SIZE - outer_dir_size;
    let required_space = REQUIRED_SPACE - free_space;
    let mut result = u32::MAX;
    while let Some(dir) = scanned_dirs.pop() {
        if dir.size >= required_space && dir.size < result {
            result = dir.size;
        }
    }
    println!("{}", result);
}

