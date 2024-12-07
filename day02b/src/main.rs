use core::str;

fn main() {
    let input = str::from_utf8(include_bytes!("../../inputs/day02")).unwrap();

    let mut levels: Vec<i32> = vec![];
    let res = input
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

    println!("{}", res);
}
