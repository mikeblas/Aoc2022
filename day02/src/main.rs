use std::fs::File;
use std::io::{BufRead, BufReader};

fn score_game(theirs: &str, mine: &str) -> i32 {

    let result =
        match mine {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0
        };

    let winscore =
    match theirs {
        // rock
        "A" =>
            match mine {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => 0
            },

        // paper
        "B" =>
            match mine {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0
            },

        // scissors
        "C" =>
            match mine {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => 0
            },

        _ => 0
    };

    winscore + result
}

fn part1() {

    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut total_score: i32 = 0;
    let mut game_count: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let splits: Vec<&str> = line.split(" ").collect();

        let theirs = splits[0];
        let mine = splits[1];

        let this_score: i32 = score_game(theirs, mine);

        total_score += this_score;
        // println!("{} vs. {}: {} total {}", theirs, mine, this_score, total_score);
        game_count += 1;
    }
    println!("{} games, total {}", game_count, total_score);
}

fn score_game2(theirs: &str, mine: &str) -> i32 {

    let result =
        match mine {
            // lose
            "X" => 0,

            // draw
            "Y" => 3,

            // win
            "Z" => 6,
            _ => 0
        };

    let winscore =
        match theirs {
            // rock
            "A" =>
                match mine {
                    "X" => 3,
                    "Y" => 1,
                    "Z" => 2,
                    _ => 0
                },

            // paper
            "B" =>
                match mine {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => 0
                },

            // scissors
            "C" =>
                match mine {
                    "X" => 2,
                    "Y" => 3,
                    "Z" => 1,
                    _ => 0
                },

            _ => 0
        };

    winscore + result
}


fn part2() {
    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut total_score: i32 = 0;
    let mut game_count: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let splits: Vec<&str> = line.split(" ").collect();

        let theirs = splits[0];
        let mine = splits[1];

        let this_score: i32 = score_game2(theirs, mine);

        total_score += this_score;
        // println!("{} vs. {}: {} total {}", theirs, mine, this_score, total_score);
        game_count += 1;
    }
    println!("{} games, total {}", game_count, total_score);
}

fn main() {
    part1();
    part2();

}
