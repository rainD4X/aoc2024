const INPUT: &[u8] = include_bytes!("../inputs/day04");

fn check_n(columns: usize, i: usize, j: usize) -> bool {
    j >= 3
        && INPUT[(j - 1) * columns + i] == b'M'
        && INPUT[(j - 2) * columns + i] == b'A'
        && INPUT[(j - 3) * columns + i] == b'S'
}

fn check_s(lines: usize, columns: usize, i: usize, j: usize) -> bool {
    j + 3 <= lines
        && INPUT[(j + 1) * columns + i] == b'M'
        && INPUT[(j + 2) * columns + i] == b'A'
        && INPUT[(j + 3) * columns + i] == b'S'
}

fn check_w(columns: usize, i: usize, j: usize) -> bool {
    i >= 3
        && INPUT[j * columns + i - 1] == b'M'
        && INPUT[j * columns + i - 2] == b'A'
        && INPUT[j * columns + i - 3] == b'S'
}

fn check_e(columns: usize, i: usize, j: usize) -> bool {
    i + 3 < columns - 1
        && INPUT[j * columns + i + 1] == b'M'
        && INPUT[j * columns + i + 2] == b'A'
        && INPUT[j * columns + i + 3] == b'S'
}

fn check_ne(columns: usize, i: usize, j: usize) -> bool {
    (j >= 3 && i + 3 <= columns)
        && INPUT[(j - 1) * columns + i + 1] == b'M'
        && INPUT[(j - 2) * columns + i + 2] == b'A'
        && INPUT[(j - 3) * columns + i + 3] == b'S'
}

fn check_nw(columns: usize, i: usize, j: usize) -> bool {
    (j >= 3 && i >= 3)
        && INPUT[(j - 1) * columns + i - 1] == b'M'
        && INPUT[(j - 2) * columns + i - 2] == b'A'
        && INPUT[(j - 3) * columns + i - 3] == b'S'
}

fn check_sw(lines: usize, columns: usize, i: usize, j: usize) -> bool {
    (j + 3 <= lines && i >= 3)
        && INPUT[(j + 1) * columns + i - 1] == b'M'
        && INPUT[(j + 2) * columns + i - 2] == b'A'
        && INPUT[(j + 3) * columns + i - 3] == b'S'
}

fn check_se(lines: usize, columns: usize, i: usize, j: usize) -> bool {
    (j + 3 <= lines && i + 3 < columns - 1)
        && INPUT[(j + 1) * columns + i + 1] == b'M'
        && INPUT[(j + 2) * columns + i + 2] == b'A'
        && INPUT[(j + 3) * columns + i + 3] == b'S'
}

pub fn part_one() {
    let (mut i, mut j) = (0, 0);
    let columns = INPUT.iter().position(|&c| c == b'\n').unwrap() + 1;
    let lines = INPUT.len() / columns - 1;
    let mut count = 0;

    while j <= lines {
        let c = INPUT[j * columns + i];
        if c == b'X' {
            count += check_ne(columns, i, j) as usize;
            count += check_n(columns, i, j) as usize;
            count += check_nw(columns, i, j) as usize;
            count += check_w(columns, i, j) as usize;
            count += check_sw(lines, columns, i, j) as usize;
            count += check_s(lines, columns, i, j) as usize;
            count += check_se(lines, columns, i, j) as usize;
            if check_e(columns, i, j) {
                count += 1;
                i += 4;
            } else {
                i += 1;
            }
        } else {
            i += 1
        }

        if i > columns - 2 {
            j += 1;
            i = 0;
        }
    }
    println!("count of XMAS {}", count);
}

fn check_xmas(columns: usize, i: usize, j: usize) -> bool {
    (INPUT[(j - 1) * columns + i - 1] == b'M' && INPUT[(j + 1) * columns + i + 1] == b'S'
        || INPUT[(j - 1) * columns + i - 1] == b'S' && INPUT[(j + 1) * columns + i + 1] == b'M')
        && (INPUT[(j + 1) * columns + i - 1] == b'M' && INPUT[(j - 1) * columns + i + 1] == b'S'
            || INPUT[(j + 1) * columns + i - 1] == b'S' && INPUT[(j - 1) * columns + i + 1] == b'M')
}

pub fn part_two() {
    let columns = INPUT.iter().position(|&c| c == b'\n').unwrap() + 1;
    let lines = INPUT.len() / columns - 1;
    let mut count = 0;

    let (mut i, mut j) = (1, 1);

    while j < lines {
        let c = INPUT[j * columns + i];
        if c == b'A' {
            count += check_xmas(columns, i, j) as usize;
        }
        i += 1;

        if i > columns - 2 {
            j += 1;
            i = 0;
        }
    }
    println!("count of XMAS {}", count);
}
