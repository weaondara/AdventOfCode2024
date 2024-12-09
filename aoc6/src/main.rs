use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;

fn within_bounds(matrix: &Vec<Vec<char>>, pos_xy: &(i32, i32)) -> bool {
    pos_xy.0 >= 0
        && pos_xy.1 >= 0
        && pos_xy.0 < matrix.len() as i32
        && pos_xy.1 < matrix[0].len() as i32
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let matrix = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut init_pos_xy = (0, 0);
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == '^' {
                init_pos_xy = (x as i32, y as i32);
                break;
            }
        }
    }

    let pos_transforms: HashMap<char, (i32, i32)> =
        HashMap::from([('N', (0, -1)), ('E', (1, 0)), ('S', (0, 1)), ('W', (-1, 0))]);
    let dir_transforms = HashMap::from([('N', 'E'), ('E', 'S'), ('S', 'W'), ('W', 'N')]);

    let mut pos_xy = init_pos_xy;
    let mut dir = 'N';

    //task 1
    let mut visited = HashSet::from([pos_xy]);
    loop {
        let transform = pos_transforms.get(&dir).unwrap();
        let next_pos_xy = (pos_xy.0 + transform.0, pos_xy.1 + transform.1);

        if !within_bounds(&matrix, &next_pos_xy) {
            break;
        }

        if matrix[next_pos_xy.1 as usize][next_pos_xy.0 as usize] == '#' {
            dir = *dir_transforms.get(&dir).unwrap();
            continue;
        }

        visited.insert(next_pos_xy);

        pos_xy = next_pos_xy;
    }

    println!("Task 1: sum {:?}", visited.len());

    // task 2
    let mut new_obstacles = HashSet::new();
    for o in visited.iter() {
        if *o == init_pos_xy {
            continue;
        }

        pos_xy = init_pos_xy;
        dir = 'N';
        let mut visited2 = HashSet::from([(pos_xy.0, pos_xy.1, dir)]);
        loop {
            let transform = pos_transforms.get(&dir).unwrap();
            let next_pos_xy = (pos_xy.0 + transform.0, pos_xy.1 + transform.1);

            if !within_bounds(&matrix, &next_pos_xy) {
                break;
            }

            if matrix[next_pos_xy.1 as usize][next_pos_xy.0 as usize] == '#'
                || next_pos_xy == *o
            {
                dir = *dir_transforms.get(&dir).unwrap();
                continue;
            }

            // let turn_right_dir = *dir_transforms.get(&dir).unwrap();
            if visited2.contains(&(next_pos_xy.0, next_pos_xy.1, dir)) {
                new_obstacles.insert(*o);
                break;
            }

            visited2.insert((next_pos_xy.0, next_pos_xy.1, dir));

            pos_xy = next_pos_xy;
        }
    }
    println!("Task 2: sum {:?}", new_obstacles.len());
}
