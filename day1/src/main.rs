use std::fs::File;
use std::io::{ BufRead, BufReader};

const words: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

fn main() {
    let input = File::open("./inputs/day1.txt").unwrap();
    let lines = BufReader::new(input).lines();    
    
    let mut sum = 0;

    for line in lines {
        let l = line.unwrap();
        let number = get_digits(l);
        println!("{number}");
        sum = sum + number; 
    }

    println!("Total sum: {sum}");
}

fn get_digits(l: String) -> i32 {
    let mut first = String::from("");
    let mut last = String::from("");
    let mut tmp = String::from("");

    // find first
    'outer: for c in l.chars() {
        tmp.push(c);
        if c.is_numeric() {
                first = c.to_string(); 
                break
        } else {
            for (i, word) in words.iter().enumerate() {
                if tmp.contains(word) {
                    let value: u8 = (i + 1).try_into().unwrap();
                    first = value.to_string();
                    break 'outer
                }
            }
        }
    }

    tmp = String::from("");

    // find last
    'outer: for c in l.chars().rev() {
        tmp.push(c);
        if c.is_numeric() {
            last = c.to_string(); 
            break
        } else {
            for (i, word) in words.iter().enumerate() {
                let reversed: String = word.chars().rev().collect();
                if tmp.contains(&reversed) {
                    let value: u8 = (i + 1).try_into().unwrap();
                    last = value.to_string();
                    break 'outer
                }
            }
        }
    }

    (first + &last).parse::<i32>().unwrap()
}

