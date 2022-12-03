use std::env;
use std::fs;

fn part_one(contents: &Vec<String>) -> i32 {
    let mut wrapping_paper_area: i32 = 0;

    for c in contents {
        if c.chars().count() == 0 { println!("End"); }
        else {
            let a: Vec<i32> = c.split('x').map(|s| s.parse().unwrap()).collect();
            if let [l, w, h] = &a[0..3] {
                let areas = [l*w, w*h, h*l];
                wrapping_paper_area += (2*l*w) + (2*w*h) + (2*h*l);
                wrapping_paper_area += areas.iter().min().unwrap()
            }
            else {
                println!("Wrong shaped vector");
            };
        }
    }
    wrapping_paper_area
}

fn part_two(contents: &Vec<String>) -> i32 {
    let mut ribbon: i32 = 0;

    for c in contents {
        if c.chars().count() == 0 { println!("End"); } else {
            let a: Vec<i32> = c.split('x').map(|s| s.parse().unwrap()).collect();
            if let [l, w, h] = &a[0..3] {
                let areas = [2 * (l + w), 2 * (w + h), 2 * (h + l)];
                ribbon += areas.iter().min().unwrap();
                ribbon += l * w * h
            } else {
                println!("Wrong shaped vector");
            };
        }
    }
    ribbon
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Using {file_path}");

    // I guess the split() returns &str
    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let result = part_one(&contents);
    println!("Part 1: {result}");

    let result = part_two(&contents);
    println!("Part 2: {result}");

}
