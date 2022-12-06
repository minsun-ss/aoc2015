use std::fs;

fn part_one(contents: &Vec<String>) -> i32 {
    // what kind of nice string nonsense is this?
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut counter = 0;

    // check to see  ab, cd, pq, or xy
    for v in contents {
        let mut consecutive_same_characters = false;

        if v.chars().count() == 0 {
        } else if v.contains("ab") {
        } else if v.contains("cd") {
        } else if v.contains("pq") {
        } else if v.contains("xy") {
        } else {
            let mut count_vowels = 0;
            let mut prev_char = ' ';
            for (i, c) in v.chars().enumerate() {
                // for just the first round we don't check for matching items
                if i==0 {
                    if vowels.contains(&c) { count_vowels += 1;}
                    prev_char = c;
                } else {
                    if vowels.contains(&c) { count_vowels += 1;}
                    if prev_char == c {
                        consecutive_same_characters = true;
                    }
                    prev_char = c
                }
            }
            // check to see there were at least 1 pair of consecutive characters
            if consecutive_same_characters {
                // now check how many vowels there are
                if count_vowels >= 3 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn part_two(contents: &Vec<String>) -> i32 {
    // now need to check every 2 characters, which, well, man it sucks
    // xyxy
    // xyx

    let mut counter = 0;

    for v in contents {
        let mut consecutive_same_characters = false;
        let mut is_first_last_repeat = false;

        if v.chars().count() == 0 {
            // do nothing
        } else {

            // println!("{v}");
            for (i, _c) in v.chars().enumerate() {
                // for just the first two rounds we don't check for matching items
                if i < 2 {
                    // do nothing
                }
                else {
                    let left = &v[i-2..i];
                    let right = &v[i..];
                    // println!("{v} {left} {right}");
                    if right.contains(left) {
                        consecutive_same_characters = true;
                    }
                }

                // now also iterate to find xyx
                if i>=v.len()-2 {
                } else {
                    let first_char = &v[i..i+1];
                    let last_char = &v[i+2..i+3];
                    if first_char == last_char {
                        is_first_last_repeat = true;
                    }
                }
            }
            // check to see there were at least 1 pair of consecutive characters
            if consecutive_same_characters & is_first_last_repeat {
                counter += 1;
            }
        }
    }
    counter
}

fn main() {
    let contents: Vec<String> = fs::read_to_string("data/day5.txt")
        .expect("Should have been able to read the file")
        .split('\n')
        .map(|s| s.to_string())
        .collect()
        ;

    let result = part_one(&contents);
    println!("Part 1: {result}");

    let result = part_two(&contents);
    println!("Part 2: {result}");

}
