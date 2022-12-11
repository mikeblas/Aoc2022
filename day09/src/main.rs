use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
// use std::option;

fn part1() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);

    let mut tailplaces = HashSet::new();
    tailplaces.insert(tail);

    for line in reader.lines() {
        let line = line.unwrap();

        let splits: Vec<&str> = line.split(" ").collect();

        let direction = splits[0].chars().next().unwrap();
        let dist = splits[1].parse::<i32>().unwrap();

        // println!("{direction}: {dist}");

        for _ in 0..dist {

            let newhead =
                match direction {
                    'U' => (head.0, head.1 + 1),
                    'D' => (head.0, head.1 - 1),
                    'L' => (head.0 - 1, head.1),
                    'R' => (head.0 + 1, head.1),
                    _ => panic!("unknown direction")
                };

            if (newhead.0 - tail.0).abs() > 1 || (newhead.1 - tail.1).abs() > 1 {
                tail = head;
            }

            head = newhead;
            tailplaces.insert(tail);
        }
    }

    println!("total of {} tail places", tailplaces.len());
}

fn part2() {
    let file = File::open("src/sample2.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    // initialize a vector of 10 knots; knot[0] is the head
    let mut knots :Vec<(i32, i32)> = Vec::new();
    for _ in 0..10 {
        knots.push((0i32, 0i32));
    }

    // use a set for the places the last tail has been
    let mut tailplaces = HashSet::new();
    tailplaces.insert(knots[9]);

    for line in reader.lines() {
        let line = line.unwrap();

        let splits: Vec<&str> = line.split(" ").collect();

        let direction = splits[0].chars().next().unwrap();
        let dist = splits[1].parse::<i32>().unwrap();

        println!("===================");
        println!("{direction}: {dist}");

        for _ in 0..dist {

            let mut temp =
                match direction {
                    'U' => (knots[0].0,     knots[0].1 + 1),
                    'D' => (knots[0].0,     knots[0].1 - 1),
                    'L' => (knots[0].0 - 1, knots[0].1),
                    'R' => (knots[0].0 + 1, knots[0].1),
                    _ => panic!("unknown direction")
                };

            for n in 0..knots.len()-1 {

                let mut newtemp : Option<(i32, i32)> = None;

                // does it need to move? If so, set newtemp to new location
                if (temp.0 - knots[n+1].0).abs() > 1 {
                    newtemp = Some( (knots[n+1].0 - temp.0, knots[n+1].1));
                }
                else if (temp.1 - knots[n+1].1).abs() > 1 {
                    newtemp = Some( (knots[n+1].0, knots[n+1].1));
                }

                if (temp.0 - knots[n+1].0).abs() > 1 || (temp.1 - knots[n+1].1).abs() > 1 {
                    if temp.0 == knots[n+1].0 {
                        newtemp = (knots)
                    }
                    newtemp = Some(knots[n].clone());
                }

                knots[n] = temp.clone();
                if newtemp == None {
                    break;
                }
                temp = newtemp.unwrap();
            }

            tailplaces.insert(knots[9]);
            println!("knots[0] at {}, {}", knots[0].0, knots[0].1);
            println!(" tail at {}, {}", knots[9].0, knots[9].1);
        }
    }

    println!("total of {} tail places", tailplaces.len());
}

fn main() {
    part1();
    part2();
}


