use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Elf {
    number: u8,
    calories: u32,
}

fn main() {
    let input = File::open("../input.txt").expect("File read");
    let entries = BufReader::new(input).lines();

    let mut winning_elf = Elf {
        number: 0,
        calories: 0,
    };

    let mut current_elf = Elf {
        number: 1,
        calories: 0,
    };

    for line in entries {
        match line {
            Ok(line) => match line.parse::<u32>() {
                Ok(value) => current_elf.calories += value,
                Err(_) => {
                    if current_elf.calories > winning_elf.calories {
                        winning_elf.number = current_elf.number;
                        winning_elf.calories = current_elf.calories;
                    }

                    current_elf.number += 1;
                    current_elf.calories = 0;
                }
            },
            Err(error) => println!("Error reading the line: {error}"),
        }
    }

    println!("\nWinning elf: {:?}", winning_elf);
}
