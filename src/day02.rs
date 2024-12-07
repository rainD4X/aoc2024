const INPUT: &str = include_str!("../inputs/day02");

pub fn part_one() {
    let mut levels: Vec<i32> = vec![];
    let safe_reports = INPUT
        .split_terminator('\n')
        .map(|line| {
            levels = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            let first = levels[1] - levels[0];
            levels.windows(2).all(|w| {
                let diff = w[1] - w[0];
                (first > 0 && diff >= 1 && diff <= 3) || (first < 0 && diff >= -3 && diff <= -1)
            })
        })
        .filter(|b| *b)
        .count();
    println!("Safe reports: {}", safe_reports);
}

pub fn part_two() {
    let mut levels: Vec<i32> = vec![];
    let safe_reports = INPUT
        .split_terminator('\n')
        .map(|line| {
            levels = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            let trend = levels
                .windows(2)
                .map(|w| if w[1] - w[0] > 0 { 1 } else { -1 })
                .sum::<i32>();

            let is_valid = |diff| {
                (trend > 1 && diff >= 1 && diff <= 3) || (trend < -1 && diff >= -3 && diff <= -1)
            };

            let mut errors = 0;
            let mut i = 1;
            while i < levels.len() && errors <= 1 {
                if !is_valid(levels[i] - levels[i - 1]) {
                    errors += 1;

                    if i == levels.len() - 1 || is_valid(levels[i + 1] - levels[i - 1]) {
                        i += 1; // removes levels[i]
                    } else if i > 1 && !is_valid(levels[i] - levels[i - 2]) {
                        errors += 1; // could not remove levels[i - 1] without errors
                    }
                }
                i += 1;
            }
            errors < 2
        })
        .filter(|b| *b)
        .count();
    println!("Safe reports: {}", safe_reports);
}
