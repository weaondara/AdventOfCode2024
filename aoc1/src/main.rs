use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lists = contents
        .split('\n')
        .filter(|e| e.len() > 0)
        .map(|s| {
            s.split("   ")
                .map(|e| e.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut list1 = lists.iter().map(|e| e[0]).collect::<Vec<i32>>();
    let mut list2 = lists.iter().map(|e| e[1]).collect::<Vec<i32>>();

    // sort
    list1.sort();
    list2.sort();

    let dist1 = (0..list1.len())
        .map(|i| (list1[i] - list2[i]).abs())
        .reduce(|a, b| a + b)
        .unwrap();
    println!("Task 1: dist {:?}", dist1);

    // task 2
    let dist2 = (0..list1.len())
        .map(|i| (list1[i] * list2.iter().filter(|&&e| e == list1[i]).count() as i32).abs())
        .reduce(|a, b| a + b)
        .unwrap();
    println!("Task 2: dist {:?}", dist2);
}
