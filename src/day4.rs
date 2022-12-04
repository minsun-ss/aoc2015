use std::fs;
extern crate md5;

fn part_one(contents: &String) -> i32 {
    let mut counter = 0;
    let result = loop {
        let checksum = format!("{contents}{}", counter.to_string());
        let digest = md5::compute(checksum);
        let val = format!("{:x}", digest);
        if &val[..5] == "00000" {
            println!("match {} {}", counter, val);
            break counter
        }
        counter += 1;
    };
    counter
}

fn part_two(contents: &String) -> i32 {
    let mut counter = 0;
    let result = loop {
        let checksum = format!("{contents}{}", counter.to_string());
        let digest = md5::compute(checksum);
        let val = format!("{:x}", digest);
        if &val[..6] == "000000" {
            println!("match {} {}", counter, val);
            break counter
        }
        counter += 1;
    };
    counter
}

fn main() {
    let contents = fs::read_to_string("data/day4.txt")
        .expect("Should have been able to read the file");



    let result = part_one(&contents);
    println!("Part 1: {result}");

    let result = part_two(&contents);
    println!("Part 2: {result}");

}
