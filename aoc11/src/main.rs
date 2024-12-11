use std::collections::HashMap;
use std::fs;

fn add_count(stone_map: &mut HashMap<u64, u64>, stone: u64, count: u64) {
    if stone_map.contains_key(&stone) {
        stone_map.insert(stone, stone_map.get(&stone).unwrap() + count);
    } else {
        stone_map.insert(stone, count);
    }
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let stones = contents
        .trim()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // task 1
    let mut stones1 = stones.clone();
    for _ in 0..25 {
        let mut i = 0;
        loop {
            if stones1[i] == 0 {
                stones1[i] = 1;
            } else if stones1[i].to_string().len() & 1 == 0 {
                let as_string = stones1[i].to_string();
                stones1[i] = as_string[..(as_string.len() / 2)].parse::<u64>().unwrap();
                stones1.insert(
                    i + 1,
                    as_string[(as_string.len() / 2)..].parse::<u64>().unwrap(),
                );
                i += 1;
            } else {
                stones1[i] *= 2024;
            }
            i += 1;
            if i >= stones1.len() {
                break;
            }
        }
    }

    println!("Task 1: sum {:?}", stones1.len());

    // task 2
    let mut stones2: HashMap<u64, u64> = HashMap::new();
    stones.iter().for_each(|&v| {
        add_count(&mut stones2, v, 1);
    });
    for _ in 0..75 {
        let mut stones2updated: HashMap<u64, u64> = HashMap::new();
        for (&stone, &count) in stones2.iter() {
            if stone == 0 {
                add_count(&mut stones2updated, 1, count);
            } else if stone.to_string().len() & 1 == 0 {
                let as_string = stone.to_string();
                let n1 = as_string[..(as_string.len() / 2)].parse::<u64>().unwrap();
                let n2 = as_string[(as_string.len() / 2)..].parse::<u64>().unwrap();
                add_count(&mut stones2updated, n1, count);
                add_count(&mut stones2updated, n2, count);
            } else {
                add_count(&mut stones2updated, stone * 2024, count);
            }
        }
        stones2 = stones2updated;
    }

    println!("Task 2: sum {:?}", stones2.values().sum::<u64>());
}
