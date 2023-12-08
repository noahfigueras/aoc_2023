use std::fs::File;
use std::io::{ BufRead, BufReader, Result};

#[derive(Debug)]
struct Number {
    number: u32,
    pos: Vec<i32>,
}

fn main() {
    let input = File::open("./inputs/day3.txt").unwrap();
    let lines: Vec<String> = BufReader::new(input)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let mut sum = 0;
    let mut gear_ratios = 0;

    // Part one
    for (i, line) in lines.iter().enumerate() {
        let numbers: Vec<Number> = get_numbers(line);
        for number in numbers.iter() {
            for x in (number.pos[0] - 1)..(number.pos[1] + 2) {
                if i > 0 && is_symbol(&lines[i-1], x) {
                    sum += number.number;
                    break;
                }
                if i < (lines.len() - 1) && is_symbol(&lines[i+1], x){
                    sum += number.number;
                    break;
                }
                if is_symbol(line, x) {
                    sum += number.number;
                    break;
                }
            }
        }
    }
    
    // Part two 
    for (x, line) in lines.iter().enumerate() {
        for (y, character) in line.chars().enumerate() {
            if character == '*' {
                let mut numbers: Vec<u32> = vec![];
                for j in (y-1)..(y + 2) {
                    let ij = j.try_into().unwrap();
                    // Top
                    if x > 0 {
                        for number in get_numbers(&lines[x-1]).iter() {
                            if number.pos[0] <= ij && number.pos[1] >= ij {
                                if !numbers.contains(&number.number) {
                                    numbers.push(number.number);
                                }
                            }
                        }
                    }
                    // Bottom
                    if x < (lines.len() - 1) {
                        for number in get_numbers(&lines[x+1]).iter() {
                            if number.pos[0] <= ij && number.pos[1] >= ij {
                                if !numbers.contains(&number.number) {
                                    numbers.push(number.number);
                                }
                            }
                        }
                    }
                    // Inline
                    for number in get_numbers(line).iter() {
                        if number.pos[1] == ij {
                            if !numbers.contains(&number.number) {
                                numbers.push(number.number);
                            }
                        };
                        if number.pos[0] == ij {
                            if !numbers.contains(&number.number) {
                                numbers.push(number.number);
                            }
                        };
                    }
                }
                if numbers.len() == 2 {
                    gear_ratios += numbers[0] * numbers[1];    
                } 
            }
        }
    }

    println!(
        "What is the sum of all of the part numbers in the engine schematic? {}", 
        sum
    );
    println!(
        "What is the sum of all of the gear ratios in your engine schematic? {}",
        gear_ratios
    );
}

fn get_numbers(line: &String) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut digits: String = String::from("");

    for (i, character) in line.chars().enumerate() {
        if character.is_numeric() { 
            digits.push(character);
            if (line.len() - 1) == i {
                numbers.push(
                    Number {
                        number: digits.parse().unwrap(),
                        pos: vec![
                            (i - digits.len() + 1).try_into().unwrap(), 
                            (i).try_into().unwrap()
                        ]
                    }
                );
            }
        } else if digits.len() > 0 {
            numbers.push(
                Number {
                    number: digits.parse().unwrap(),
                    pos: vec![
                        (i - digits.len()).try_into().unwrap(), 
                        (i - 1).try_into().unwrap()
                    ]
                }
            );
            digits = String::from("");
        }
    }
    numbers
}

fn is_symbol(character: &String, index: i32) -> bool {
    if index < 0 || index >= character.len().try_into().unwrap() { return false; }
    let symbol = character.chars().nth(index.try_into().unwrap()).unwrap();
    if !symbol.is_numeric() && symbol != '.' {
        return true;
    }
    return false;
}
