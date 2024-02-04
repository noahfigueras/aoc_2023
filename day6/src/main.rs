
fn main() {
    let time = vec![62, 64, 91, 90];
    let distance = vec![553, 1010, 1473, 1074];

    println!("Part one -> {:?}", part_one(&time, &distance));
    println!("Part two -> {:?}", part_two(62649190, 553101014731074));
}

fn part_one(time: &Vec<i32>, distance: &Vec<i32>) -> i32 {
    let mut total_count: i32 = 0;

    for (i, t) in time.iter().enumerate() {
        let mut i_count = 0;
        for x in 0..*t {
            let f = (t - x) * x;
            if f > distance[i] {
                i_count += 1;
            }
        }
        if i == 0 { total_count = i_count } else { total_count *= i_count };
    }
    total_count
}

fn part_two(time: u64, distance: u64) -> u64 {
    let mut total_count: u64 = 0;
    for x in 0..time {
        let f: u64 = ((time - x) * x).into();
        if f > distance {
            total_count += 1;
        }
    }
    total_count
}
