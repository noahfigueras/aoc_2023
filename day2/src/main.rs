use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    let input = File::open("./inputs/day2.txt").unwrap();
    let lines = BufReader::new(input).lines();    
    
    let mut total_games = 0;
    let mut total_power = 0;

    for line in lines {
        let l = line.unwrap();

        let game: Vec<&str> = l.split(":").collect();
        let game_id: Vec<&str> = game[0].split(" ").collect();
        let gid: u32 = game_id[1].parse().unwrap();
        let sets: Vec<&str> = game[1].split(";").collect();

        let mut impossible = false;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in sets {                
            let cubes: Vec<&str> = set.split(",").collect();
            for cube in cubes {
                let digit: Vec<&str> = cube.split(" ").collect();
                let number: u32 = digit[1].parse().unwrap();
                if cube.contains("red") { 
                    if number > 12 { impossible = true; }
                    if number > red { red = number; }
                } else if cube.contains("green") {
                    if number > 13 { impossible = true; }
                    if number > green { green = number; }
                } else if cube.contains("blue") {
                    if number > 14 { impossible = true; }
                    if number > blue { blue = number; }
                }
            }
        }
        if !impossible {
            total_games = total_games + gid;
        }
        total_power = total_power + (red * green * blue);
    }

    println!("Solution 1: {total_games}");
    println!("Solution 2: {total_power}");
}
