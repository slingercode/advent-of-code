use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

enum Result {
    Win,
    Draw,
    Lose,
}

fn get_object_points(entry: &String) -> u32 {
    let char = entry.chars().nth(1).unwrap();

    if char == 'X' {
        return 1;
    } else if char == 'Y' {
        return 2;
    } else {
        return 3;
    }
}

fn play_game(entry: &String) -> u32 {
    let mut game_result: u32 = 0;

    let true_table: HashMap<String, Result> = HashMap::from([
        (String::from("AX"), Result::Draw),
        (String::from("BX"), Result::Lose),
        (String::from("CX"), Result::Win),
        (String::from("AY"), Result::Win),
        (String::from("BY"), Result::Draw),
        (String::from("CY"), Result::Lose),
        (String::from("AZ"), Result::Lose),
        (String::from("BZ"), Result::Win),
        (String::from("CZ"), Result::Draw),
    ]);

    match true_table.get(entry) {
        Some(result) => match result {
            Result::Win => game_result = 6,
            Result::Draw => game_result = 3,
            Result::Lose => {}
        },
        None => {}
    }

    return game_result + get_object_points(entry);
}

fn main() {
    let input = File::open("../input.txt").expect("File read");
    let lines = BufReader::new(input).lines();

    let mut score: u32 = 0;

    for line in lines {
        match line {
            Ok(entry) => {
                let cleaned_entry = entry.replace(" ", "");
                score += play_game(&cleaned_entry);
            }
            Err(error) => println!("Error reading the line: {error}"),
        }
    }

    println!("{score}");
}
