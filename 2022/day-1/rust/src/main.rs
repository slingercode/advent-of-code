use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Elf {
    number: u8,
    calories: u32,
}

impl Elf {
    fn new(number: u8, calories: u32) -> Self {
        Self { number, calories }
    }

    fn default() -> Self {
        Self::new(0, 0)
    }

    fn reset(&mut self) {
        self.number += 1;
        self.calories = 0;
    }

    fn sum_calories(&mut self, calories: u32) {
        self.calories += calories
    }

    fn compare_elfs(&self, elf1: Elf, elf2: Elf, elf3: Elf) -> Vec<Elf> {
        if self.calories > elf1.calories {
            return vec![self.clone(), elf1, elf2];
        } else if self.calories > elf2.calories {
            return vec![elf1, self.clone(), elf2];
        } else if self.calories > elf3.calories {
            return vec![elf1, elf2, self.clone()];
        } else {
            return vec![elf1, elf2, elf3];
        }
    }
}

fn main() {
    let input = File::open("../input.txt").expect("File read");
    let lines = BufReader::new(input).lines();

    let mut current_elf = Elf::new(1, 0);
    let mut winning_elfs = vec![Elf::default(), Elf::default(), Elf::default()];

    for line in lines {
        match line {
            Ok(entry) => match entry.parse::<u32>() {
                Ok(calories) => current_elf.sum_calories(calories),
                Err(_) => {
                    winning_elfs = current_elf.compare_elfs(
                        winning_elfs[0].clone(),
                        winning_elfs[1].clone(),
                        winning_elfs[2].clone(),
                    );

                    current_elf.reset();
                }
            },
            Err(error) => println!("Error reading the line: {error}"),
        }
    }

    println!("\nWinning elfs: {:?}", winning_elfs);
    println!(
        "Total calories: {:?}",
        winning_elfs.iter().fold(0, |acc, elf| acc + elf.calories)
    )
}
