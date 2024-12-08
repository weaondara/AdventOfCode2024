use std::fs;

fn is_report_valid(report: &Vec<i32>) -> bool {
    let len = report.len() - 1;
    ((0..len).all(|i| report[i] < report[i + 1]) //asc
        || (0..len).all(|i| report[i] > report[i + 1])) // desc
        && (0..len).all(|i|{
            1 <= (report[i] - report[i + 1]).abs()
                && (report[i] - report[i + 1]).abs() <= 3
    })
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let reports = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // task 1
    let safe1 = reports.iter().filter(|&e| is_report_valid(e)).count();
    println!("Task 1: safe {:?}", safe1);

    // task 2
    let safe2 = reports
        .iter()
        .filter(|&report| {
            is_report_valid(report)
                || (0..report.len()).any(|ignore_index| {
                    let mut new_reports = report.clone();
                    new_reports.remove(ignore_index);
                    return is_report_valid(&new_reports);
                })
        })
        .count();
    println!("Task 2: safe {:?}", safe2);
}
