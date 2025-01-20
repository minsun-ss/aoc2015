use std::collections::HashMap;
use std::str::FromStr;
use std::{env, fs};

fn parse(clause: &String) -> Vec<String> {
    clause
        .split("->")
        .map(|s| s.to_string().trim().to_owned())
        .collect()
}

fn part_one(contents: &Vec<String>) -> u16 {
    let mut hm: HashMap<String, u16> = HashMap::new();

    let mut current_list: Vec<String> = Vec::new();
    let mut next_list: Vec<String> = Vec::new();
    current_list.clone_from(contents);

    while current_list.len() > 0 {
        for content in &current_list {
            let val = parse(&content);

            // check to see if there is a number
            if !val[0].contains(" ") {
                // try to unwrap
                let value = u16::from_str(&val[0]).unwrap_or(u16::MAX);
                if value != u16::MAX {
                    hm.insert(val[1].to_owned(), u16::from_str(&val[0]).unwrap());
                } else {
                    // then the value isn't a number
                    if hm.contains_key(&val[0]) {
                        let left = hm.get(&val[0]).unwrap();
                        hm.insert(val[1].to_owned(), *left);
                    } else {
                        next_list.push(content.to_string());
                    }
                }
            }
            // check to see if there is and AND
            else if val[0].contains("AND") {
                let left_right: Vec<String> = val[0]
                    .split("AND")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                let left;
                if left_right[0].chars().any(|c| c.is_numeric()) {
                    left = u16::from_str(&left_right[0]).unwrap();
                } else if hm.contains_key(&left_right[0]) {
                    left = *hm.get(&left_right[0]).unwrap();
                } else {
                    next_list.push(content.to_string());
                    continue;
                }

                let right;
                if left_right[1].chars().any(|c| c.is_numeric()) {
                    right = u16::from_str(&left_right[1]).unwrap();
                } else if hm.contains_key(&left_right[1]) {
                    right = *hm.get(&left_right[1]).unwrap();
                } else {
                    next_list.push(content.to_string());
                    continue;
                }

                let bitwise_and = left & right;
                hm.insert(val[1].to_owned(), bitwise_and);
            } else if val[0].contains("OR") {
                let left_right: Vec<String> = val[0]
                    .split("OR")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                let left;
                if left_right[0].chars().any(|c| c.is_numeric()) {
                    left = u16::from_str(&left_right[0]).unwrap();
                } else if hm.contains_key(&left_right[0]) {
                    left = *hm.get(&left_right[0]).unwrap();
                } else {
                    next_list.push(content.to_string());
                    continue;
                }

                let right;
                if left_right[1].chars().any(|c| c.is_numeric()) {
                    right = u16::from_str(&left_right[1]).unwrap();
                } else if hm.contains_key(&left_right[1]) {
                    right = *hm.get(&left_right[1]).unwrap();
                } else {
                    next_list.push(content.to_string());
                    continue;
                }

                let bitwise_or = left | right;
                hm.insert(val[1].to_owned(), bitwise_or);
            } else if val[0].contains("LSHIFT") {
                let left_right: Vec<String> = val[0]
                    .split("LSHIFT")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                if hm.contains_key(&left_right[0]) {
                    let right = u16::from_str(&left_right[1]).unwrap();
                    let left = hm.get(&left_right[0]).unwrap() << right;
                    hm.insert(val[1].to_owned(), left);
                } else {
                    next_list.push(content.to_string());
                }
            } else if val[0].contains("RSHIFT") {
                let left_right: Vec<String> = val[0]
                    .split("RSHIFT")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                if hm.contains_key(&left_right[0]) {
                    let right = u16::from_str(&left_right[1]).unwrap();
                    let left = hm.get(&left_right[0]).unwrap() >> right;
                    hm.insert(val[1].to_owned(), left);
                } else {
                    next_list.push(content.to_string());
                }
            } else if val[0].contains("NOT") {
                let left_right: Vec<String> = val[0]
                    .split(" ")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                if hm.contains_key(&left_right[1]) {
                    let right = hm.get(&left_right[1]).unwrap();
                    hm.insert(val[1].to_owned(), !right);
                } else {
                    next_list.push(content.to_string());
                }
            } else {
                next_list.push(content.to_string());
            }
        }

        current_list = next_list;
        next_list = Vec::new();
    }

    hm.get("a").copied().expect("Value should be an u16")
}

fn part_two(contents: &Vec<String>) -> u16 {
    let mut hm: HashMap<String, u16> = HashMap::new();

    let mut current_list: Vec<String> = Vec::new();
    let mut next_list: Vec<String> = Vec::new();
    // current_list.clone_from(contents);

    for content in contents {
        if content.ends_with("-> b") {
            let new_string = format!("{:?} -> b", part_one(&contents));
            current_list.push(new_string);
        } else {
            current_list.push(content.to_string());
        }
    }

    while current_list.len() > 0 {
        for content in &current_list {
            let val = parse(&content);

            // check to see if there is a number
            if !val[0].contains(" ") {
                // try to unwrap
                let value = u16::from_str(&val[0]).unwrap_or(u16::MAX);
                if value != u16::MAX {
                    hm.insert(val[1].to_owned(), u16::from_str(&val[0]).unwrap());
                } else {
                    // then the value isn't a number
                    if hm.contains_key(&val[0]) {
                        let left = hm.get(&val[0]).unwrap();
                        hm.insert(val[1].to_owned(), *left);
                    } else {
                        next_list.push(content.to_string());
                    }
                }
            }
            // check to see if there is and AND
            else if val[0].contains("AND") {
                let left_right: Vec<String> = val[0]
                    .split("AND")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                let left;
                if left_right[0].chars().any(|c| c.is_numeric()) {
                    left = u16::from_str(&left_right[0]).unwrap();
                } else if hm.contains_key(&left_right[0]) {
                    left = *hm.get(&left_right[0]).unwrap();
                } else {
                    next_list.push(content.to_string());
                    continue;
                }

                let right;
                if left_right[1].chars().any(|c| c.is_numeric()) {
                    right = u16::from_str(&left_right[1]).unwrap();
                } else if hm.contains_key(&left_right[1]) {
                    right = *hm.get(&left_right[1]).unwrap();
                } else {
                    next_list.push(content.to_string());
                    continue;
                }

                let bitwise_and = left & right;
                hm.insert(val[1].to_owned(), bitwise_and);
            } else if val[0].contains("OR") {
                let left_right: Vec<String> = val[0]
                    .split("OR")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                let left;
                if left_right[0].chars().any(|c| c.is_numeric()) {
                    left = u16::from_str(&left_right[0]).unwrap();
                } else if hm.contains_key(&left_right[0]) {
                    left = *hm.get(&left_right[0]).unwrap();
                } else {
                    next_list.push(content.to_string());
                    continue;
                }

                let right;
                if left_right[1].chars().any(|c| c.is_numeric()) {
                    right = u16::from_str(&left_right[1]).unwrap();
                } else if hm.contains_key(&left_right[1]) {
                    right = *hm.get(&left_right[1]).unwrap();
                } else {
                    next_list.push(content.to_string());
                    continue;
                }

                let bitwise_or = left | right;
                hm.insert(val[1].to_owned(), bitwise_or);
            } else if val[0].contains("LSHIFT") {
                let left_right: Vec<String> = val[0]
                    .split("LSHIFT")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                if hm.contains_key(&left_right[0]) {
                    let right = u16::from_str(&left_right[1]).unwrap();
                    let left = hm.get(&left_right[0]).unwrap() << right;
                    hm.insert(val[1].to_owned(), left);
                } else {
                    next_list.push(content.to_string());
                }
            } else if val[0].contains("RSHIFT") {
                let left_right: Vec<String> = val[0]
                    .split("RSHIFT")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                if hm.contains_key(&left_right[0]) {
                    let right = u16::from_str(&left_right[1]).unwrap();
                    let left = hm.get(&left_right[0]).unwrap() >> right;
                    hm.insert(val[1].to_owned(), left);
                } else {
                    next_list.push(content.to_string());
                }
            } else if val[0].contains("NOT") {
                let left_right: Vec<String> = val[0]
                    .split(" ")
                    .map(|s| s.to_string().trim().to_owned())
                    .collect();

                if hm.contains_key(&left_right[1]) {
                    let right = hm.get(&left_right[1]).unwrap();
                    hm.insert(val[1].to_owned(), !right);
                } else {
                    next_list.push(content.to_string());
                }
            } else {
                next_list.push(content.to_string());
            }
        }

        current_list = next_list;
        next_list = Vec::new();
    }

    hm.get("a").copied().expect("Value should be an u16")
}

fn main() {
    let home = env::current_dir().expect("Failure in retrieving current directory");
    let filepath = home.join("src/data/day7.txt");
    let contents: Vec<String> = fs::read_to_string(filepath)
        .expect("Failure to open filepath")
        .trim()
        .to_owned()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut result = part_one(&contents);
    println!("Part 1: {:?}", result);

    result = part_two(&contents);
    println!("Part 2: {:?}", result);
}
