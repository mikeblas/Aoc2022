use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

// iterator that knows a step count and will go from start to size
// or start to 0
struct SimpleStepRange(usize, usize, isize);  // start, end, and step

impl Iterator for SimpleStepRange {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        // print!(" iter: {}, {}, {} returns ", self.0, self.1, self.2);
        // if self.0 >= 0 && ((self.0 < self.1 && self.2 > 0) || (self.0 > self.1 && self.2 < 0)) {
        if /* self.0 != usize::MAX && */ self.0 < self.1 {
            let v = self.0;
            self.0 = ((v as isize) + self.2) as usize;
            // println!("{v}");
            Some(v)
        } else {
            // println!("None");
            None
        }
    }
}

// make a SimpleStepRange given start, end and step;
// start and end don't need to be in the right order
fn makeiter(a :usize, b:usize, c:isize) -> SimpleStepRange {
    // println!("makeiter: {}, {}, {}", a, b, c);
    SimpleStepRange( ((a as isize) + c) as usize, b, c)
}


fn part1() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    // get the file into this table
    let mut table :Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line_chars: Vec<char> = line.chars().collect();

        let mut this_line :Vec<i32> = Vec::new();
        for ch in line_chars {
            this_line.push(ch.to_digit(10).unwrap() as i32);
        }

        table.push(this_line);
    }

    // set of visible points
    let mut visible = HashSet::new();

    // visible from left
    for y in 0..table.len() {

        let mut previous_height = -1;

        for x in 0..table[y].len() {
            if previous_height < table[y][x] {
                previous_height = table[y][x];
                visible.insert((x,y));
                // println!("{x},{y} visible from left");
            } else {
            }
        }
    }

    // visible from right
    for y in 0..table.len() {

        let mut previous_height = -1;

        for x in (0..table[y].len()).rev() {
            if previous_height < table[y][x] {
                previous_height = table[y][x];
                visible.insert((x,y));
                // println!("{x},{y} visible from right");
            } else {
            }
        }
    }

    // visible from top
    for x in 0..table[0].len() {

        let mut previous_height = -1;

        for y in 0..table.len() {
            if previous_height < table[y][x] {
                previous_height = table[y][x];
                visible.insert((x,y));
            } else {
                /*
                if previous_height > table[y][x] {
                    break;
                }
                 */
            }
        }
    }

    // visible from bottom
    for x in 0..table[0].len() {

        let mut previous_height = -1;

        for y in (0..table.len()).rev() {
            if previous_height < table[y][x] {
                previous_height = table[y][x];
                visible.insert((x,y));
            } else {
            }
        }
    }

    /*
    for y in 0..table.len() {
        for x in 0..table[0].len() {
            if visible.contains(&(x,y)) {
                print!("V");
            } else {
                print!(" " );
            }
        }
        println!();
    }
     */

    println!("{} were visible", visible.len());
}

fn part2() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    // get the file into this table
    let mut table :Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line_chars: Vec<char> = line.chars().collect();

        let mut this_line :Vec<i32> = Vec::new();
        for ch in line_chars {
            this_line.push(ch.to_digit(10).unwrap() as i32);
        }

        table.push(this_line);
    }

    let mut best_score: i32 = 0;

    for y in 0..table.len() {
        for x in 0..table[0].len()  {
            let mut total_score = 1;
            for dx in vec![-1, 1]  {
                let mut view_distance = 0;
                for qx in makeiter(x, table[0].len(), dx) {
                    // println!("dx = {dx}: {x},{y} to {x},{qx}");
                    view_distance += 1;
                    if table[y][qx] >= table[y][x] {
                        // println!("break! {} >= {}", table[y][qx], table[y][x]);
                        break;
                    }
                }

                // println!("dx = {dx}: {x},{y} scores {view_distance}");
                total_score *= view_distance;
            }
            for dy in vec![-1, 1] {
                let mut view_distance = 0;
                for qy in makeiter(y, table.len(), dy) {
                    view_distance += 1;
                    if table[qy][x] >= table[y][x] {
                        break;
                    }
                }

                // println!("dy = {dy}: {x},{y} scores {view_distance}");
                total_score *= view_distance;
            }

            // println!("score at {x},{y} is {total_score}");

            if best_score < total_score {
                best_score = total_score;
            }
        }
    }

    println!("best score is {best_score}");
}

fn main() {
    part1();
    part2();

    /*
    for dx in vec![-1, 1] {
        println!("==== {dx}");
        for i in makeiter(5, 10, dx) {
            println!("{}", i);
        }
    }
     */
}
