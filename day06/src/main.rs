use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn part1() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);


    for line in reader.lines() {
        let line = line.unwrap();
        let chars :Vec<char>= line.chars().collect();

        for idx in 0..chars.len()-4 {

            if chars[idx] == chars[idx+1] || chars[idx] == chars[idx+2] {
                continue;
            }
            if chars[idx] == chars[idx+3] || chars[idx+2] == chars[idx+3] {
                continue;
            }
            if chars[idx+1] == chars[idx+2] || chars[idx+1] == chars[idx+3] {
                continue;
            }

            println!("found at {}", idx+4);
            break;
        }
    }

}

fn part2() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let chars :Vec<char>= line.chars().collect();

        for idx in 0..chars.len()-14 {

            let mut hash:HashMap<&char, bool> = HashMap::new();

            let mut startpoint = idx;

            for off in 0..14 {
                let ch :&char = &chars[idx+off];
                if hash.contains_key(ch) {
                    startpoint = 0;
                    break;
                } else {
                    hash.insert(ch, true);
                }
            }

            if startpoint != 0 {
                println!("found at {}", idx+14);
                break;
            }
        }
    }
}

fn main() {
    part1();
    part2();
}