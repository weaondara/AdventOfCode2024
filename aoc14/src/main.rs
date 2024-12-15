use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let robots = contents
        .trim()
        .lines()
        .map(|line| {
            let split = line.split(&[',', ' ', '='][..]).collect::<Vec<&str>>();
            (
                (
                    split[1].parse::<i32>().unwrap(),
                    split[2].parse::<i32>().unwrap(),
                ),
                (
                    split[4].parse::<i32>().unwrap(),
                    split[5].parse::<i32>().unwrap(),
                ),
            )
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();
    let size = (101, 103);

    //task 1
    let iterations1 = 100;
    let mut robots1 = robots.clone();
    for ((px, py), (vx, vy)) in robots1.iter_mut() {
        *px = (*px + (*vx + size.0) * iterations1 + size.0) % size.0;
        *py = (*py + (*vy + size.1) * iterations1 + size.1) % size.1;
    }

    let factor1 = [
        ((0, size.0 / 2), (0, size.1 / 2)),
        ((size.0 - size.0 / 2, size.0), (0, size.1 / 2)),
        ((0, size.0 / 2), (size.1 - size.1 / 2, size.1)),
        ((size.0 - size.0 / 2, size.0), (size.1 - size.1 / 2, size.1)),
    ]
    .iter()
    .map(|&((xs, xe), (ys, ye))| {
        robots1
            .iter()
            .filter(|&&((px, py), _)| xs <= px && px < xe && ys <= py && py < ye)
            .count()
    })
    .reduce(|a, b| a * b)
    .unwrap();

    println!("Task 1: factor {:?}", factor1);

    //task 2
    let mut robots2 = robots.clone();
    for i in 0..10000000 {
        for ((px, py), (vx, vy)) in robots2.iter_mut() {
            *px = (*px + (*vx + size.0) + size.0) % size.0;
            *py = (*py + (*vy + size.1) + size.1) % size.1;
        }

        let mut matrix = (0..size.1)
            .map(|_| (0..size.0).map(|_| 0).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();
        for &((px, py), _) in robots2.iter() {
            matrix[py as usize][px as usize] += 1;
        }
        let fully_surrounded_robots = (1..size.1 - 1)
            .flat_map(|y| (1..size.0 - 1).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                matrix[y as usize][x as usize] > 0
                    && matrix[y as usize - 1][x as usize] > 0
                    && matrix[y as usize + 1][x as usize] > 0
                    && matrix[y as usize][x as usize - 1] > 0
                    && matrix[y as usize][x as usize + 1] > 0
            })
            .count();

        if fully_surrounded_robots < 20 {
            continue;
        }

        let str = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| if c == 0 { ' ' } else { '*' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", str);
        println!("Task 2: seconds {:?}", i + 1);
        break;
    }
}
