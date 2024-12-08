use std::fs;

fn correctly_sorted(update: &Vec<i32>, rule: &Vec<i32>) -> bool {
    let index1 = update.iter().position(|&e| e == rule[0]);
    let index2 = update.iter().position(|&e| e == rule[1]);
    index1.is_none() || index2.is_none() || index1 < index2
}

fn correctly_sorted_all(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    return rules.iter().all(|rule| correctly_sorted(update, rule));
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    //task 1
    let lines: Vec<&str> = contents.split("\n").collect();

    let rules = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split("|")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let updates = lines
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let correct = updates
        .iter()
        .filter(|u| correctly_sorted_all(u, &rules))
        .collect::<Vec<&Vec<i32>>>();
    let sum1 = correct.iter().map(|u| u[u.len() / 2]).sum::<i32>();

    println!("Task 1: sum {:?}", sum1);

    // task 2
    let wrong = updates
        .iter()
        .filter(|u| !correct.contains(u))
        .collect::<Vec<&Vec<i32>>>();
    let sum2 = wrong
        .iter()
        .map(|&u| u.clone())
        .map(|mut u: Vec<i32>| {
            loop {
                rules.iter().for_each(|r| {
                    let index1 = u.iter().position(|&e| e == r[0]);
                    let index2 = u.iter().position(|&e| e == r[1]);
                    if !index1.is_none() && !index2.is_none() && index1 > index2 {
                        u.swap(index1.unwrap(), index2.unwrap());
                    }
                });
                if correctly_sorted_all(&u, &rules) {
                    break;
                }
            }
            return u;
        })
        .map(|u| u[u.len() / 2])
        .sum::<i32>();

    println!("Task 2: sum {:?}", sum2);
}
