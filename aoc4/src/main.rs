use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();

    //task 1
    let lines: Vec<&str> = contents.split("\n").filter(|l| !l.is_empty()).collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();
    let horizontal = lines
        .iter()
        .map(|e| e.matches("XMAS").count() + e.matches("SAMX").count())
        .sum::<usize>();
    let vertical = (0..cols)
        .map(|col| {
            // transform
            lines
                .iter()
                .map(|l| l.chars().nth(col).unwrap().to_string())
                .reduce(|a, b| format!("{a}{b}"))
                .unwrap()
        })
        .map(|e| e.matches("XMAS").count() + e.matches("SAMX").count())
        .sum::<usize>();
    let diagonal = (-(cols as i32)..(cols as i32))
        .map(|col| {
            // transform
            (0..(rows as i32))
                .map(|row| {
                    let pos = row + col;
                    if pos < 0 || pos >= (rows as i32) {
                        "".to_owned()
                    } else {
                        lines[row as usize]
                            .chars()
                            .nth(pos as usize)
                            .unwrap()
                            .to_string()
                    }
                })
                .reduce(|a, b| format!("{a}{b}"))
                .unwrap()
        })
        .map(|e| e.matches("XMAS").count() + e.matches("SAMX").count())
        .sum::<usize>();
    let diagonal2 = (0..(cols * 2))
        .map(|col| {
            // transform
            (0..(rows as i32))
                .map(|row| {
                    let pos = col as i32 - row;
                    if pos < 0 || pos >= (rows as i32) {
                        "".to_owned()
                    } else {
                        lines[row as usize]
                            .chars()
                            .nth(pos as usize)
                            .unwrap()
                            .to_string()
                    }
                })
                .reduce(|a, b| format!("{a}{b}"))
                .unwrap()
        })
        .map(|e| e.matches("XMAS").count() + e.matches("SAMX").count())
        .sum::<usize>();
    let sum1 = horizontal + vertical + diagonal + diagonal2;
    println!(
        "Task 1: {:?} {:?} {:?} {:?} sum {:?}",
        horizontal, vertical, diagonal, diagonal2, sum1
    );

    // task 2
    let sum2 = (1..(rows - 1))
        .map(|row| {
            (1..(cols - 1))
                .map(|col| {
                    if lines[row].chars().nth(col).unwrap() != 'A' {
                        return 0;
                    }

                    let d1 = format!(
                        "{}A{}",
                        lines[row - 1].chars().nth(col - 1).unwrap(),
                        lines[row + 1].chars().nth(col + 1).unwrap()
                    );
                    let d2 = format!(
                        "{}A{}",
                        lines[row - 1].chars().nth(col + 1).unwrap(),
                        lines[row + 1].chars().nth(col - 1).unwrap()
                    );

                    if (d1 == "MAS" || d1 == "SAM") && (d2 == "MAS" || d2 == "SAM") {
                        return 1;
                    } else {
                        return 0;
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Task 2: sum {:?}", sum2);
}
