use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

fn within_bounds(pos: (i32, i32), bounds: (usize, usize)) -> bool {
    pos.0 >= 0 && pos.0 < bounds.0 as i32 && pos.1 >= 0 && pos.1 < bounds.1 as i32
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let size = (
        contents.lines().next().unwrap().len(),
        contents.trim().lines().count(),
    );
    let nodes = contents
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                if c == '.' {
                    None
                } else {
                    Some((c, x as i32, y as i32))
                }
            })
        })
        .filter(|e| !e.is_none())
        .map(|e| e.unwrap())
        .sorted_by_key(|e| e.0)
        .chunk_by(|e| e.0)
        .into_iter()
        .map(|(key, chunk)| (key, chunk.collect::<Vec<(char, i32, i32)>>()))
        .collect::<HashMap<char, Vec<(char, i32, i32)>>>();

    // task 1
    let mut anti_nodes = HashSet::<(i32, i32)>::new();

    for (_, positions) in nodes.iter() {
        let pairs = positions.iter().combinations(2);
        for pair in pairs {
            let anti_node1 = (
                pair[0].1 - (pair[1].1 - pair[0].1),
                pair[0].2 - (pair[1].2 - pair[0].2),
            );
            if within_bounds(anti_node1, size) {
                anti_nodes.insert(anti_node1);
            }
            let anti_node2 = (
                pair[1].1 + (pair[1].1 - pair[0].1),
                pair[1].2 + (pair[1].2 - pair[0].2),
            );
            if within_bounds(anti_node2, size) {
                anti_nodes.insert(anti_node2);
            }
        }
    }

    println!("Task 1: sum {:?}", anti_nodes.len());

    // task 2
    let mut anti_nodes2 = HashSet::<(i32, i32)>::new();

    for (_, positions) in nodes.iter() {
        let pairs = positions.iter().combinations(2);
        for pair in pairs {
            for i in 0..=max(size.0, size.1) as i32 {
                let anti_node1 = (
                    pair[0].1 - (pair[1].1 - pair[0].1) * i,
                    pair[0].2 - (pair[1].2 - pair[0].2) * i,
                );
                if within_bounds(anti_node1, size) {
                    anti_nodes2.insert(anti_node1);
                } else {
                    break;
                }
            }
            for i in 0..=max(size.0, size.1) as i32 {
                let anti_node2 = (
                    pair[1].1 + (pair[1].1 - pair[0].1) * i,
                    pair[1].2 + (pair[1].2 - pair[0].2) * i,
                );
                if within_bounds(anti_node2, size) {
                    anti_nodes2.insert(anti_node2);
                } else {
                    break;
                }
            }
        }
    }

    println!("Task 2: sum {:?}", anti_nodes2.len());
}
