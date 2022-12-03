#![allow(unused_parens)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn char_to_priority(ch:char) -> i32 {

    let mut pri:i32 = 0;

    if ch >= 'a' && ch <= 'z' {
        // ch as i32 - 'a' as i32 + 1
        pri = ch as i32 - 'a' as i32 + 1;
    }

    if ch >= 'A' && ch <= 'Z' {
        // ch as i32 - 'A' as i32 + 27
        pri = ch as i32 - 'A' as i32 + 27;
    }

    pri
}

fn part1() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut total_priority: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let chars:Vec<char> = line.chars().collect();
        let splits: Vec<&[char]> = chars.chunks(line.len()/2).collect();

        let mut left:HashMap<&char, i32> = HashMap::new();
        let mut right:HashMap<&char, i32> = HashMap::new();

        // println!("{} chars, {} vecs, {}", line.len(), chars.len(), chars[3]);

        for ch in splits[0] {
            left.insert(ch, 1);
        }

        for ch in splits[1] {
            right.insert(ch, 1);
        }

        for (chkey, _) in left {
            if right.contains_key(chkey) {
                total_priority += char_to_priority(*chkey);
                // println!("found {chkey}");
            }
        }
    }

    println!("total priority is {}", total_priority);
}

fn part2() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut total_priority: i32 = 0;
    let mut three:Vec<String> = Vec::new();


    for (line_no, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        println!("{}: {}", line_no, line);

        three.push(line);

        if three.len() == 3 {
            let mut total:HashMap<char, i32> = HashMap::new();

            for (row_no, one) in three.iter().enumerate() {

                for ch in one.chars() {
                    *total.entry(ch).or_insert(0) |= (1 << row_no);
                }
            }

            for (ch, mask) in total {
                if mask == 7 {
                    println!("found {ch}");
                    total_priority += char_to_priority(ch);
                }
            }


            three.clear();
        }
    }

    println!("total priority is {total_priority}");
}

fn main() {
    part1();
    part2();
}
