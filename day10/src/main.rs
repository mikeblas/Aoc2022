use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut cycle: i32 = 1;
    let mut x: i32 = 1;
    let mut sum: i32 = 0;
    let mut crt_total: Vec<String> = Vec::new();
    let mut crt_line: Vec<char> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let splits: Vec<&str> = line.split(" ").collect();

        let instruction = splits[0];

        // at the start ...
        let clocks = match instruction {
            "noop" => 1,
            "addx" => 2,
            _ => panic!("unknown instruction")
        };

        // during ...
        for _ in 0..clocks {
            if (cycle + 20) % 40 == 0 {
                println!("cycle {cycle} sampled {x}");
                sum += cycle * x;
            }

            // sprite is 3 wide
            if (cycle-1) % 40 >= x - 1 && (cycle-1) % 40 <= x + 1 {
                crt_line.push('#');
            } else {
                crt_line.push(' ');
            }

            if crt_line.len() == 40 {
                crt_total.push(crt_line.iter().cloned().collect::<String>());
                crt_line.clear();
            }

            cycle += 1;
        }

        // after ...
        if instruction == "addx" {
            x += splits[1].parse::<i32>().unwrap();
        }
    }

    println!("sum is {sum}");
    for line in crt_total {
        println!(">{}<", line);
    }
}

fn part2() {

}

fn main() {
    part1();
    part2();
}
