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
            let first = levels[1] - levels[0];
            levels.windows(2).all(|w| {
                let diff = w[1] - w[0];
                (first > 0 && diff >= 1 && diff <= 3) || (first < 0 && diff >= -3 && diff <= -1)
            })
        })
        .filter(|b| *b)
        .count();

    println!("{}", res);
}
