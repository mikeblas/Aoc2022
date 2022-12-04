use std::fs::File;
use std::io::{BufRead, BufReader};

fn splitup(s: &str) -> (i32, i32) {

    let postsplits: Vec<&str> = s.split("-").collect();
    // println!("{} {}", postsplits[0], postsplits[1]);

    let start: i32 = postsplits[0].parse().unwrap();
    let end: i32 = postsplits[1].parse().unwrap();

    (start, end)
}

fn part1() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut overlapping: i32 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();

        let presplits: Vec<&str> = line.split(",").collect();

        let (start1, end1) = splitup(presplits[0]);
        let (start2, end2) = splitup(presplits[1]);
        // println!("{line}: {start1} {end1} and {start2} {end2}");

        if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
            overlapping += 1;
        }
    }

    println!("total overlapping is {overlapping}");
}

fn part2() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut overlapping: i32 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();

        let presplits: Vec<&str> = line.split(",").collect();

        let (start1, end1) = splitup(presplits[0]);
        let (start2, end2) = splitup(presplits[1]);
        // println!("{line}: {start1} {end1} and {start2} {end2}");
        // println!("{line}:");

        if (start1 >= start2 && start1 <= end2) || (end1 >= start2 && end1 <= end2) {
            // println!("{line}: {start1} {end1} and {start2} {end2}");
            overlapping += 1;
        } else if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
            overlapping += 1;
        }
    }

    println!("total overlapping is {overlapping}");
}

fn main() {
    part1();
    part2();
}
