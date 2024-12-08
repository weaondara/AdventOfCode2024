use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    // task 1
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let sum1 = re
        .captures_iter(contents.as_str())
        .map(|c| c.extract())
        .map(|(_, m)| m)
        .map(|[n1, n2]| n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("Task 1: sum {:?}", sum1);

    //task 2
    let re2 = Regex::new(r"(?<do>do(?<not>n't)?\(\))|(?<mul>mul\((?<n1>[0-9]+),(?<n2>[0-9]+)\))")
        .unwrap();
    let mut findings = re2
        .captures_iter(contents.as_str())
        .map(|c| {
            [
                c.name("do").map_or("", |m| m.as_str()),
                c.name("not").map_or("", |m| m.as_str()),
                c.name("n1").map_or("", |m| m.as_str()),
                c.name("n2").map_or("", |m| m.as_str()),
            ]
        })
        .collect::<Vec<[&str; 4]>>();

    let mut i = 0;
    while i < findings.len() {
        if findings[i][0] == "don't()" {
            findings.remove(i);
            while i < findings.len() && findings[i][0] != "do()" {
                findings.remove(i);
            }
        }
        i += 1;
    }

    let sum2 = findings
        .iter()
        .filter(|m| m[0].is_empty())
        .map(|[_, _, n1, n2]| n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("Task 2: sum {:?}", sum2);
}
