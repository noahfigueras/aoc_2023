use std::fs::File;
use std::io::{ BufRead, BufReader, Lines};

fn main() {
    let input = File::open("./inputs/day5.txt").unwrap();
    let lines = BufReader::new(input).lines();    

    let mut seeds: String = String::new();
    let mut lowest_location: u32 = 0;
    let mut lowest_location_tmp: u32 = 0;

    for (i, line) in lines.enumerate() {
        let l = line.unwrap();
        let map: Vec<&str> = l.split(" ").collect();

        if i == 0 {
            seeds = l;
            continue;
        } else if l.is_empty() {
            continue;
        }

        // Map seeds
        let _seeds: Vec<&str> = seeds.split(" ").collect();
        for seed in _seeds[1..].to_vec() {
            let seed_num: u32 = seed.parse().unwrap();
            let destination = match map[0].parse::<u32>() {
                Ok(n) => n,
                Err(_e) => continue,
            };
            let source = match map[1].parse::<u32>() {
                Ok(n) => n,
                Err(_e) => continue,
            };
            let length = match map[2].parse::<u32>() {
                Ok(n) => n,
                Err(_e) => continue,
            };
            println!("{seed_num}, {source}, {destination}, {length}");
        }
    }
}


