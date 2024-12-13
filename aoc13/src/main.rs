use std::cmp::{max, min};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let machines = contents
        .trim()
        .split("\n\n")
        .map(|m| {
            let lines = m.trim().lines().collect::<Vec<&str>>();
            let a_split = lines[0].split(&['+', ',']).collect::<Vec<&str>>();
            let b_split = lines[1].split(&['+', ',']).collect::<Vec<&str>>();
            let p_split = lines[2].split(&['=', ',']).collect::<Vec<&str>>();
            return (
                (
                    a_split[1].parse::<u64>().unwrap(),
                    a_split[3].parse::<u64>().unwrap(),
                ),
                (
                    b_split[1].parse::<u64>().unwrap(),
                    b_split[3].parse::<u64>().unwrap(),
                ),
                (
                    p_split[1].parse::<u64>().unwrap(),
                    p_split[3].parse::<u64>().unwrap(),
                ),
            );
        })
        .collect::<Vec<((u64, u64), (u64, u64), (u64, u64))>>();

    // task 1
    let sum1 = machines
        .iter()
        .map(|&((ax, ay), (bx, by), (px, py))| {
            // generate all combinations
            (0..=min(px / ax, 100))
                .flat_map(|fa| {
                    (0..=min(px / bx, 100))
                        .map(move |fb| (ax * fa + bx * fb, ay * fa + by * fb, fa, fb))
                })
                .filter(|&(sum_x, sum_y, _, _)| sum_x == px && sum_y == py) // find matching combinations
                .map(|(_, _, fa, fb)| fa * 3 + fb) //compute cost
                .min() //lowest cost
                .or(Some(0)) //if no combinations win the prize we do not use coins
                .unwrap()
        })
        .sum::<u64>();

    println!("Task 1: sum {:?}", sum1);

    // task 2
    let sum2 = machines
        .iter()
        .map(|&((ax, ay), (bx, by), (px, py))| {
            (
                (ax, ay),
                (bx, by),
                (10000000000000u64 + px, 10000000000000u64 + py),
            )
        })
        .map(|((ax, ay), (bx, by), (px, py))| {
            let cm_x = ax * bx;
            let cm_y = ay * by;
            let residual_px = px % cm_x;
            let residual_py = py % cm_y;
            // generate all combinations
            (0..= (residual_px / ax))
                .flat_map(|fa| {
                    (0..=(residual_px / bx)).map(move |fb| (ax * fa + bx * fb, ay * fa + by * fb, fa, fb))
                })
                .filter(|&(sum_x, sum_y, _, _)| sum_x == residual_px && sum_y == residual_py) // find matching combinations
                .map(|(_, _, fa, fb)| fa * 3 + fb) //compute cost
                .min() //lowest cost
                .or(Some(0)) //if no combinations win the prize we do not use coins
                .unwrap()
        })
        .sum::<u64>();

    println!("Task 2: sum {:?}", sum2);
}
