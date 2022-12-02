use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {

    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    aopdkfpaokspdokf

    let mut max_weight = 0;
    let mut current_weight = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            if current_weight > max_weight {
                max_weight = current_weight;
            }
            current_weight = 0;
        }
        else {
            current_weight += line.parse::<i32>().unwrap();
        }
    }

    println!("max weight is {}", max_weight);
}

fn part2() {

    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut v :Vec<i32> = Vec::new();

    let mut current_weight = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            v.push(current_weight);
            current_weight = 0;
        }
        else {
            current_weight += line.parse::<i32>().unwrap();
        }
    }

    v.sort_by(|a, b| b.cmp(a));

    let mut total = 0;
    for i in 0..3 {
        println!("rank {} has weight {}", i+1, v[i] );
        total += v[i];
    }
    println!("total top 3 is {}", total);

}

fn main() {

    part1();
    part2();
}
