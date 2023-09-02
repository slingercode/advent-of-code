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

    let mut current_elf = Elf {
        number: 1,
        calories: 0,
    };

    let mut winning_elfs = vec![
        Elf {
            number: 0,
            calories: 0,
        },
        Elf {
            number: 0,
            calories: 0,
        },
        Elf {
            number: 0,
            calories: 0,
        },
    ];

    for line in entries {
        match line {
            Ok(line) => match line.parse::<u32>() {
                Ok(value) => current_elf.calories += value,
                Err(_) => {
                    println!("Current elf: {:?}", current_elf);

                    if current_elf.calories > winning_elfs[0].calories {
                        winning_elfs = vec![
                            Elf {
                                number: current_elf.number,
                                calories: current_elf.calories,
                            },
                            Elf {
                                number: winning_elfs[0].number,
                                calories: winning_elfs[0].calories,
                            },
                            Elf {
                                number: winning_elfs[1].number,
                                calories: winning_elfs[1].calories,
                            },
                        ]
                    } else if current_elf.calories > winning_elfs[1].calories {
                        winning_elfs = vec![
                            Elf {
                                number: winning_elfs[0].number,
                                calories: winning_elfs[0].calories,
                            },
                            Elf {
                                number: current_elf.number,
                                calories: current_elf.calories,
                            },
                            Elf {
                                number: winning_elfs[1].number,
                                calories: winning_elfs[1].calories,
                            },
                        ]
                    } else if current_elf.calories > winning_elfs[2].calories {
                        winning_elfs = vec![
                            Elf {
                                number: winning_elfs[0].number,
                                calories: winning_elfs[0].calories,
                            },
                            Elf {
                                number: winning_elfs[1].number,
                                calories: winning_elfs[1].calories,
                            },
                            Elf {
                                number: current_elf.number,
                                calories: current_elf.calories,
                            },
                        ]
                    }

                    current_elf.number += 1;
                    current_elf.calories = 0;
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
