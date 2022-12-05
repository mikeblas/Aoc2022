use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut vecvec :Vec<Vec<char>> = Vec::new();

    let mut mode = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        // println!("{mode}, {}, {line}", line.len());

        if mode == 0 {
            for x in 0..1 + (line.len() / 4) {
                if x >= vecvec.len() {
                    let v: Vec<char> = Vec::new();
                    vecvec.push(v);
                }

                let string_index = 1 + x * 4;

                let ch = line.chars().nth(string_index).unwrap();
                println!("    {x}, {string_index}: '{}'", ch);

                if ch != ' ' {
                    if ch == '1' {
                        mode = 1;
                        break;
                    }
                    vecvec[x].insert(0, ch);
                }
            }
        }
        else if mode == 1 {
            if line.len() == 0 {
                continue;
            }

            println!("---------");
            for v in &vecvec {
                for ch in v {
                    print!("{ch},");
                }
                println!("");
            }

            let from_start = line.find("from").unwrap();
            let to_start = line.find("to").unwrap();

            let count = line[5..from_start-1].parse::<usize>().unwrap();
            let source = line[from_start+5..to_start-1].parse::<usize>().unwrap();
            let dest = line[to_start+3..].parse::<usize>().unwrap();

            println!("{count}, {source}, {dest}");

            for _ in 0..count {
                let t = vecvec[source-1].pop().unwrap();
                vecvec[dest-1].push(t);
            }
        }
    }

    let mut total = String::from("");
    println!("---------");
    for v in &vecvec {
        for ch in v {
            print!("{ch},");
        }
        println!("");

        total.push(*v.last().unwrap());
    }

    println!("total = {total}");
}

fn part2() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut vecvec :Vec<Vec<char>> = Vec::new();

    let mut mode = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        // println!("{mode}, {}, {line}", line.len());

        if mode == 0 {
            for x in 0..1 + (line.len() / 4) {
                if x >= vecvec.len() {
                    let v: Vec<char> = Vec::new();
                    vecvec.push(v);
                }

                let string_index = 1 + x * 4;

                let ch = line.chars().nth(string_index).unwrap();
                println!("    {x}, {string_index}: '{}'", ch);

                if ch != ' ' {
                    if ch == '1' {
                        mode = 1;
                        break;
                    }
                    vecvec[x].insert(0, ch);
                }
            }
        }
        else if mode == 1 {
            if line.len() == 0 {
                continue;
            }

            println!("---------");
            for v in &vecvec {
                for ch in v {
                    print!("{ch},");
                }
                println!("");
            }

            let from_start = line.find("from").unwrap();
            let to_start = line.find("to").unwrap();

            let count = line[5..from_start-1].parse::<usize>().unwrap();
            let source = line[from_start+5..to_start-1].parse::<usize>().unwrap();
            let dest = line[to_start+3..].parse::<usize>().unwrap();

            println!("{count}, {source}, {dest}");

            let lastcount = vecvec[source-1].len()-count;
            let mut u = vecvec[source-1].drain(lastcount..).collect();
            vecvec[dest-1].append(&mut u);
        }
    }

    println!("---------");
    for v in &vecvec {
        for ch in v {
            print!("{ch},");
        }
        println!("");
    }

    let total2 :String = vecvec.iter().map(|v| v[v.len()-1]).collect();
    println!("total2 = {total2}");
}

fn main() {
    part1();
    part2();
}
