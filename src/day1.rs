use std::env;
use std::fs;

fn part_one(contents: &String) -> i32 {
        // let mut v: Vec<char> = Vec::new();
    let mut floor: i32 = 0;

    for c in contents.chars() {
        if c == '(' {
            floor += 1;
        }
        else {
            floor -= 1;
        }
    }
    floor
}

fn part_two(contents: &String) -> usize {
    let mut floor: i32 = 0;

    for (i, c) in contents.chars().enumerate() {
        if c == '(' {
            floor += 1;
        }
        else {
            floor -= 1;
        }

        if floor == -1 {
            return i+1;
        }
    }
    return 0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Using {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let result = part_one(&contents);
    println!("Part 1: {result}");

    let result = part_two(&contents);
    println!("Part 2: {result}");

}
