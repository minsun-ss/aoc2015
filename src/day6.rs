use std::fs;

fn part_one(contents: &Vec<String>) -> i32 {
    let mut lights: [[bool; 1000];1000] = [[false; 1000]; 1000];
    for c in contents {
        let commands: Vec<String> = c.split(" ").map(|s| s.to_string()).collect();
        match commands[0].as_ref() {
            "toggle" => {
                let start: Vec<usize> = commands[1].split(",").map(|i| i.parse().unwrap()).collect();
                let end: Vec<usize> = commands[3].split(",").map(|i| i.trim().parse().unwrap()).collect();
                for x in start[0]..end[0]+1 {
                    for y in start[1]..end[1]+1 {
                        lights[x][y] ^= true;
                    }
                }
            },
            "turn" => {
                if commands[1] == "off" {
                    let start: Vec<usize> = commands[2].split(",").map(|i| i.parse().unwrap()).collect();
                    let end: Vec<usize> = commands[4].split(",").map(|i| i.trim().parse().unwrap()).collect();
                    for x in start[0]..end[0]+1 {
                        for y in start[1]..end[1]+1 {
                            lights[x][y] = false;
                        }
                    }
                } else {
                    let start: Vec<usize> = commands[2].split(",").map(|i| i.parse().unwrap()).collect();
                    let end: Vec<usize> = commands[4].split(",").map(|i| i.trim().parse().unwrap()).collect();
                    for x in start[0]..end[0]+1 {
                        for y in start[1]..end[1]+1 {
                            lights[x][y] = true;
                        }
                    }
                }
            },
            _ => {}
        }
    }

    // now count lights
    let mut light_counts = 0;
    for r in 0..1000 {
        for c in 0..1000 {
            if lights[r][c] {
                light_counts += 1;
            }
        }
    }
    light_counts
}

fn part_two(contents: &Vec<String>) -> i32 {
    let mut lights = vec![vec![0; 1000]; 1000];
    for c in contents {
        let commands: Vec<String> = c.split(" ").map(|s| s.to_string()).collect();
        match commands[0].as_ref() {
            "toggle" => {
                let start: Vec<usize> = commands[1].split(",").map(|i| i.parse().unwrap()).collect();
                let end: Vec<usize> = commands[3].split(",").map(|i| i.trim().parse().unwrap()).collect();
                for x in start[0]..end[0]+1 {
                    for y in start[1]..end[1]+1 {
                        lights[x][y] += 2;
                    }
                }
            },
            "turn" => {
                if commands[1] == "off" {
                    let start: Vec<usize> = commands[2].split(",").map(|i| i.parse().unwrap()).collect();
                    let end: Vec<usize> = commands[4].split(",").map(|i| i.trim().parse().unwrap()).collect();
                    for x in start[0]..end[0]+1 {
                        for y in start[1]..end[1]+1 {
                            lights[x][y] -= 1;
                            if lights[x][y] < 0 {
                                lights[x][y] = 0;
                            };
                        }
                    }
                } else {
                    let start: Vec<usize> = commands[2].split(",").map(|i| i.parse().unwrap()).collect();
                    let end: Vec<usize> = commands[4].split(",").map(|i| i.trim().parse().unwrap()).collect();
                    for x in start[0]..end[0]+1 {
                        for y in start[1]..end[1]+1 {
                            // println!("{x} {y}");
                            lights[x][y] += 1;
                        }
                    }
                }
            },
            _ => {}
        }
    }

    // now count lights
    let mut light_counts = 0;
    for r in 0..1000 {
        for c in 0..1000 {
            light_counts += lights[r][c];
        }
    }
    light_counts
}

fn main() {
    let contents: Vec<String> = fs::read_to_string("data/day6.txt")
        .expect("Should have been able to read the file")
        .split('\n')
        .map(|s| s.to_string())
        .collect()
        ;

    let result = part_one(&contents);
    println!("Part 1: {:?}", result);

    let result = part_two(&contents);
    println!("Part 2: {result}");

}
