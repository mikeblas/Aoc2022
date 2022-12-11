use std::fs::File;
use std::io::{BufRead, BufReader};
use num::integer::lcm;

fn lcm_list(numbers: Vec<i64>) -> Option<i64> {

    if numbers.len() == 0 {
        None
    } else if numbers.len() == 1 {
        Some(numbers[0])
    } else {

        let mut ret = lcm(numbers[0], numbers[1]);

        for idx in 2..numbers.len() {
            ret = lcm(ret, numbers[idx]);
        }

        Some(ret)
    }
}

struct Monkey {
    id: usize,
    items: Vec<i64>,
    ops: Vec<String>,
    divisor: i64,
    true_target: usize,
    false_target: usize,
    inspection_count: usize
}

fn part1(part_two: bool) {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }

    let mut monkeys: Vec<Monkey> = Vec::new();

    for idx in (0..lines.len()).step_by(7) {
        // println!("{}", lines[idx]);

        // monkey ID
        // monkey 0:
        let space = lines[idx].find(' ').unwrap();
        let monkey_id =  lines[idx][space+1..space+2].parse::<usize>().unwrap();
        // println!("monkey {monkey_id}");

        // Starting items: 79, 98
        let colon = lines[idx+1].find(':').unwrap();
        let items :Vec<i64> = lines[idx+1][colon+2..].split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
        // println!(" items: ({}), {:?}", items.len(), items);

        // Operation: new = old * 19
        let equals = lines[idx+2].find('=').unwrap();
        let ops :Vec<String> = lines[idx+2][equals+2..].split(' ').map(str::to_string).collect();
        // println!(" ops: ({}), {:?}", ops.len(), ops);

        // Test: divisible by 23
        let by = lines[idx+3].find(" by ").unwrap();
        let divisor = lines[idx+3][by+4..].parse::<i64>().unwrap();
        // println!(" divisor: {}", divisor);

        // If true: throw to monkey 2
        // If false: throw to monkey 0
        let monkey_true = lines[idx+4].find("monkey").unwrap();
        let monkey_false = lines[idx+5].find("monkey").unwrap();

        let true_target = lines[idx+4][monkey_true+7..].parse::<usize>().unwrap();
        let false_target = lines[idx+5][monkey_false+7..].parse::<usize>().unwrap();
        // println!("  true {true_target}, false {false_target}");

        let m = Monkey { id: monkey_id, items, ops, divisor, true_target, false_target, inspection_count: 0 };
        monkeys.push(m);
    }

    let divisors = monkeys.iter().map(|m| m.divisor).collect();
    let divisor_lcm = lcm_list(divisors).unwrap();
    println!("divisor_lcm is {divisor_lcm}");

    // 20 rounds for part1, 10000 rounds in part 2
    let rounds = if part_two { 10000 } else { 20 };

    for _ in 0..rounds {

        for n in 0..monkeys.len() {

            for item in monkeys[n].items.clone() {
                // let old = m.items.remove(0);
                let old = item;

                let right = if monkeys[n].ops[2] == "old" {
                    old
                }  else {
                    monkeys[n].ops[2].parse::<i64>().unwrap()
                };

                let mut new = match monkeys[n].ops[1].as_str() {
                    "+" => old + right,
                    "*" => old * right,
                    _ => panic!("unknown operand")
                };

                if !part_two {
                     new = new / 3;
                } else {
                    new = new % divisor_lcm;
                }

                let remainder = new % monkeys[n].divisor;

                if remainder == 0 {
                    let t = monkeys[n].true_target;
                    monkeys[t].items.push(new)
                } else {
                    let t = monkeys[n].false_target;
                    monkeys[t].items.push(new)
                }

                // println!("was {old} now {new}, remainder {remainder}");
            }

            monkeys[n].inspection_count += monkeys[n].items.len();
            monkeys[n].items.clear();
        }
    }

    let mut counts = Vec::new();
    for m in monkeys {
        println!("{} inspected {}", m.id, m.inspection_count);
        counts.push(m.inspection_count);
    }

    // sort descending
    counts.sort_by(|a, b| b.cmp(a));

    println!("top two are {} {}", counts[0], counts[1]);
    println!("result is {}", counts[0] * counts[1]);
}

fn main() {
    part1(false);
    part1(true);
}
