use std::collections::HashSet;
use std::fs;

fn within_bounds(matrix: &Vec<Vec<u32>>, pos_xy: (i32, i32)) -> bool {
    pos_xy.0 >= 0
        && pos_xy.1 >= 0
        && pos_xy.0 < matrix.len() as i32
        && pos_xy.1 < matrix[0].len() as i32
}

fn find_trails(matrix: &Vec<Vec<u32>>, pos_xy: (usize, usize)) -> Vec<(usize, usize)> {
    if matrix[pos_xy.1][pos_xy.0] == 9 {
        Vec::from([pos_xy])
    } else {
        [
            (pos_xy.0 as i32, pos_xy.1 as i32 + 1),
            (pos_xy.0 as i32, pos_xy.1 as i32 - 1),
            (pos_xy.0 as i32 + 1, pos_xy.1 as i32),
            (pos_xy.0 as i32 - 1, pos_xy.1 as i32),
        ]
        .iter()
        .filter(|&pos| within_bounds(matrix, *pos))
        .map(|pos| (pos.0 as usize, pos.1 as usize))
        .filter(|pos| matrix[pos.1][pos.0] == matrix[pos_xy.1][pos_xy.0] + 1)
        .flat_map(|pos| find_trails(matrix, (pos.0, pos.1)))
        .collect::<Vec<(usize, usize)>>()
    }
}

fn find_trails_unique(matrix: &Vec<Vec<u32>>, pos_xy: (usize, usize)) -> HashSet<(usize, usize)> {
    HashSet::from_iter(find_trails(matrix, pos_xy))
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let matrix = contents
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as u32 - '0' as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let start_positions = (0..matrix.len())
        .flat_map(|y| {
            let matrix_y = &matrix[y];
            (0..matrix[y].len()).map(move |x| if matrix_y[x] == 0 { Some((x, y)) } else { None })
        })
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect::<Vec<(usize, usize)>>();

    // task 1
    let sum1 = start_positions
        .iter()
        .map(|p| find_trails_unique(&matrix.clone(), *p).len())
        .sum::<usize>();

    println!("Task 1: sum {:?}", sum1);

    // task 1
    let sum2 = start_positions
        .iter()
        .map(|p| find_trails(&matrix.clone(), *p).len())
        .sum::<usize>();

    println!("Task 2: sum {:?}", sum2);
}
