use std::collections::HashMap;
use std::env;
use std::fs;

fn part_one(contents: &String) -> i32 {
    let mut pos = (0, 0);
    let mut visits: HashMap<(i32, i32), i32> = HashMap::new();
    // need to initialize the very first position
    visits.insert(pos, 1);

    for c in contents.chars() {
        match c {
            '^' => pos.1 += 1,
            '>' => pos.0 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            _ => println!("Nothing"),
        }
        match visits.get(&pos) {
            Some(_visit_count) => {
                visits.entry(pos).and_modify(|pos| *pos += 1);
            }
            None => {
                visits.insert(pos, 1);
            }
        }
    }
    let mut presents = 0;
    for (_key, value) in &visits {
        if value >= &1 {
            presents += 1
        }
    }
    presents
}

fn part_two(contents: &String) -> i32 {
    // regular old santa and robo santa
    let mut santas = [(0, 0), (0, 0)];
    let mut visits: HashMap<(i32, i32), i32> = HashMap::new();
    // need to initialize the very first position for fake santa and robo santa
    visits.insert((0, 0), 2);

    for (i, c) in contents.chars().enumerate() {
        let santa_version = i % 2;
        match c {
            '^' => santas[santa_version].1 += 1,
            '>' => santas[santa_version].0 += 1,
            'v' => santas[santa_version].1 -= 1,
            '<' => santas[santa_version].0 -= 1,
            _ => println!("Nothing"),
        }
        // println!("{:?} {:?}", santas[0], santas[1]);
        match visits.get(&santas[santa_version]) {
            Some(_visit_count) => {
                visits
                    .entry(santas[santa_version])
                    .and_modify(|pos| *pos += 1);
            }
            None => {
                visits.insert(santas[santa_version], 1);
            }
        }
    }
    let mut presents = 0;
    for (_key, value) in &visits {
        if value >= &1 {
            presents += 1
        }
    }
    presents
}

fn main() {
    let home = env::current_dir().expect("Bad mojo");
    let filepath = home.join("src/data/day3.txt");
    // I guess the split() returns &str
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let result = part_one(&contents);
    println!("Part 1: {:?}", result);

    let result = part_two(&contents);
    println!("Part 2: {result}");
}
