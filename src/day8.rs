use regex::Regex;
use std::{env, fs};

fn part_one(contents: &Vec<String>) -> i32 {
    let escape = Regex::new(r#"\\\\"#).unwrap();
    let dquote = Regex::new(r#"\\""#).unwrap();
    let acode = Regex::new(r#"\\x[0-9a-f]{2}"#).unwrap();
    let bcode = Regex::new(r#"(\\\\)|(\\")|(\\x[0-9a-f]{2})"#).unwrap();

    let mut tlength: i32 = 0;
    let mut alength: i32 = 0;
    for content in contents {
        let subcontent = &content[1..content.len() - 1];
        let mut substitute = subcontent.to_string();
        let actual_length: i32 = content.len() as i32;

        while let Some(mat) = bcode.find(&substitute.clone()) {
            let val = r#"\\"#.to_string();
            let val2 = r#"\""#.to_string();
            if mat.as_str() == val {
                substitute = bcode.replace(&substitute.clone(), r"\").into_owned();
            } else if mat.as_str() == val2 {
                substitute = bcode.replace(&substitute.clone(), r#"""#).into_owned();
            } else {
                let mut c =
                    char::from(u8::from_str_radix(&mat.as_str()[mat.len() - 2..], 16).unwrap())
                        .to_string();
                // as it turns out there are some hex charas that are 2 length
                if c.len() >= 2 {
                    c = "a".to_string();
                }
                substitute = bcode.replace(&substitute.clone(), c.clone()).into_owned();
            }
            // println!("{} {}", mat.as_str(), &substitute);
        }
        // getting rid of double quotes
        // 1730 is too high
        // 1130 is too low
        // 1359 is also not the right answer
        let mut content_length: i32 = substitute.len() as i32;
        tlength += actual_length;
        alength += content_length;

        println!(
            "{}: {} {} {}",
            content, substitute, actual_length, content_length
        );
    }

    let diff: i32 = tlength - alength;
    diff
}

fn main() {
    let home = env::current_dir().expect("Error on retrieving current directory");
    let filepath = home.join("src/data/day8.txt");
    let contents: Vec<String> = fs::read_to_string(filepath)
        .expect("Failed to open the file")
        .trim()
        .to_owned()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    println!("{:?}", contents);
    let result = part_one(&contents);
    println!("Part 1: {}", result);
}
