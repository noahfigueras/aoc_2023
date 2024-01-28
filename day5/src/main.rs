use std::fs;
use std::cmp;

fn main() {
    let input = fs::read_to_string("./inputs/day5.txt").unwrap();
    let segments: Vec<&str> = input.split("\n\n").collect();

    println!("Part 1: {}", part_one(&segments)); 
    println!("Part 2: {}", part_two(&segments)); 
}

fn part_two(segments: &Vec<&str>) -> i64 {
    let binding = segments[0].split(": ")
        .collect::<Vec<&str>>()[1]
        .split(" ").collect::<Vec<&str>>();
    let mut seeds: Vec<i64> = binding.iter().map(|x| match x.parse::<i64>() {
        Ok(n) => n,
        Err(e) => 0,
    }).collect();

    let mut ranges: Vec<(i64, i64, usize)> = vec![];
    for (i, start) in seeds.iter().enumerate().step_by(2) {
       let length = seeds[i+1];
       let end = start + length;
       ranges.push((*start, end , 1)); 
    }

    let mut min = 429496729500000;
    while ranges.len() > 0 {
        let (mut start, mut end, mut level) = match ranges.pop() {
            Some(x) => x,
            None => (0,0,0),
        };
        if level == 8 {
            min = cmp::min(start, min);
            continue; 
        }
        
        let mut done = false;
        let segment: Vec<_> = segments[level].split("\n").collect();
        'inner: for map in &segment[1..] {
            if map.is_empty() { continue; }
            let (destination, source, length) = parse_segment(map);
            let diff = destination - source;

            if end <= source || start >= (source + length) {
                continue;
            }
            if start < source {
                ranges.push((start, source, level));
                start = source;
            }
            if (source + length) < end {
                let y = source + length;
                ranges.push((y, end, level));
                end = y;
            }
            ranges.push((start + diff, end + diff, level + 1));
            done = true;
            break 'inner;
        } 
        if !done { 
            ranges.push((start, end, level + 1));
        }
    }
    min
}

fn part_one(segments: &Vec<&str>) -> i64 {
    let binding = segments[0].split(": ")
        .collect::<Vec<&str>>()[1]
        .split(" ").collect::<Vec<&str>>();
    let mut seeds: Vec<i64> = binding.iter().map(|x| match x.parse::<i64>() {
        Ok(n) => n,
        Err(e) => 0,
    }).collect();

    let mut min = 429496729500000;
    for mut seed in seeds {
        let mut level = 1;
        while level < 8 {
            let segment: Vec<_> = segments[level].split("\n").collect();
            'inner: for map in &segment[1..] {
                if map.is_empty() { continue; }
                let (destination, source, length) = parse_segment(map);
                if seed >= source && seed < ((source + length)){
                    let new = destination + (seed - source);
                    seed = new.clone();
                    break 'inner;
                }
            }
            level += 1;
        }
        min = cmp::min(seed, min);
    }

    min
}

fn parse_segment(segment: &str) -> (i64, i64, i64) {
    let binding = segment.to_string();
    let x: Vec<&str> = binding.split(" ").collect();
    (
    x[0].parse::<i64>().unwrap(), 
    x[1].parse::<i64>().unwrap(),
    x[2].parse::<i64>().unwrap()
    )
}
