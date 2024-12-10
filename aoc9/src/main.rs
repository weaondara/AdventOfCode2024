use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    let expanded = contents
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            (0..(c as u32 - '0' as u32)).map(move |_| if i & 1 == 1 { -1 } else { i as i64 / 2 })
        })
        .collect::<Vec<i64>>();

    // task 1
    let mut expanded1 = expanded.clone();
    let mut i = 0;
    for r in (0..expanded1.len()).rev() {
        if expanded1[r] == -1 {
            continue;
        }

        //advance to next free pos
        loop {
            if expanded1[i] != -1 {
                i += 1;
            } else {
                break;
            }
        }

        //done?
        if i >= r {
            break;
        }

        //defrag
        expanded1.swap(i, r);
    }

    let checksum1 = expanded1
        .iter()
        .filter(|&&e| e != -1)
        .enumerate()
        .map(|(i, &x)| i as i64 * x)
        .sum::<i64>();

    println!("Task 1: sum {:?}", checksum1);

    // task 2
    let mut expanded2 = expanded.clone();

    let mut r2 = expanded2.len() as i64 - 1;
    'outer: loop {
        if r2 < 0 {
            break 'outer;
        }

        if expanded2[r2 as usize] == -1 {
            r2 -= 1;
            continue;
        }

        let mut r_len = 1;
        while r2 >= r_len && expanded2[(r2 - r_len) as usize] == expanded2[r2 as usize] {
            r_len += 1;
        }

        //advance to next free pos
        let mut i2: i64 = 0;
        loop {
            if expanded2[i2 as usize] != -1 {
                i2 += 1;
                continue;
            }

            //check if space is big enough
            let mut i_len: i64 = 1;
            while i2 + i_len < r2 && expanded2[(i2 + i_len) as usize] == expanded2[i2 as usize] {
                i_len += 1;
            }
            if i_len >= r_len {
                break;
            }

            //space not big enough
            i2 += i_len;

            //goto next file
            if (i2 + 1) as usize >= expanded2.len() {
                r2 -= r_len;
                continue 'outer;
            }
        }

        //done?
        if i2 >= r2 {
            break;
        }

        //defrag
        for x in 0..r_len {
            expanded2.swap((i2 + x) as usize, (r2 - x) as usize);
        }
    }

    let checksum2 = expanded2
        .iter()
        .enumerate()
        .map(|(i, &x)| if x == -1 { 0 } else { i as i64 * x })
        .sum::<i64>();

    println!("Task 2: sum {:?}", checksum2);
}
