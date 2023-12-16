use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    let input = File::open("./inputs/day4.txt").unwrap();
    let lines = BufReader::new(input).lines();    

    let mut total_points = 0;
    let mut scratchcards: Vec<Vec<u32>>= vec![vec![];209];

    for (i, line) in lines.enumerate() {
        let l = line.unwrap();

        let card: Vec<&str> = l.split(":").collect();
        let card_numbers: Vec<&str> = card[1].split("|").collect();
        let winning: Vec<&str> = card_numbers[0].split(" ").collect();
        let numbers: Vec<&str> = card_numbers[1].split(" ").collect();

        let mut points = 0;
        let mut first: bool = true;
        let mut match_count = 0;

        for number in numbers {
            let n: u32 = match number.parse() {
                Ok(x) => x,
                Err(_x) => 0,
            };
            for w in &winning {
                let wn: u32 = match w.parse() {
                    Ok(x) => x,
                    Err(_x) => 0,
                };
                if wn == 0 {
                    continue;
                } else if first && n == wn {
                    match_count += 1;
                    points += 1;
                    first = false;
                } else if n == wn {
                    match_count += 1;
                    points *= 2;
                }
            }
        }

        for m in 0..match_count {
            let id = (i + 1) + (m + 1);
            scratchcards[i+1].push(id.try_into().unwrap());
        }

        total_points += points;
    }

    println!("How many points are they worth in total? {total_points}");
    println!(
        "how many total scratchcards do you end up with? {}", 
         scratchcard_count(&scratchcards, scratchcards.clone(), &mut 0) - 1
    );
}

fn scratchcard_count(
    scratchcards: &Vec<Vec<u32>>, 
    new: Vec<Vec<u32>>, 
    total: &mut usize
) -> usize {
    *total += new.len();
    for card in &new {
        let mut new_vec: Vec<Vec<u32>> = vec![]; 
        for number in card {
            new_vec.push(scratchcards[*number as usize].clone());
        }
        scratchcard_count(scratchcards, new_vec.clone(), total);
    }
    *total
}

