use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let file = File::open("src/sample1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }

    for idx in (0..lines.len()).step_by(7) {
        // println!("{}", lines[idx]);

        // monkey ID
        // monkey 0:
        let space = lines[idx].find(' ').unwrap();
        let monkey_id =  lines[idx][space+1..space+2].parse::<usize>().unwrap();
        println!("monkey {monkey_id}");

        // Starting items: 79, 98
        let colon = lines[idx+1].find(':').unwrap();
        let items :Vec<i32> = lines[idx+1][colon+2..].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
        println!(" items: ({}), {:?}", items.len(), items);

        // Operation: new = old * 19
        let equals = lines[idx+2].find('=').unwrap();
        let ops :Vec<&str> = lines[idx+2][equals+2..].split(' ').collect();
        println!(" ops: ({}), {:?}", ops.len(), ops);

        // Test: divisible by 23
        let by = lines[idx+3].find(" by ").unwrap();
        let divisor = lines[idx+3][by+4..].parse::<i32>().unwrap();
        println!(" divisor: {}", divisor);

        // If true: throw to monkey 2
        // If false: throw to monkey 0
        let monkey_true = lines[idx+4].find("monkey").unwrap();
        let monkey_false = lines[idx+5].find("monkey").unwrap();

        let true_target = lines[idx+4][monkey_true+7..].parse::<i32>().unwrap();
        let false_target = lines[idx+5][monkey_false+7..].parse::<i32>().unwrap();
        println!("  true {true_target}, false {false_target}");

    }


}

fn part2() {

}

fn main() {
    part1();
    part2();
}
