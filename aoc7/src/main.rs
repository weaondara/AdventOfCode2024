use std::fs;

fn try_combinations(result: u64, values: &Vec<u64>) -> bool {
    if values.len() > 2 {
        let remainder = Vec::from(&values[2..]);
        let mut add = Vec::from([values[0] + values[1]]);
        let mut mul = Vec::from([values[0] * values[1]]);
        add.extend(remainder.iter());
        mul.extend(remainder.iter());
        try_combinations(result, &add) || try_combinations(result, &mul)
    } else {
        values[0] + values[1] == result || values[0] * values[1] == result
    }
}

fn concat_numbers(n1: u64, n2: u64) -> u64 {
    format!("{}{}", n1, n2).parse::<u64>().unwrap()
}

fn try_combinations2(result: u64, values: &Vec<u64>) -> bool {
    if values.len() > 2 {
        let remainder = Vec::from(&values[2..]);
        let mut add = Vec::from([values[0] + values[1]]);
        let mut mul = Vec::from([values[0] * values[1]]);
        let mut con = Vec::from([concat_numbers(values[0], values[1])]);
        add.extend(remainder.iter());
        mul.extend(remainder.iter());
        con.extend(remainder.iter());
        try_combinations2(result, &add)
            || try_combinations2(result, &mul)
            || try_combinations2(result, &con)
    } else {
        values[0] + values[1] == result
            || values[0] * values[1] == result
            || concat_numbers(values[0], values[1]) == result
    }
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let equations = contents
        .trim()
        .lines()
        .map(|line| line.split(": ").collect::<Vec<&str>>())
        .map(|e| {
            (
                e[0].parse::<u64>().unwrap(),
                e[1].split(" ")
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    // task 1
    let sum1 = equations
        .iter()
        .filter(|e| try_combinations(e.0, &e.1))
        .map(|e| e.0)
        .sum::<u64>();

    println!("Task 1: sum {:?}", sum1);

    // task 2
    let sum2 = equations
        .iter()
        .filter(|e| try_combinations2(e.0, &e.1))
        .map(|e| e.0)
        .sum::<u64>();
    println!("Task 2: sum {:?}", sum2);
}
