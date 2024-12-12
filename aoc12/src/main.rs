use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn within_bounds(matrix: &Vec<Vec<char>>, pos_xy: (i32, i32)) -> bool {
    pos_xy.0 >= 0
        && pos_xy.1 >= 0
        && pos_xy.1 < matrix.len() as i32
        && pos_xy.0 < matrix[0].len() as i32
}

fn neighbours(matrix: &Vec<Vec<char>>, pos_xy: (usize, usize)) -> Vec<(usize, usize)> {
    neighbours_with_direction(matrix, pos_xy)
        .iter()
        .map(|&(x, y, _)| (x, y))
        .collect::<Vec<(usize, usize)>>()
}

fn neighbours_with_direction(
    matrix: &Vec<Vec<char>>,
    pos_xy: (usize, usize),
) -> Vec<(usize, usize, char)> {
    [
        (pos_xy.0 as i32 - 1, pos_xy.1 as i32, 'E'),
        (pos_xy.0 as i32 + 1, pos_xy.1 as i32, 'W'),
        (pos_xy.0 as i32, pos_xy.1 as i32 - 1, 'N'),
        (pos_xy.0 as i32, pos_xy.1 as i32 + 1, 'S'),
    ]
    .iter()
    .filter(|&&(x, y, _)| within_bounds(&matrix, (x, y)))
    .map(|&(x, y, d)| (x as usize, y as usize, d))
    .collect::<Vec<(usize, usize, char)>>()
}

fn discover_patch(matrix: &Vec<Vec<char>>, pos_xy: (usize, usize)) -> (Vec<(usize, usize)>, usize) {
    let patch_char = matrix[pos_xy.1][pos_xy.0];
    let mut patch_coords = Vec::<(usize, usize)>::from([pos_xy]);
    let mut patch_circumference: i32 = 4;

    let mut i = 0;
    while i < patch_coords.len() {
        let neighboars = neighbours(matrix, patch_coords[i])
            .iter()
            .filter(|&coord| !patch_coords.contains(coord))
            .filter(|&&(x2, y2)| matrix[y2][x2] == patch_char)
            .map(|&coord| coord)
            .collect::<Vec<(usize, usize)>>();

        neighboars.iter().for_each(|&coord| {
            patch_coords.push(coord);

            let already_added_to_patch = neighbours(matrix, coord)
                .iter()
                .filter(|coord| patch_coords.contains(coord))
                .count();
            patch_circumference += 4 - already_added_to_patch as i32 * 2;
        });
        i += 1;
    }

    (patch_coords, patch_circumference as usize)
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let matrix = contents
        .trim()
        .lines()
        .map(|n| n.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    //task 1
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut patches = Vec::<Vec<(usize, usize)>>::new();
    let mut sum1 = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if visited.contains(&(x, y)) {
                continue;
            }

            //discover patch
            let (patch_coords, patch_circumference) = discover_patch(&matrix, (x, y));

            visited.extend(&patch_coords);
            patches.push(patch_coords.clone());
            sum1 += patch_coords.len() * patch_circumference;
        }
    }

    println!("Task 1: sum {:?}", sum1);

    //task 2
    let directions = Vec::from(['N', 'E', 'S', 'W']);
    let sum2 = patches
        .iter()
        .map(|patch_coords| {
            let coords_with_border_directions = patch_coords
                .iter()
                .flat_map(|&coord| {
                    let occupied_directions = neighbours_with_direction(&matrix, coord)
                        .iter()
                        .filter(|&&(x, y, _)| patch_coords.contains(&(x, y)))
                        .map(|&(_, _, nd)| nd)
                        .collect::<HashSet<char>>();
                    directions
                        .iter()
                        .filter(|&dir| !occupied_directions.contains(&dir))
                        .map(|d| (coord.0, coord.1, *d))
                        .collect::<Vec<(usize, usize, char)>>()
                })
                .collect::<Vec<(usize, usize, char)>>();

            let sides = coords_with_border_directions
                .iter()
                //group by char
                .sorted_by_key(|&&(_, _, d)| d)
                .chunk_by(|(_, _, d)| d)
                .into_iter()
                .map(|(_, d_chunk)| {
                    let section_sum = d_chunk
                        //group by lanes
                        .sorted_by_key(|&&(x, y, d)| if d == 'N' || d == 'S' { y } else { x }) // sort by different lanes, for N or S border use rows
                        .chunk_by(|&&(x, y, d)| if d == 'N' || d == 'S' { y } else { x })
                        .into_iter()
                        //count sections
                        .map(|(_, lane)| {
                            let sorted_lane = lane
                                .sorted_by_key(
                                    |&&(x, y, d)| if d == 'N' || d == 'S' { x } else { y }, // sort within lanes, other direction as above
                                )
                                .map(|&e| e)
                                .collect::<Vec<(usize, usize, char)>>();

                            let mut sections = 1;
                            for i in 1..sorted_lane.len() {
                                let (px, py, _) = sorted_lane[i - 1];
                                let (x, y, d) = sorted_lane[i];
                                if d == 'N' || d == 'S' {
                                    if px + 1 != x {
                                        sections += 1;
                                    }
                                } else {
                                    if py + 1 != y {
                                        sections += 1;
                                    }
                                }
                            }
                            sections
                        })
                        .sum::<usize>();
                    return section_sum;
                })
                .sum::<usize>();
            return patch_coords.len() * sides;
        })
        .sum::<usize>();

    println!("Task 2: sum {:?}", sum2);
}
